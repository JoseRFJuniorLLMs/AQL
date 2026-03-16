//! CognitivePlanner — converts AQL AST into ExecutionPlans.
//! Applies mood, energy hooks, and self-resolution.

use crate::ast::*;
use crate::error::{AqlError, AqlResult};
use crate::plans::*;
use crate::types::*;

/// Planner configuration, influenced by MOOD.
#[derive(Debug, Clone)]
pub struct PlannerConfig {
    pub default_limit: u32,
    pub default_knn_k: u32,
    pub default_diffuse_depth: u8,
    pub max_chain_depth: u8,
    pub confidence_floor: f32,
    pub novelty_bias: f32,
    pub recency_bias: Option<RecencyDegree>,
    pub scope_override: Option<ContextScope>,
    pub fade_suppressed: bool,
    pub imprint_restricted: bool,
    pub agent_id: String,
    pub active_collection: Option<String>,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            default_limit: 10,
            default_knn_k: 20,
            default_diffuse_depth: 3,
            max_chain_depth: 10,
            confidence_floor: 0.0,
            novelty_bias: 0.5,
            recency_bias: None,
            scope_override: None,
            fade_suppressed: false,
            imprint_restricted: false,
            agent_id: "default".into(),
            active_collection: None,
        }
    }
}

impl PlannerConfig {
    pub fn apply_mood(&mut self, mood: MoodState) {
        match mood {
            MoodState::Creative => {
                self.default_knn_k = (self.default_knn_k * 2).min(200);
                self.default_diffuse_depth = (self.default_diffuse_depth + 2).min(20);
                self.novelty_bias = 0.8;
            }
            MoodState::Analytical => {
                self.default_limit = 5;
                self.max_chain_depth = 3;
                self.confidence_floor = 0.7;
            }
            MoodState::Anxious => {
                self.default_limit = 3;
                self.max_chain_depth = 2;
                self.recency_bias = Some(RecencyDegree::Fresh);
            }
            MoodState::Focused => {
                self.default_limit = 5;
                self.scope_override = Some(ContextScope::Session);
            }
            MoodState::Exploratory => {
                self.default_knn_k = (self.default_knn_k * 3).min(200);
                self.max_chain_depth = 10;
                self.fade_suppressed = true;
            }
            MoodState::Conservative => {
                self.confidence_floor = 0.8;
                self.novelty_bias = 0.1;
                self.imprint_restricted = true;
            }
        }
    }
}

/// The cognitive planner.
pub struct CognitivePlanner {
    pub config: PlannerConfig,
}

impl CognitivePlanner {
    pub fn new(config: PlannerConfig) -> Self {
        Self { config }
    }

    /// Plan a full program.
    pub fn plan_program(&mut self, program: &Program) -> AqlResult<Vec<ExecutionPlan>> {
        program
            .statements
            .iter()
            .map(|s| self.plan_statement(s))
            .collect()
    }

    /// Plan a single statement.
    pub fn plan_statement(&mut self, stmt: &Statement) -> AqlResult<ExecutionPlan> {
        match stmt {
            Statement::Simple(s) => self.plan_simple(s),
            Statement::Chain(c) => {
                let plans: AqlResult<Vec<_>> = c.steps.iter().map(|s| self.plan_simple(s)).collect();
                Ok(ExecutionPlan::Chain(plans?))
            }
            Statement::Parallel(p) => {
                let branches: AqlResult<Vec<_>> =
                    p.branches.iter().map(|s| self.plan_simple(s)).collect();
                let join = p
                    .join_step
                    .as_ref()
                    .map(|s| self.plan_simple(s))
                    .transpose()?
                    .map(Box::new);
                Ok(ExecutionPlan::Parallel {
                    branches: branches?,
                    join,
                })
            }
            Statement::Atomic(a) => {
                let plans: AqlResult<Vec<_>> =
                    a.statements.iter().map(|s| self.plan_statement(s)).collect();
                Ok(ExecutionPlan::Atomic(plans?))
            }
            Statement::Watch(w) => {
                let query = subject_to_query(&w.subject);
                Ok(ExecutionPlan::Watch(WatchPlan {
                    subject_query: query,
                    trigger: w.trigger,
                }))
            }
            Statement::Explain(e) => {
                let query = subject_to_query(&e.inner.subject);
                Ok(ExecutionPlan::Explain(ExplainPlan {
                    inner_verb: e.inner.verb.to_string(),
                    inner_query: query,
                }))
            }
        }
    }

    fn plan_simple(&mut self, stmt: &SimpleStatement) -> AqlResult<ExecutionPlan> {
        // Save config so mood mutations don't leak across statements in a chain
        let saved_config = self.config.clone();

        // Apply mood if present (scoped to this statement only)
        for q in &stmt.qualifiers {
            if let Qualifier::Mood(mood) = q {
                self.config.apply_mood(*mood);
            }
        }

        let base = self.build_base(&stmt.subject, &stmt.qualifiers);
        let query = subject_to_query(&stmt.subject);

        let result = match stmt.verb {
            Verb::Recall => {
                let mut plan = RecallPlan::new(query);
                plan.base = base;
                plan.type_filter = extract_type_filter(&stmt.subject);
                plan.novelty = extract_novelty(&stmt.qualifiers);
                plan.magnitude_range = extract_magnitude(&stmt.qualifiers);
                plan.curvature = extract_curvature(&stmt.qualifiers);
                Ok(ExecutionPlan::Recall(plan))
            }
            Verb::Resonate => Ok(ExecutionPlan::Resonate(ResonatePlan {
                base,
                query,
                depth: extract_depth(&stmt.qualifiers),
                novelty: extract_novelty(&stmt.qualifiers),
            })),
            Verb::Reflect => {
                let target = match &stmt.subject {
                    Subject::SelfRef => ReflectTarget::SelfAgent,
                    Subject::TypeFilter(t) => ReflectTarget::TypeFilter(*t),
                    _ => ReflectTarget::Collection(query),
                };
                Ok(ExecutionPlan::Reflect(ReflectPlan { base, target }))
            }
            Verb::Trace => {
                let (from, to) = match &stmt.subject {
                    Subject::TraceRange { from, to } => (from.clone(), to.clone()),
                    _ => (query.clone(), String::new()),
                };
                Ok(ExecutionPlan::Trace(TracePlan {
                    base,
                    from,
                    to,
                    depth: extract_depth(&stmt.qualifiers),
                }))
            }
            Verb::Imprint => {
                let etype = extract_as_type(&stmt.qualifiers)
                    .or_else(|| extract_type_filter(&stmt.subject));
                let link = extract_linking(&stmt.qualifiers);
                let energy = extract_confidence(&stmt.qualifiers)
                    .or_else(|| etype.map(|e| e.initial_energy()));
                Ok(ExecutionPlan::Imprint(ImprintPlan {
                    base,
                    content: query,
                    epistemic_type: etype,
                    link_to: link,
                    initial_energy: energy,
                }))
            }
            Verb::Associate => {
                let target = extract_linking(&stmt.qualifiers).unwrap_or_default();
                Ok(ExecutionPlan::Associate(AssociatePlan {
                    base,
                    source: query,
                    source_type: extract_type_filter(&stmt.subject),
                    target,
                }))
            }
            Verb::Distill => Ok(ExecutionPlan::Distill(DistillPlan {
                base,
                query: query.clone(),
                type_filter: extract_type_filter(&stmt.subject),
                depth: extract_depth(&stmt.qualifiers),
            })),
            Verb::Fade => {
                if self.config.fade_suppressed {
                    Err(AqlError::Planning {
                        verb: Verb::Fade,
                        reason: "FADE suppressed by current mood".into(),
                    })
                } else {
                    // FADE does not support FROM/TO range subjects
                    if matches!(&stmt.subject, Subject::TraceRange { .. }) {
                        return Err(AqlError::Planning {
                            verb: Verb::Fade,
                            reason: "FADE does not support FROM/TO range subjects".into(),
                        });
                    }
                    Ok(ExecutionPlan::Fade(FadePlan {
                        base,
                        query: query.clone(),
                        type_filter: extract_type_filter(&stmt.subject),
                        ..Default::default()
                    }))
                }
            }
            Verb::Descend => Ok(ExecutionPlan::Descend(DescendPlan {
                base,
                content: query,
                depth: extract_depth(&stmt.qualifiers).unwrap_or(3),
                magnitude_range: extract_magnitude(&stmt.qualifiers),
            })),
            Verb::Ascend => Ok(ExecutionPlan::Ascend(AscendPlan {
                base,
                content: query,
                depth: extract_depth(&stmt.qualifiers).unwrap_or(3),
                curvature: extract_curvature(&stmt.qualifiers),
            })),
            Verb::Orbit => Ok(ExecutionPlan::Orbit(OrbitPlan {
                base,
                content: query,
                radius: extract_radius(&stmt.qualifiers).unwrap_or(0.1),
                novelty: extract_novelty(&stmt.qualifiers),
            })),
            Verb::Dream => Ok(ExecutionPlan::Dream(DreamPlan {
                base,
                topic: query,
                depth: extract_depth(&stmt.qualifiers),
                novelty: extract_novelty(&stmt.qualifiers),
            })),
            Verb::Imagine => Ok(ExecutionPlan::Imagine(ImaginePlan {
                base,
                premise: query,
                depth: extract_depth(&stmt.qualifiers),
            })),
        };

        // Restore config so mood mutations don't leak into subsequent statements
        self.config = saved_config;

        // Wrap in ConditionalPlan if WHEN clause is present
        match (&stmt.condition, result) {
            (Some(when_clause), Ok(then_plan)) => {
                let else_plan = stmt
                    .else_stmt
                    .as_ref()
                    .map(|e| self.plan_simple(e))
                    .transpose()?
                    .map(Box::new);
                Ok(ExecutionPlan::Conditional(ConditionalPlan {
                    condition: when_clause.clone(),
                    then_plan: Box::new(then_plan),
                    else_plan,
                }))
            }
            (_, result) => result,
        }
    }

    fn build_base(&self, subject: &Subject, qualifiers: &[Qualifier]) -> PlanBase {
        let mut base = PlanBase {
            collection: self.config.active_collection.clone(),
            query_source: resolve_query_source(subject),
            ..Default::default()
        };
        for q in qualifiers {
            match q {
                Qualifier::Confidence(c) => base.confidence_floor = Some(*c),
                Qualifier::Recency(r) => base.recency = Some(*r),
                Qualifier::Within(s) => base.scope = Some(s.clone()),
                Qualifier::Limit(l) => base.limit = Some(*l),
                Qualifier::Valence(v) => base.valence = Some(v.clone()),
                Qualifier::Arousal(a) => base.arousal = Some(a.clone()),
                Qualifier::Mood(m) => base.mood = Some(*m),
                Qualifier::Evidence(e) => base.evidence = Some(*e),
                _ => {}
            }
        }
        if base.limit.is_none() {
            base.limit = Some(self.config.default_limit);
        }
        base
    }
}

// ── Helper extractors ────────────────────────────────────────

fn subject_to_query(subject: &Subject) -> String {
    match subject {
        Subject::Text(t) => t.clone(),
        Subject::TypeWithContent { content, .. } => content.clone(),
        Subject::TraceRange { from, .. } => from.clone(),
        Subject::About(inner) => subject_to_query(inner),
        Subject::SelfRef => "@self".into(),
        Subject::AgentRef(name) => name.clone(),
        Subject::ResultsRef { index } => {
            index.map(|i| format!("@results[{i}]")).unwrap_or("@results".into())
        }
        Subject::LastDream => "@last_dream".into(),
        Subject::DelegateResult => "@delegate.result".into(),
        Subject::TypeFilter(t) => t.to_string(),
    }
}

fn extract_type_filter(subject: &Subject) -> Option<EpistemicType> {
    match subject {
        Subject::TypeFilter(t) | Subject::TypeWithContent { etype: t, .. } => Some(*t),
        _ => None,
    }
}

fn extract_depth(qualifiers: &[Qualifier]) -> Option<u8> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Depth(d) => Some(*d),
        _ => None,
    })
}

fn extract_novelty(qualifiers: &[Qualifier]) -> Option<NoveltyDegree> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Novelty(n) => Some(*n),
        _ => None,
    })
}

fn extract_magnitude(qualifiers: &[Qualifier]) -> Option<MagnitudeRange> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Magnitude(min, max) => Some(MagnitudeRange { min: *min, max: *max }),
        _ => None,
    })
}

fn extract_curvature(qualifiers: &[Qualifier]) -> Option<CurvatureDegree> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Curvature(c) => Some(*c),
        _ => None,
    })
}

fn extract_radius(qualifiers: &[Qualifier]) -> Option<f32> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Radius(r) => Some(*r),
        _ => None,
    })
}

fn extract_confidence(qualifiers: &[Qualifier]) -> Option<f32> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Confidence(c) => Some(*c),
        _ => None,
    })
}

fn extract_as_type(qualifiers: &[Qualifier]) -> Option<EpistemicType> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::As(t) => Some(*t),
        _ => None,
    })
}

fn extract_linking(qualifiers: &[Qualifier]) -> Option<String> {
    qualifiers.iter().find_map(|q| match q {
        Qualifier::Linking(LinkTarget::Text(t)) => Some(t.clone()),
        Qualifier::Linking(LinkTarget::ResultsRef { index }) => {
            Some(format!("@results[{}]", index.unwrap_or(0)))
        }
        Qualifier::Linking(LinkTarget::SelfRef) => Some("@self".to_string()),
        _ => None,
    })
}

/// Determine query source from subject type.
fn resolve_query_source(subject: &Subject) -> QuerySource {
    match subject {
        Subject::ResultsRef { index } => QuerySource::PreviousResults { index: *index },
        Subject::LastDream => QuerySource::LastDream,
        Subject::DelegateResult => QuerySource::DelegateResult,
        // Also handle Text subjects that look like variable references
        // (e.g., parser might produce Text("@results") instead of ResultsRef)
        Subject::Text(t) if t.starts_with("@results") => QuerySource::PreviousResults { index: None },
        Subject::Text(t) if t == "@last_dream" => QuerySource::LastDream,
        Subject::Text(t) if t == "@delegate.result" => QuerySource::DelegateResult,
        _ => QuerySource::Text,
    }
}
