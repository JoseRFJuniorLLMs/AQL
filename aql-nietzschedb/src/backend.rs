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
use aql_core::types::{ArousalSpec, ValenceSpec};
use std::collections::HashMap;
use std::time::Instant;
use tonic::transport::Channel;

use crate::hyperbolic;
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

    /// Batch-fetch full nodes from FTS results with concurrent GetNode calls.
    /// Uses tokio::spawn for parallel gRPC hydration (bounded by result count).
    async fn fetch_fts_nodes(
        client: &mut NietzscheDbClient<Channel>,
        collection: &str,
        results: Vec<proto::FtsResultProto>,
    ) -> Vec<CognitiveNode> {
        use futures::future::join_all;

        let futs: Vec<_> = results
            .into_iter()
            .map(|r| {
                let mut c = client.clone();
                let col = collection.to_string();
                async move {
                    match c
                        .get_node(proto::NodeIdRequest {
                            id: r.node_id.clone(),
                            collection: col,
                        })
                        .await
                    {
                        Ok(node_resp) => {
                            let n = node_resp.into_inner();
                            if n.found {
                                Some(node_response_to_cognitive(&n, r.score as f32))
                            } else {
                                None
                            }
                        }
                        Err(e) => {
                            tracing::warn!(node_id = %r.node_id, error = %e, "GetNode failed during FTS hydration");
                            None
                        }
                    }
                }
            })
            .collect();

        join_all(futs).await.into_iter().flatten().collect()
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

                    let fts_nodes = Self::fetch_fts_nodes(&mut client, &collection, resp.into_inner().results).await;
                    all_nodes.extend(fts_nodes);
                }
                NaqInstruction::FullTextSearch { collection, query, limit } => {
                    let resp = client.full_text_search(proto::FullTextSearchRequest {
                        collection: collection.clone(),
                        query,
                        limit,
                    }).await.map_err(|e| AqlError::Backend(format!("FullTextSearch: {e}")))?;

                    let fts_nodes = Self::fetch_fts_nodes(&mut client, &collection, resp.into_inner().results).await;
                    all_nodes.extend(fts_nodes);
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
                        collection: collection.clone(),
                    }).await.map_err(|e| AqlError::Backend(format!("BFS: {e}")))?;

                    // Hydrate BFS results with full node data
                    for id in resp.into_inner().visited_ids {
                        match client.get_node(proto::NodeIdRequest {
                            id: id.clone(),
                            collection: collection.clone(),
                        }).await {
                            Ok(node_resp) => {
                                let n = node_resp.into_inner();
                                if n.found {
                                    all_nodes.push(node_response_to_cognitive(&n, 1.0));
                                }
                            }
                            Err(_) => {
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
                    }
                }
                NaqInstruction::Dijkstra { collection, start, end } => {
                    let resp = client.dijkstra(proto::TraversalRequest {
                        start_node_id: start,
                        max_depth: 10,
                        max_cost: 0.0,
                        energy_min: 0.0,
                        max_nodes: 1000,
                        collection: collection.clone(),
                    }).await.map_err(|e| AqlError::Backend(format!("Dijkstra: {e}")))?;

                    let inner = resp.into_inner();

                    // If a target is specified, truncate to only nodes up to
                    // and including the target (Dijkstra visits in cost order).
                    let visit_count = if !end.is_empty() {
                        match inner.visited_ids.iter().position(|id| id == &end) {
                            Some(pos) => pos + 1, // include the target itself
                            None => {
                                // Target unreachable — return empty result instead of all visited nodes
                                tracing::warn!(target = %end, visited = inner.visited_ids.len(),
                                    "TRACE target unreachable via Dijkstra");
                                0
                            }
                        }
                    } else {
                        inner.visited_ids.len()
                    };

                    // Hydrate with full node data instead of empty shells
                    for (i, id) in inner.visited_ids.iter().take(visit_count).enumerate() {
                        let cost = inner.costs.get(i).copied().unwrap_or(0.0) as f32;
                        match client.get_node(proto::NodeIdRequest {
                            id: id.clone(),
                            collection: collection.clone(),
                        }).await {
                            Ok(node_resp) => {
                                let n = node_resp.into_inner();
                                if n.found {
                                    let mut node = node_response_to_cognitive(&n, cost);
                                    node.confidence = cost; // use cost as confidence for path ordering
                                    all_nodes.push(node);
                                }
                            }
                            Err(_) => {
                                all_nodes.push(CognitiveNode {
                                    id: id.clone(),
                                    content: String::new(),
                                    node_type: String::new(),
                                    energy: 0.0,
                                    confidence: cost,
                                    valence: 0.0,
                                    arousal: 0.0,
                                    magnitude: None,
                                    created_at: None,
                                    updated_at: None,
                                    metadata: HashMap::new(),
                                });
                            }
                        }
                    }
                }
                NaqInstruction::TriggerDream { collection, topic } => {
                    if !topic.is_empty() {
                        tracing::warn!(topic = %topic, "DREAM topic provided but TriggerSleep gRPC has no topic parameter — running generic sleep");
                    }
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

    /// FTS search + GetNode hydration helper for geometric verbs.
    async fn fts_with_hydration(
        &self,
        collection: &str,
        query: &str,
        limit: u32,
    ) -> Result<Vec<CognitiveNode>, AqlError> {
        let mut client = self.client().await?;
        let resp = client.full_text_search(proto::FullTextSearchRequest {
            collection: collection.to_string(),
            query: query.to_string(),
            limit,
        }).await.map_err(|e| AqlError::Backend(format!("FullTextSearch: {e}")))?;

        Ok(Self::fetch_fts_nodes(&mut client, collection, resp.into_inner().results).await)
    }
}

/// Convert a NodeResponse (flat proto) to CognitiveNode.
/// Extracts valence/arousal from the content JSON (not proto fields — they don't exist there).
fn node_response_to_cognitive(n: &proto::NodeResponse, score: f32) -> CognitiveNode {
    let content_str = String::from_utf8_lossy(&n.content).to_string();

    // Extract valence/arousal from content JSON if present
    let (valence, arousal) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content_str) {
        let v = json.get("valence").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let a = json.get("arousal").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        (v, a)
    } else {
        (0.0, 0.0)
    };

    // Extract display content: try "description" then "content" key, fallback to raw string
    let display_content = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content_str) {
        json.get("description")
            .or_else(|| json.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or(&content_str)
            .to_string()
    } else {
        content_str
    };

    CognitiveNode {
        id: n.id.clone(),
        content: display_content,
        node_type: n.node_type.clone(),
        energy: n.energy,
        confidence: score,
        valence,
        arousal,
        magnitude: Some(n.depth), // depth field = magnitude in Poincaré
        created_at: if n.created_at > 0 { Some(n.created_at.to_string()) } else { None },
        updated_at: None,
        metadata: HashMap::new(),
    }
}

/// Apply post-filters from PlanBase qualifiers to a result set.
/// Filters by confidence floor, valence, arousal, and recency.
fn apply_qualifier_filters(result: &mut CognitiveResult, base: &PlanBase) {
    // Filter by confidence floor (use confidence if set, else fall back to energy)
    if let Some(floor) = base.confidence_floor {
        result.nodes.retain(|n| {
            let c = if n.confidence > 0.0 { n.confidence } else { n.energy };
            c >= floor
        });
    }

    // Filter by valence
    if let Some(ref v) = base.valence {
        result.nodes.retain(|n| match v {
            ValenceSpec::Positive => n.valence > 0.0,
            ValenceSpec::Negative => n.valence < 0.0,
            ValenceSpec::Neutral => n.valence.abs() < 0.1,
            ValenceSpec::Exact(target) => (n.valence - target).abs() < 0.2,
        });
    }

    // Filter by arousal
    if let Some(ref a) = base.arousal {
        result.nodes.retain(|n| match a {
            ArousalSpec::High => n.arousal >= 0.7,
            ArousalSpec::Medium => n.arousal >= 0.3 && n.arousal < 0.7,
            ArousalSpec::Low => n.arousal > 0.0 && n.arousal < 0.3,
            ArousalSpec::Calm => n.arousal < 0.2,
            ArousalSpec::Exact(target) => (n.arousal - target).abs() < 0.15,
        });
    }

    // Filter by recency (using created_at timestamp if available)
    if let Some(ref recency) = base.recency {
        if let Some(window_secs) = recency.to_time_window_secs() {
            let now_secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let cutoff = now_secs.saturating_sub(window_secs as u64);
            result.nodes.retain(|n| {
                n.created_at
                    .as_ref()
                    .and_then(|ts| ts.parse::<u64>().ok())
                    .map_or(true, |ts| ts >= cutoff)
            });
        }
    }

    // Enforce limit
    if let Some(limit) = base.limit {
        result.nodes.truncate(limit as usize);
    }

    // Update metadata counts
    result.metadata.count = result.nodes.len() as u32;
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
        let mut result = self.execute_instructions(instructions, "RECALL").await?;
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
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
        let decrement = plan.energy_decrement;
        let delete_floor = plan.delete_threshold;

        // Align with server semantics: FADE targets a single node.
        // If the query looks like a UUID, target that node directly.
        // Otherwise, use FTS to find the single best-matching node.
        let target_node = if plan.query.len() == 36 && plan.query.contains('-') {
            // Likely a UUID — fetch the node directly via NQL
            let query_instructions = vec![NaqInstruction::QueryNodes {
                collection: col.clone(),
                nql: format!(
                    "MATCH (n) WHERE n.id = \"{}\" RETURN n LIMIT 1",
                    plan.query
                ),
                limit: 1,
            }];
            let result = self.execute_instructions(query_instructions, "FADE_LOOKUP").await?;
            result.nodes.into_iter().next()
        } else if !plan.query.is_empty() {
            // Text query — find single best match via FTS
            let query_instructions = vec![NaqInstruction::FullTextSearch {
                collection: col.clone(),
                query: plan.query.clone(),
                limit: 1,
            }];
            let result = self.execute_instructions(query_instructions, "FADE_LOOKUP").await?;
            result.nodes.into_iter().next()
        } else {
            return Err(AqlError::Backend("FADE requires a node ID or query text".into()));
        };

        let node = match target_node {
            Some(n) => n,
            None => return Err(AqlError::Backend(format!(
                "FADE: no node found for query '{}'", plan.query
            ))),
        };

        // Apply fade: decrement energy or delete if below floor
        let new_energy = node.energy - decrement;
        let instructions = if new_energy <= delete_floor {
            vec![NaqInstruction::DeleteNode {
                collection: col.clone(),
                node_id: node.id.clone(),
            }]
        } else {
            vec![NaqInstruction::UpdateEnergy {
                collection: col.clone(),
                node_id: node.id.clone(),
                new_energy,
            }]
        };

        self.execute_instructions(instructions, "FADE").await
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

        // Step 1: Find multiple seed candidates via FTS (matches server multi-seed approach)
        let seed_instructions = vec![NaqInstruction::FullTextSearch {
            collection: col.clone(),
            query: plan.query.clone(),
            limit: 5,
        }];
        let seed_result = self.execute_instructions(seed_instructions, "RESONATE_SEED").await?;

        if seed_result.nodes.is_empty() {
            return Ok(seed_result);
        }

        // Step 2: Try BFS from each seed — pick the one with most neighbors
        let mut best_result = CognitiveResult::empty();
        for seed in &seed_result.nodes {
            let bfs_instructions = vec![NaqInstruction::Bfs {
                collection: col.clone(),
                start: seed.id.clone(),
                max_depth: depth,
            }];
            let candidate = self.execute_instructions(bfs_instructions, "RESONATE").await?;
            if candidate.nodes.len() > best_result.nodes.len() {
                best_result = candidate;
                if best_result.nodes.len() > 1 { break; } // found connected subgraph
            }
        }

        apply_qualifier_filters(&mut best_result, &plan.base);
        Ok(best_result)
    }

    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        let instructions = lower_trace(plan);
        let mut result = self.execute_instructions(instructions, "TRACE").await?;
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
    }

    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        // REFLECT → PageRank to find central knowledge
        let resp = self.client().await?.run_page_rank(proto::PageRankRequest {
            collection: col.clone(),
            damping_factor: 0.85,
            max_iterations: 20,
            convergence_threshold: 1e-7,
        }).await.map_err(|e| AqlError::Backend(format!("PageRank: {e}")))?;

        let limit = plan.base.limit.unwrap_or(10) as usize;
        let mut scores = resp.into_inner().scores;
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // Hydrate top nodes with full content via GetNode
        let top_scores: Vec<_> = scores.into_iter().take(limit).collect();
        let mut client = self.client().await?;
        let mut nodes = Vec::with_capacity(top_scores.len());

        for s in top_scores {
            match client.get_node(proto::NodeIdRequest {
                id: s.node_id.clone(),
                collection: col.clone(),
            }).await {
                Ok(node_resp) => {
                    let n = node_resp.into_inner();
                    if n.found {
                        let mut node = node_response_to_cognitive(&n, s.score as f32);
                        node.confidence = s.score as f32;
                        nodes.push(node);
                    } else {
                        nodes.push(CognitiveNode {
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
                        });
                    }
                }
                Err(e) => {
                    tracing::warn!(node_id = %s.node_id, error = %e, "GetNode failed during REFLECT hydration");
                    nodes.push(CognitiveNode {
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
                    });
                }
            }
        }

        let mut result = CognitiveResult::from_nodes(nodes);
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
    }

    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let limit = plan.base.limit.unwrap_or(10) as usize;

        // DISTILL → Louvain community detection (NOT PageRank — that's REFLECT)
        // Find communities, then pick the highest-energy representative from each
        let resp = self.client().await?.run_louvain(proto::LouvainRequest {
            collection: col.clone(),
            max_iterations: 20,
            resolution: 1.0,
        }).await.map_err(|e| AqlError::Backend(format!("Louvain: {e}")))?;

        let inner = resp.into_inner();
        let assignments = inner.assignments;

        // Group nodes by community_id
        let mut communities: HashMap<u64, Vec<proto::NodeCommunity>> = HashMap::new();
        for assignment in assignments {
            communities.entry(assignment.community_id).or_default().push(assignment);
        }

        // For each community, fetch the first node to get energy, pick highest-energy as representative
        let mut representatives = Vec::new();
        let mut client = self.client().await?;

        for (_community_id, members) in &communities {
            let mut best_node: Option<CognitiveNode> = None;
            let mut best_energy: f32 = f32::NEG_INFINITY;

            // Sample up to 5 members per community to find the best representative
            for member in members.iter().take(5) {
                if let Ok(node_resp) = client.get_node(proto::NodeIdRequest {
                    id: member.node_id.clone(),
                    collection: col.clone(),
                }).await {
                    let n = node_resp.into_inner();
                    if n.found && n.energy > best_energy {
                        best_energy = n.energy;
                        best_node = Some(node_response_to_cognitive(&n, 0.0));
                    }
                }
            }

            if let Some(mut node) = best_node {
                // Set confidence to community size ratio for ranking
                node.confidence = members.len() as f32;
                representatives.push(node);
            }
        }

        // Sort by community size (confidence field) descending, take top N
        representatives.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        representatives.truncate(limit);

        let mut result = CognitiveResult {
            nodes: representatives,
            edges: vec![],
            metadata: ResultMetadata {
                verb: "DISTILL".into(),
                count: communities.len() as u32,
                avg_energy: 0.0,
                execution_time_ms: 0,
                backend: "NietzscheDB".into(),
                ..Default::default()
            },
        };
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
    }

    // ── Geometric verbs (Poincaré-aware) ──────────────

    async fn descend(&self, plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let fetch_limit = (plan.depth as u32) * 10; // over-fetch for filtering

        // Step 1: FTS to find contextually relevant nodes
        let candidates = self.fts_with_hydration(&col, &plan.content, fetch_limit).await?;

        if candidates.is_empty() {
            return Ok(CognitiveResult::from_nodes(vec![]));
        }

        // Step 2: Find source magnitude (first FTS result = anchor)
        let source_mag = candidates[0].magnitude.unwrap_or(0.0);
        let max_depth_delta = plan.depth as f32 * 0.15; // ~0.15 magnitude per depth level

        // Step 3: Filter to descendants (higher magnitude = deeper in Poincaré ball)
        let descendants: Vec<CognitiveNode> = candidates.into_iter()
            .filter(|n| {
                let mag = n.magnitude.unwrap_or(0.0);
                hyperbolic::filter_descendants(source_mag, mag, max_depth_delta)
            })
            .take(plan.base.limit.unwrap_or(10) as usize)
            .collect();

        let mut result = CognitiveResult::from_nodes(descendants);
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
    }

    async fn ascend(&self, plan: &AscendPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);
        let fetch_limit = (plan.depth as u32) * 10;

        // Step 1: FTS to find contextually relevant nodes
        let candidates = self.fts_with_hydration(&col, &plan.content, fetch_limit).await?;

        if candidates.is_empty() {
            return Ok(CognitiveResult::from_nodes(vec![]));
        }

        // Step 2: Find source magnitude
        let source_mag = candidates[0].magnitude.unwrap_or(0.5);

        // Step 3: Filter to ancestors (lower magnitude = shallower in Poincaré ball)
        let ancestors: Vec<CognitiveNode> = candidates.into_iter()
            .filter(|n| {
                let mag = n.magnitude.unwrap_or(0.0);
                hyperbolic::filter_ancestors(source_mag, mag)
            })
            .take(plan.base.limit.unwrap_or(10) as usize)
            .collect();

        let mut result = CognitiveResult::from_nodes(ancestors);
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
    }

    async fn orbit(&self, plan: &OrbitPlan) -> Result<CognitiveResult, AqlError> {
        let col = self.collection(&plan.base.collection);

        // Step 1: FTS to find contextually relevant nodes
        let candidates = self.fts_with_hydration(&col, &plan.content, 50).await?;

        if candidates.is_empty() {
            return Ok(CognitiveResult::from_nodes(vec![]));
        }

        // Step 2: Find source magnitude
        let source_mag = candidates[0].magnitude.unwrap_or(0.5);
        let orbit_radius = plan.radius.max(0.05); // minimum radius

        // Step 3: Filter to nodes at similar depth (orbit = same magnitude band)
        let orbit_nodes: Vec<CognitiveNode> = candidates.into_iter()
            .skip(1) // skip the source itself
            .filter(|n| {
                let mag = n.magnitude.unwrap_or(0.0);
                hyperbolic::filter_orbit(source_mag, mag, orbit_radius)
            })
            .take(plan.base.limit.unwrap_or(10) as usize)
            .collect();

        let mut result = CognitiveResult::from_nodes(orbit_nodes);
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
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
        let mut result = self.execute_instructions(instructions, "IMAGINE").await?;
        apply_qualifier_filters(&mut result, &plan.base);
        Ok(result)
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
