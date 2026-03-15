//! NietzscheBackend — impl AqlBackend for NietzscheDB.
//! Connects via gRPC to NietzscheDB server.
//! Each verb method lowers the plan to NaqInstructions, then executes
//! them as individual gRPC calls (KnnSearch, InsertNode, BFS, etc.).

use async_trait::async_trait;
use aql_core::capabilities::BackendCapabilities;
use aql_core::error::AqlError;
use aql_core::plans::*;
use aql_core::result::*;
use aql_core::traits::AqlBackend;
use std::collections::HashMap;
use std::time::Instant;
use tonic::transport::Channel;

use crate::lowering::*;
use crate::proto;
use crate::proto::nietzsche_db_client::NietzscheDbClient;

/// NietzscheDB backend with full AQL support via individual gRPC calls.
pub struct NietzscheBackend {
    endpoint: String,
    default_collection: Option<String>,
    channel: tokio::sync::OnceCell<Channel>,
}

impl NietzscheBackend {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            default_collection: None,
            channel: tokio::sync::OnceCell::new(),
        }
    }

    pub fn with_collection(mut self, collection: impl Into<String>) -> Self {
        self.default_collection = Some(collection.into());
        self
    }

    fn collection(&self, plan_collection: &Option<String>) -> String {
        plan_collection
            .clone()
            .or_else(|| self.default_collection.clone())
            .unwrap_or_else(|| "default".to_string())
    }

    async fn client(&self) -> Result<NietzscheDbClient<Channel>, AqlError> {
        let channel = self.channel.get_or_try_init(|| async {
            Channel::from_shared(self.endpoint.clone())
                .map_err(|e| AqlError::Backend(format!("invalid endpoint: {e}")))?
                .connect()
                .await
                .map_err(|e| AqlError::Backend(format!("gRPC connect failed: {e}")))
        }).await?;
        Ok(NietzscheDbClient::new(channel.clone()))
    }

    /// Execute a list of NaqInstructions and collect results.
    async fn execute_instructions(
        &self,
        instructions: Vec<NaqInstruction>,
        verb: &str,
    ) -> Result<CognitiveResult, AqlError> {
        let start = Instant::now();
        let mut client = self.client().await?;
        let mut all_nodes = Vec::new();
        let mut all_edges = Vec::new();

        for instr in instructions {
            match instr {
                NaqInstruction::KnnSearch { collection, query_text, k } => {
                    // KNN needs embeddings — fall back to FTS for text queries
                    let resp = client.full_text_search(proto::FullTextSearchRequest {
                        collection: collection.clone(),
                        query: query_text,
                        limit: k,
                    }).await.map_err(|e| AqlError::Backend(format!("FullTextSearch: {e}")))?;

                    for r in resp.into_inner().results {
                        if let Ok(node_resp) = client.get_node(proto::NodeIdRequest {
                            id: r.node_id.clone(),
                            collection: collection.clone(),
                        }).await {
                            let n = node_resp.into_inner();
                            if n.found {
                                all_nodes.push(node_response_to_cognitive(&n, r.score as f32));
                            }
                        }
                    }
                }
                NaqInstruction::FullTextSearch { collection, query, limit } => {
                    let resp = client.full_text_search(proto::FullTextSearchRequest {
                        collection: collection.clone(),
                        query,
                        limit,
                    }).await.map_err(|e| AqlError::Backend(format!("FullTextSearch: {e}")))?;

                    for r in resp.into_inner().results {
                        if let Ok(node_resp) = client.get_node(proto::NodeIdRequest {
                            id: r.node_id.clone(),
                            collection: collection.clone(),
                        }).await {
                            let n = node_resp.into_inner();
                            if n.found {
                                all_nodes.push(node_response_to_cognitive(&n, r.score as f32));
                            }
                        }
                    }
                }
                NaqInstruction::InsertNode { collection, content, node_type, energy } => {
                    let resp = client.insert_node(proto::InsertNodeRequest {
                        id: String::new(),
                        embedding: None,
                        content: content.into_bytes(),
                        node_type,
                        energy,
                        collection,
                        expires_at: 0,
                    }).await.map_err(|e| AqlError::Backend(format!("InsertNode: {e}")))?;

                    let n = resp.into_inner();
                    all_nodes.push(node_response_to_cognitive(&n, 0.0));
                }
                NaqInstruction::InsertEdge { collection, source, target, edge_type, weight } => {
                    client.insert_edge(proto::InsertEdgeRequest {
                        id: String::new(),
                        from: source.clone(),
                        to: target.clone(),
                        edge_type: edge_type.clone(),
                        weight: weight as f64,
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("InsertEdge: {e}")))?;

                    all_edges.push(CognitiveEdge {
                        source,
                        target,
                        edge_type,
                        weight,
                    });
                }
                NaqInstruction::UpdateEnergy { collection, node_id, new_energy } => {
                    client.update_energy(proto::UpdateEnergyRequest {
                        node_id,
                        energy: new_energy,
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("UpdateEnergy: {e}")))?;
                }
                NaqInstruction::DeleteNode { collection, node_id } => {
                    client.delete_node(proto::NodeIdRequest {
                        id: node_id,
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("DeleteNode: {e}")))?;
                }
                NaqInstruction::Bfs { collection, start, max_depth } => {
                    let resp = client.bfs(proto::TraversalRequest {
                        start_node_id: start,
                        max_depth,
                        max_cost: 0.0,
                        energy_min: 0.0,
                        max_nodes: 1000,
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("BFS: {e}")))?;

                    for id in resp.into_inner().visited_ids {
                        all_nodes.push(CognitiveNode {
                            id,
                            content: String::new(),
                            node_type: String::new(),
                            energy: 0.0,
                            confidence: 1.0,
                            valence: 0.0,
                            arousal: 0.0,
                            magnitude: None,
                            created_at: None,
                            updated_at: None,
                            metadata: HashMap::new(),
                        });
                    }
                }
                NaqInstruction::Dijkstra { collection, start, end: _ } => {
                    let resp = client.dijkstra(proto::TraversalRequest {
                        start_node_id: start,
                        max_depth: 10,
                        max_cost: 0.0,
                        energy_min: 0.0,
                        max_nodes: 1000,
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("Dijkstra: {e}")))?;

                    let inner = resp.into_inner();
                    for (i, id) in inner.visited_ids.iter().enumerate() {
                        all_nodes.push(CognitiveNode {
                            id: id.clone(),
                            content: String::new(),
                            node_type: String::new(),
                            energy: 0.0,
                            confidence: inner.costs.get(i).copied().unwrap_or(0.0) as f32,
                            valence: 0.0,
                            arousal: 0.0,
                            magnitude: None,
                            created_at: None,
                            updated_at: None,
                            metadata: HashMap::new(),
                        });
                    }
                }
                NaqInstruction::TriggerDream { collection, topic: _ } => {
                    let resp = client.trigger_sleep(proto::SleepRequest {
                        collection,
                        noise: 0.02,
                        adam_steps: 10,
                        adam_lr: 5e-3,
                        hausdorff_threshold: 0.15,
                        rng_seed: 0,
                    }).await.map_err(|e| AqlError::Backend(format!("TriggerSleep: {e}")))?;

                    let inner = resp.into_inner();
                    tracing::info!(
                        hausdorff_before = inner.hausdorff_before,
                        hausdorff_after = inner.hausdorff_after,
                        nodes_perturbed = inner.nodes_perturbed,
                        "DREAM cycle completed"
                    );
                }
                NaqInstruction::TriggerSleep { collection } => {
                    client.trigger_sleep(proto::SleepRequest {
                        collection,
                        noise: 0.02,
                        adam_steps: 10,
                        adam_lr: 5e-3,
                        hausdorff_threshold: 0.15,
                        rng_seed: 0,
                    }).await.map_err(|e| AqlError::Backend(format!("TriggerSleep: {e}")))?;
                }
                NaqInstruction::QueryNodes { collection, nql, limit: _ } => {
                    let resp = client.query(proto::QueryRequest {
                        nql,
                        params: HashMap::new(),
                        collection,
                    }).await.map_err(|e| AqlError::Backend(format!("Query: {e}")))?;

                    for n in resp.into_inner().nodes {
                        all_nodes.push(node_response_to_cognitive(&n, 0.0));
                    }
                }
            }
        }

        let elapsed = start.elapsed().as_millis() as u64;
        let count = all_nodes.len() as u32;
        let avg_energy = if all_nodes.is_empty() {
            0.0
        } else {
            all_nodes.iter().map(|n| n.energy).sum::<f32>() / all_nodes.len() as f32
        };

        Ok(CognitiveResult {
            nodes: all_nodes,
            edges: all_edges,
            metadata: ResultMetadata {
                verb: verb.into(),
                count,
                avg_energy,
                execution_time_ms: elapsed,
                backend: "NietzscheDB".into(),
                ..Default::default()
            },
        })
    }
}

/// Convert a NodeResponse (flat proto) to CognitiveNode.
fn node_response_to_cognitive(n: &proto::NodeResponse, score: f32) -> CognitiveNode {
    let content = String::from_utf8_lossy(&n.content).to_string();
    CognitiveNode {
        id: n.id.clone(),
        content,
        node_type: n.node_type.clone(),
        energy: n.energy,
        confidence: score,
        valence: 0.0,
        arousal: 0.0,
        magnitude: Some(n.depth), // depth field = magnitude in Poincaré
        created_at: if n.created_at > 0 { Some(n.created_at.to_string()) } else { None },
        updated_at: None,
        metadata: HashMap::new(),
    }
}

#[async_trait]
impl AqlBackend for NietzscheBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::nietzschedb()
    }

    fn name(&self) -> &str {
        "NietzscheDB"
    }

    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError> {
        let instructions = lower_recall(plan);
        self.execute_instructions(instructions, "RECALL").await
    }

    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let node_type = plan
            .epistemic_type
            .map(|t| t.to_nietzsche_node_type().to_string())
            .unwrap_or("Semantic".into());
        let energy = plan.initial_energy.unwrap_or(0.6);

        // Step 1: Insert the node
        let mut client = self.client().await?;
        let resp = client.insert_node(proto::InsertNodeRequest {
            id: String::new(),
            embedding: None,
            content: plan.content.clone().into_bytes(),
            node_type,
            energy,
            collection: col.clone(),
            expires_at: 0,
        }).await.map_err(|e| AqlError::Backend(format!("InsertNode: {e}")))?;

        let node_resp = resp.into_inner();
        let node_id = node_resp.id.clone();
        let node = node_response_to_cognitive(&node_resp, 0.0);

        // Step 2: Link edge using the ACTUAL returned node ID (not content string)
        if let Some(ref link_to) = plan.link_to {
            let edge_instructions = vec![NaqInstruction::InsertEdge {
                collection: col,
                source: node_id,
                target: link_to.clone(),
                edge_type: "ASSOCIATED".into(),
                weight: 1.0,
            }];
            let edge_result = self.execute_instructions(edge_instructions, "IMPRINT_LINK").await?;
            let mut combined = CognitiveResult::from_nodes(vec![node]);
            combined.edges = edge_result.edges;
            return Ok(combined);
        }

        Ok(CognitiveResult::from_nodes(vec![node]))
    }

    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let threshold = plan.energy_decrement + plan.delete_threshold;
        let decrement = plan.energy_decrement;
        let delete_floor = plan.delete_threshold;

        // Step 1: Find low-energy nodes
        let query_instructions = vec![NaqInstruction::QueryNodes {
            collection: col.clone(),
            nql: format!(
                "MATCH (n) WHERE n.energy < {} RETURN n LIMIT 50",
                threshold
            ),
            limit: 50,
        }];
        let query_result = self.execute_instructions(query_instructions, "FADE_QUERY").await?;

        // Step 2: Mutate each node — delete if below floor, else decrement energy
        let mut mutation_instructions = Vec::new();
        for node in &query_result.nodes {
            let new_energy = node.energy - decrement;
            if new_energy <= delete_floor {
                mutation_instructions.push(NaqInstruction::DeleteNode {
                    collection: col.clone(),
                    node_id: node.id.clone(),
                });
            } else {
                mutation_instructions.push(NaqInstruction::UpdateEnergy {
                    collection: col.clone(),
                    node_id: node.id.clone(),
                    new_energy,
                });
            }
        }

        if mutation_instructions.is_empty() {
            return Ok(query_result);
        }
        self.execute_instructions(mutation_instructions, "FADE").await
    }

    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::InsertEdge {
            collection: col,
            source: plan.source.clone(),
            target: plan.target.clone(),
            edge_type: "ASSOCIATED".into(),
            weight: 1.0,
        }];
        self.execute_instructions(instructions, "ASSOCIATE").await
    }

    async fn resonate(&self, plan: &ResonatePlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let depth = plan.depth.unwrap_or(3) as u32;

        // Step 1: Find seed node via FTS
        let seed_instructions = vec![NaqInstruction::FullTextSearch {
            collection: col.clone(),
            query: plan.query.clone(),
            limit: 1,
        }];
        let seed_result = self.execute_instructions(seed_instructions, "RESONATE_SEED").await?;

        if seed_result.nodes.is_empty() {
            return Ok(seed_result);
        }

        let seed_id = seed_result.nodes[0].id.clone();

        // Step 2: BFS from seed — activation spreading through graph
        let bfs_instructions = vec![NaqInstruction::Bfs {
            collection: col.clone(),
            start: seed_id,
            max_depth: depth,
        }];
        self.execute_instructions(bfs_instructions, "RESONATE").await
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        let instructions = lower_trace(plan);
        self.execute_instructions(instructions, "TRACE").await
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        // REFLECT → PageRank to find central knowledge
        let resp = self.client().await?.run_page_rank(proto::PageRankRequest {
            collection: col,
            damping_factor: 0.85,
            max_iterations: 20,
            convergence_threshold: 1e-7,
        }).await.map_err(|e| AqlError::Backend(format!("PageRank: {e}")))?;

        let limit = plan.base.limit.unwrap_or(10) as usize;
        let mut scores = resp.into_inner().scores;
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let nodes: Vec<CognitiveNode> = scores.into_iter().take(limit).map(|s| CognitiveNode {
            id: s.node_id,
            content: String::new(),
            node_type: String::new(),
            energy: s.score as f32,
            confidence: s.score as f32,
            valence: 0.0,
            arousal: 0.0,
            magnitude: None,
            created_at: None,
            updated_at: None,
            metadata: HashMap::new(),
        }).collect();

        Ok(CognitiveResult::from_nodes(nodes))
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let resp = self.client().await?.run_page_rank(proto::PageRankRequest {
            collection: col,
            damping_factor: 0.85,
            max_iterations: 20,
            convergence_threshold: 1e-7,
        }).await.map_err(|e| AqlError::Backend(format!("PageRank: {e}")))?;

        let limit = plan.base.limit.unwrap_or(10) as usize;
        let mut scores = resp.into_inner().scores;
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let nodes: Vec<CognitiveNode> = scores.into_iter().take(limit).map(|s| CognitiveNode {
            id: s.node_id,
            content: String::new(),
            node_type: String::new(),
            energy: s.score as f32,
            confidence: s.score as f32,
            valence: 0.0,
            arousal: 0.0,
            magnitude: None,
            created_at: None,
            updated_at: None,
            metadata: HashMap::new(),
        }).collect();

        Ok(CognitiveResult::from_nodes(nodes))
    }

    // ── Geometric verbs (native in NietzscheDB) ──────────────

    async fn descend(&self, plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::FullTextSearch {
            collection: col,
            query: plan.content.clone(),
            limit: (plan.depth as u32) * 5,
        }];
        self.execute_instructions(instructions, "DESCEND").await
    }

    async fn ascend(&self, plan: &AscendPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::FullTextSearch {
            collection: col,
            query: plan.content.clone(),
            limit: (plan.depth as u32) * 5,
        }];
        self.execute_instructions(instructions, "ASCEND").await
    }

    async fn orbit(&self, plan: &OrbitPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::FullTextSearch {
            collection: col,
            query: plan.content.clone(),
            limit: 20,
        }];
        self.execute_instructions(instructions, "ORBIT").await
    }

    // ── Altered states (NietzscheDB exclusive) ────────────────

    async fn dream(&self, plan: &DreamPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::TriggerDream {
            collection: col,
            topic: plan.topic.clone(),
        }];
        self.execute_instructions(instructions, "DREAM").await
    }

    async fn imagine(&self, plan: &ImaginePlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let instructions = vec![NaqInstruction::FullTextSearch {
            collection: col,
            query: plan.premise.clone(),
            limit: plan.base.limit.unwrap_or(10),
        }];
        self.execute_instructions(instructions, "IMAGINE").await
    }

    async fn watch(&self, _plan: &WatchPlan) -> Result<WatchHandle, AqlError> {
        Ok(WatchHandle {
            id: uuid::Uuid::new_v4().to_string(),
            active: true,
        })
    }

    async fn explain(&self, plan: &ExplainPlan) -> Result<Explanation, AqlError> {
        Ok(Explanation {
            supported: true,
            verb: plan.inner_verb.clone(),
            strategy: "NietzscheDB gRPC (lowered NaqInstructions)".into(),
            steps: vec![ExplanationStep {
                action: "lower_and_execute".into(),
                detail: format!("Query: {}", plan.inner_query),
            }],
            confidence_chain: vec![1.0],
        })
    }
}
