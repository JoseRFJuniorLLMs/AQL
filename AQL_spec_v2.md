# AQL — Agent Cognition Language
## Especificacao Formal v2.0

**Projecto:** AQL (protocolo universal, backend-agnostic)
**Autor:** Jose R F Junior
**Licenca:** AGPL-3.0
**Repositorio:** github.com/JoseRFJuniorLLMs/AQL
**Versao:** 2.0.0 — 2026
**Backend de referencia:** NietzscheDB (experiencia completa)

---

> *O NQL fala para quem le. O NAQ fala para quem calcula. O AQL fala para quem pensa.*
> *E agora — AQL fala para qualquer banco que queira pensar.*

---

## Indice

1. [Filosofia e Posicionamento](#1-filosofia-e-posicionamento)
2. [Stack Completa](#2-stack-completa)
3. [Arquitectura Backend-Agnostic](#3-arquitectura-backend-agnostic)
4. [BackendCapabilities](#4-backendcapabilities)
5. [Layout do Projecto](#5-layout-do-projecto)
6. [Gramatica Formal (Pest PEG)](#6-gramatica-formal-pest-peg)
7. [Sistema de Tipos Epistemicos](#7-sistema-de-tipos-epistemicos)
8. [AST Completo](#8-ast-completo)
9. [Os 13 Verbos Cognitivos](#9-os-13-verbos-cognitivos)
10. [Verbos Geometricos Hiperbolicos](#10-verbos-geometricos-hiperbolicos)
11. [Qualifiers v2](#11-qualifiers-v2)
12. [Execucao Paralela (AND / FORK / JOIN)](#12-execucao-paralela-and--fork--join)
13. [Condicionais (WHEN / ELSE)](#13-condicionais-when--else)
14. [Reactividade (WATCH / SUBSCRIBE)](#14-reactividade-watch--subscribe)
15. [Multi-Agent (SHARE / DELEGATE / NEGOTIATE)](#15-multi-agent-share--delegate--negotiate)
16. [Dimensao Afectiva (Valence / Arousal)](#16-dimensao-afectiva-valence--arousal)
17. [DREAM / IMAGINE — Estados Alterados](#17-dream--imagine--estados-alterados)
18. [EXPLAIN / WHY — Proveniencia](#18-explain--why--proveniencia)
19. [Uncertainty Propagation](#19-uncertainty-propagation)
20. [Transaccoes (ATOMIC)](#20-transaccoes-atomic)
21. [Atencao e Focus](#21-atencao-e-focus)
22. [Cognitive Planner v2](#22-cognitive-planner-v2)
23. [Lowering AQL → Backend](#23-lowering-aql--backend)
24. [State Management — WorkingMemory](#24-state-management--workingmemory)
25. [Resolucao de @self](#25-resolucao-de-self)
26. [Logica de Upsert Epistemico (IMPRINT)](#26-logica-de-upsert-epistemico-imprint)
27. [Cognitive Energy Model](#27-cognitive-energy-model)
28. [Executor e Pipeline](#28-executor-e-pipeline)
29. [Erros e Protocolo de Resposta](#29-erros-e-protocolo-de-resposta)
30. [SDKs Multi-Linguagem](#30-sdks-multi-linguagem)
31. [Mapeamento Por Backend](#31-mapeamento-por-backend)
32. [Exemplos Completos v2](#32-exemplos-completos-v2)
33. [Testes de Aceitacao](#33-testes-de-aceitacao)
34. [Roadmap de Implementacao](#34-roadmap-de-implementacao)

---

## 1. Filosofia e Posicionamento

### v1.0 vs v2.0

| Aspecto | v1.0 | v2.0 |
|---------|------|------|
| Scope | Feature do NietzscheDB | **Protocolo universal** |
| Backend | Apenas NietzscheDB | Qualquer DB (trait) |
| Execucao | Sequential (THEN) | **Paralelo (AND/FORK/JOIN)** |
| Decisao | Sem condicionais | **WHEN/ELSE** |
| Geometria | Ignorada | **DESCEND/ASCEND/ORBIT** |
| Emocao | Ignorada | **Valence/Arousal** |
| Reactivity | Nenhuma | **WATCH/SUBSCRIBE** |
| Multi-agent | Nenhum | **SHARE/DELEGATE/NEGOTIATE** |
| Explicacao | Nenhuma | **EXPLAIN/WHY** |
| Atomicidade | Nenhuma | **ATOMIC blocks** |
| Verbos | 8 | **13** |

### Principio Core

AQL e o **SQL dos agentes cognitivos**. SQL nao pertence ao PostgreSQL — funciona com MySQL, SQLite, Oracle. AQL funciona com qualquer backend que implemente o trait `AqlBackend`.

A experiencia completa — geometria hiperbolica, dream cycles, energy model, L-System — **so existe com NietzscheDB**. Outros backends funcionam com degradacao graceful.

---

## 2. Stack Completa

```
┌─────────────────────────────────────────────────────┐
│                  Agente / LLM / MCP                 │
├─────────────────────────────────────────────────────┤
│           AQL — Agent Cognition Language             │
│  RECALL · RESONATE · REFLECT · TRACE · IMPRINT      │
│  ASSOCIATE · DISTILL · FADE · DESCEND · ASCEND      │
│  DREAM · EXPLAIN · WATCH                            │
│  AND · WHEN · ATOMIC · SHARE · DELEGATE             │
├─────────────────────────────────────────────────────┤
│              AQL Backend Trait                       │
│    fn recall()  fn resonate()  fn imprint() ...     │
├───────────┬───────────┬───────────┬─────────────────┤
│NietzscheDB│   Neo4j   │  Qdrant   │  Pinecone/etc   │
│  (NAQ)    │  (Cypher) │  (REST)   │  (REST)         │
│ Poincare  │ Euclidean │ Euclidean │  Euclidean      │
│ FULL ★    │  Parcial  │  Parcial  │   Minimo        │
└───────────┴───────────┴───────────┴─────────────────┘
```

---

## 3. Arquitectura Backend-Agnostic

### AqlBackend Trait

```rust
/// Qualquer banco que implemente isto pode correr AQL.
/// Verbos com implementacao default fazem fallback graceful.
#[async_trait]
pub trait AqlBackend: Send + Sync {
    /// Declara o que este backend suporta.
    fn capabilities(&self) -> BackendCapabilities;

    /// Nome do backend (para logs e EXPLAIN).
    fn name(&self) -> &str;

    // ── Verbos fundamentais (OBRIGATORIO implementar) ────────────

    /// RECALL: recuperar memoria relevante.
    async fn recall(&self, plan: &RecallPlan) -> Result<CognitiveResult, AqlError>;

    /// IMPRINT: escrever novo conhecimento.
    async fn imprint(&self, plan: &ImprintPlan) -> Result<CognitiveResult, AqlError>;

    /// FADE: esquecimento intencional.
    async fn fade(&self, plan: &FadePlan) -> Result<CognitiveResult, AqlError>;

    /// ASSOCIATE: criar/reforcar ligacao.
    async fn associate(&self, plan: &AssociatePlan) -> Result<CognitiveResult, AqlError>;

    // ── Verbos avancados (default com fallback) ──────────────────

    /// RESONATE: busca por ressonancia semantica.
    /// Default: fallback para recall (sem diffusao).
    async fn resonate(&self, plan: &ResonatePlan) -> Result<CognitiveResult, AqlError> {
        self.recall(&plan.as_recall()).await
    }

    /// TRACE: seguir caminho causal entre dois conceitos.
    /// Default: BFS simples.
    async fn trace(&self, plan: &TracePlan) -> Result<CognitiveResult, AqlError> {
        self.default_bfs_trace(plan).await
    }

    /// DISTILL: extrair Pattern de multiplos episodios.
    /// Default: clustering client-side.
    async fn distill(&self, plan: &DistillPlan) -> Result<CognitiveResult, AqlError> {
        self.default_client_cluster(plan).await
    }

    /// REFLECT: meta-cognicao sobre estado do agente/grafo.
    /// Default: stats basicos.
    async fn reflect(&self, plan: &ReflectPlan) -> Result<CognitiveResult, AqlError> {
        self.default_stats_reflect(plan).await
    }

    // ── Verbos geometricos (so backends hiperbolicos) ────────────

    /// DESCEND: navegar para filhos na hierarquia hiperbolica.
    async fn descend(&self, _plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "DESCEND".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    /// ASCEND: subir para conceitos mais abstractos.
    async fn ascend(&self, _plan: &AscendPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "ASCEND".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    /// ORBIT: nos na mesma camada hiperbolica.
    async fn orbit(&self, _plan: &OrbitPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "ORBIT".into(),
            reason: "requires hyperbolic geometry backend".into(),
        })
    }

    // ── Verbos de estados alterados (so NietzscheDB) ─────────────

    /// DREAM: activar dream cycle sobre um tema.
    async fn dream(&self, _plan: &DreamPlan) -> Result<CognitiveResult, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "DREAM".into(),
            reason: "requires dream cycle support (NietzscheDB)".into(),
        })
    }

    // ── Reactivity (opcional) ────────────────────────────────────

    /// WATCH: registar callback para mudancas.
    async fn watch(&self, _plan: &WatchPlan) -> Result<WatchHandle, AqlError> {
        Err(AqlError::UnsupportedVerb {
            verb: "WATCH".into(),
            reason: "backend does not support reactive subscriptions".into(),
        })
    }

    // ── EXPLAIN (opcional, mas recomendado) ──────────────────────

    /// Retorna proveniencia e raciocinio da ultima operacao.
    async fn explain(&self, _plan: &ExplainPlan) -> Result<Explanation, AqlError> {
        Ok(Explanation::not_supported())
    }
}
```

---

## 4. BackendCapabilities

```rust
/// O backend declara o que suporta. O Planner usa isto para
/// escolher estrategias e degradar gracefully.
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    // ── Geometria ────────────────────────────────────────
    pub geometry:        Geometry,
    pub has_magnitude:   bool,   // hierarquia por profundidade
    pub has_curvature:   bool,   // curvatura local

    // ── Busca ────────────────────────────────────────────
    pub has_vector_search: bool, // KNN / ANN
    pub has_full_text:     bool, // busca textual
    pub has_diffusion:     bool, // wave propagation
    pub has_graph_algos:   bool, // PageRank, Louvain, BFS, etc.

    // ── Grafo ────────────────────────────────────────────
    pub has_edges:       bool,   // relacoes explicitas
    pub has_typed_edges: bool,   // edges com tipo
    pub has_edge_weight: bool,   // peso nas edges
    pub has_traversal:   bool,   // BFS, Dijkstra, shortest path

    // ── Energia / Cognicao ───────────────────────────────
    pub has_energy:      bool,   // energy model nos nos
    pub has_decay:       bool,   // temporal decay
    pub has_valence:     bool,   // dimensao emocional
    pub has_arousal:     bool,   // activacao emocional
    pub has_sleep:       bool,   // consolidacao tipo sono
    pub has_dream:       bool,   // dream cycles

    // ── Tempo ────────────────────────────────────────────
    pub has_timestamps:  bool,   // created_at, updated_at
    pub has_ttl:         bool,   // expiration

    // ── Operacional ──────────────────────────────────────
    pub max_batch_size:  usize,  // limite de batch
    pub supports_atomic: bool,   // transaccoes
    pub supports_watch:  bool,   // reactive subscriptions
}

#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    /// NietzscheDB — Poincare ball model.
    Hyperbolic { curvature: f64 },
    /// Qdrant, Pinecone, Weaviate, pgvector.
    Euclidean,
    /// Milvus (opcao).
    Spherical,
    /// Redis, MongoDB — sem geometria vectorial.
    None,
}

impl BackendCapabilities {
    /// NietzscheDB: tudo activo.
    pub fn nietzschedb() -> Self {
        Self {
            geometry:          Geometry::Hyperbolic { curvature: -1.0 },
            has_magnitude:     true,
            has_curvature:     true,
            has_vector_search: true,
            has_full_text:     true,
            has_diffusion:     true,
            has_graph_algos:   true,
            has_edges:         true,
            has_typed_edges:   true,
            has_edge_weight:   true,
            has_traversal:     true,
            has_energy:        true,
            has_decay:         true,
            has_valence:       true,
            has_arousal:       true,
            has_sleep:         true,
            has_dream:         true,
            has_timestamps:    true,
            has_ttl:           true,
            max_batch_size:    10_000,
            supports_atomic:   true,
            supports_watch:    true,
        }
    }

    /// Neo4j: grafo forte, sem vectores nativos.
    pub fn neo4j() -> Self {
        Self {
            geometry:          Geometry::Euclidean,
            has_magnitude:     false,
            has_curvature:     false,
            has_vector_search: true,  // Neo4j 5.x tem vector index
            has_full_text:     true,
            has_diffusion:     false,
            has_graph_algos:   true,  // GDS library
            has_edges:         true,
            has_typed_edges:   true,
            has_edge_weight:   true,
            has_traversal:     true,
            has_energy:        false, // simulavel via property
            has_decay:         false,
            has_valence:       false,
            has_arousal:       false,
            has_sleep:         false,
            has_dream:         false,
            has_timestamps:    true,
            has_ttl:           false,
            max_batch_size:    50_000,
            supports_atomic:   true,  // ACID transactions
            supports_watch:    false,
        }
    }

    /// Qdrant: vectores fortes, sem grafo.
    pub fn qdrant() -> Self {
        Self {
            geometry:          Geometry::Euclidean,
            has_magnitude:     false,
            has_curvature:     false,
            has_vector_search: true,
            has_full_text:     true,
            has_diffusion:     false,
            has_graph_algos:   false,
            has_edges:         false,  // payload links only
            has_typed_edges:   false,
            has_edge_weight:   false,
            has_traversal:     false,
            has_energy:        false,
            has_decay:         false,
            has_valence:       false,
            has_arousal:       false,
            has_sleep:         false,
            has_dream:         false,
            has_timestamps:    true,
            has_ttl:           false,
            max_batch_size:    10_000,
            supports_atomic:   false,
            supports_watch:    false,
        }
    }

    /// Pinecone: vectores serverless, minimalista.
    pub fn pinecone() -> Self {
        Self {
            geometry:          Geometry::Euclidean,
            has_vector_search: true,
            has_full_text:     false,
            has_timestamps:    true,
            max_batch_size:    1_000,
            // tudo o resto false
            ..Self::minimal()
        }
    }

    /// pgvector (PostgreSQL): SQL + vectores.
    pub fn pgvector() -> Self {
        Self {
            geometry:          Geometry::Euclidean,
            has_vector_search: true,
            has_full_text:     true,
            has_edges:         true,  // tabela de relacoes
            has_typed_edges:   true,
            has_edge_weight:   true,
            has_timestamps:    true,
            max_batch_size:    50_000,
            supports_atomic:   true,  // PostgreSQL ACID
            ..Self::minimal()
        }
    }

    /// Redis Stack: cache + busca.
    pub fn redis() -> Self {
        Self {
            geometry:          Geometry::None,
            has_vector_search: true,  // RediSearch
            has_full_text:     true,
            has_ttl:           true,
            has_timestamps:    true,
            max_batch_size:    10_000,
            ..Self::minimal()
        }
    }

    fn minimal() -> Self {
        Self {
            geometry: Geometry::None,
            has_magnitude: false, has_curvature: false,
            has_vector_search: false, has_full_text: false,
            has_diffusion: false, has_graph_algos: false,
            has_edges: false, has_typed_edges: false,
            has_edge_weight: false, has_traversal: false,
            has_energy: false, has_decay: false,
            has_valence: false, has_arousal: false,
            has_sleep: false, has_dream: false,
            has_timestamps: false, has_ttl: false,
            max_batch_size: 100, supports_atomic: false,
            supports_watch: false,
        }
    }
}
```

---

## 5. Layout do Projecto

```
aql/                              ← Repo independente
├── Cargo.toml                    ← Workspace
├── LICENSE                       ← AGPL-3.0
├── README.md
│
├── aql-core/                     ← Parser, AST, Planner (zero deps de DB)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── grammar.pest          ← PEG grammar completa
│   │   ├── parser.rs             ← pest → AST
│   │   ├── ast.rs                ← Todos os tipos AST
│   │   ├── types.rs              ← EpistemicType, RecencyDegree, etc.
│   │   ├── planner.rs            ← CognitivePlanner (backend-aware)
│   │   ├── traits.rs             ← AqlBackend trait
│   │   ├── capabilities.rs       ← BackendCapabilities
│   │   ├── executor.rs           ← AqlExecutor (generic over backend)
│   │   ├── memory.rs             ← WorkingMemory
│   │   ├── energy.rs             ← EnergyHooks
│   │   ├── parallel.rs           ← AND/FORK/JOIN runtime
│   │   ├── conditionals.rs       ← WHEN/ELSE evaluation
│   │   ├── atomic.rs             ← Transaction manager
│   │   ├── explain.rs            ← Provenance tracking
│   │   ├── affect.rs             ← Valence/Arousal/Mood
│   │   ├── attention.rs          ← Focus/Salience/Attention budget
│   │   ├── uncertainty.rs        ← Confidence propagation
│   │   ├── multiagent.rs         ← SHARE/DELEGATE/NEGOTIATE
│   │   ├── reactive.rs           ← WATCH/SUBSCRIBE runtime
│   │   └── error.rs              ← AqlError
│   └── Cargo.toml
│
├── aql-nietzschedb/              ← impl AqlBackend for NietzscheDB
│   ├── src/
│   │   ├── lib.rs
│   │   ├── backend.rs            ← NietzscheBackend (gRPC/NAQ)
│   │   ├── hyperbolic.rs         ← DESCEND/ASCEND/ORBIT nativos
│   │   ├── dream.rs              ← DREAM cycle integration
│   │   └── lowering.rs           ← AQL plan → NAQ instructions
│   └── Cargo.toml
│
├── aql-neo4j/                    ← impl AqlBackend for Neo4j
│   ├── src/
│   │   ├── lib.rs
│   │   ├── backend.rs            ← Neo4jBackend (Bolt protocol)
│   │   └── lowering.rs           ← AQL plan → Cypher queries
│   └── Cargo.toml
│
├── aql-qdrant/                   ← impl AqlBackend for Qdrant
│   ├── src/
│   │   ├── lib.rs
│   │   ├── backend.rs            ← QdrantBackend (gRPC/REST)
│   │   └── lowering.rs           ← AQL plan → Qdrant API calls
│   └── Cargo.toml
│
├── aql-pgvector/                 ← impl AqlBackend for PostgreSQL+pgvector
│   ├── src/
│   │   ├── lib.rs
│   │   ├── backend.rs            ← PgVectorBackend (sqlx)
│   │   └── lowering.rs           ← AQL plan → SQL queries
│   └── Cargo.toml
│
├── aql-redis/                    ← impl AqlBackend for Redis Stack
│   ├── src/
│   │   ├── lib.rs
│   │   ├── backend.rs            ← RedisBackend
│   │   └── lowering.rs           ← AQL plan → Redis commands
│   └── Cargo.toml
│
├── aql-python/                   ← Python bindings (PyO3)
│   ├── src/lib.rs
│   ├── python/aql/__init__.py
│   └── pyproject.toml
│
├── aql-js/                       ← JavaScript/TypeScript SDK
│   ├── src/
│   ├── package.json
│   └── tsconfig.json
│
├── aql-wasm/                     ← WASM build (browser, edge)
│   ├── src/lib.rs
│   └── Cargo.toml
│
└── aql-cli/                      ← CLI interactivo (REPL)
    ├── src/main.rs
    └── Cargo.toml
```

---

## 6. Gramatica Formal (Pest PEG) — v2.0

```pest
// ═══════════════════════════════════════════════════════════════
// AQL v2.0 — Agent Cognition Language
// Grammar for pest parser (Rust)
// ═══════════════════════════════════════════════════════════════

program = { SOI ~ (statement ~ NEWLINE*)+ ~ EOI }

// ── Statements ───────────────────────────────────────────────

statement = {
    atomic_block
  | parallel_block
  | watch_statement
  | explain_statement
  | chain_statement
}

chain_statement = {
    verb_statement ~ (THEN ~ verb_statement)*
}

verb_statement = {
    conditional_statement
  | simple_statement
}

conditional_statement = {
    simple_statement ~ when_clause ~ (ELSE ~ simple_statement)?
}

when_clause = {
    WHEN ~ condition_expr
}

condition_expr = {
    "@results" ~ "." ~ ident ~ comp_op ~ (number | string)
}

comp_op = { ">=" | "<=" | "!=" | ">" | "<" | "==" }

simple_statement = {
    verb ~ subject ~ qualifier*
}

// ── Parallel execution ───────────────────────────────────────

parallel_block = {
    simple_statement ~ (AND ~ simple_statement)+
    ~ (THEN ~ verb_statement)?
}

// ── Atomic transaction ───────────────────────────────────────

atomic_block = {
    ATOMIC ~ "{" ~ NEWLINE* ~ (statement ~ NEWLINE*)+ ~ "}"
}

// ── Watch / Subscribe ────────────────────────────────────────

watch_statement = {
    WATCH ~ subject ~ ON_CHANGE ~ chain_statement
  | SUBSCRIBE ~ subject ~ ON_INSERT ~ chain_statement
}

// ── Explain ──────────────────────────────────────────────────

explain_statement = {
    EXPLAIN ~ simple_statement
}

// ── 13 Verbs ─────────────────────────────────────────────────

verb = {
    RECALL | RESONATE | REFLECT | TRACE
  | IMPRINT | ASSOCIATE | DISTILL | FADE
  | DESCEND | ASCEND | ORBIT
  | DREAM | IMAGINE
}

RECALL    = { "RECALL" }
RESONATE  = { "RESONATE" }
REFLECT   = { "REFLECT" }
TRACE     = { "TRACE" }
IMPRINT   = { "IMPRINT" }
ASSOCIATE = { "ASSOCIATE" }
DISTILL   = { "DISTILL" }
FADE      = { "FADE" }
DESCEND   = { "DESCEND" }
ASCEND    = { "ASCEND" }
ORBIT     = { "ORBIT" }
DREAM     = { "DREAM" }
IMAGINE   = { "IMAGINE" }

// ── Keywords ─────────────────────────────────────────────────

THEN      = { "THEN" }
AND       = { "AND" }
WHEN      = { "WHEN" }
ELSE      = { "ELSE" }
ATOMIC    = { "ATOMIC" }
WATCH     = { "WATCH" }
SUBSCRIBE = { "SUBSCRIBE" }
ON_CHANGE = { "ON_CHANGE" }
ON_INSERT = { "ON_INSERT" }
EXPLAIN   = { "EXPLAIN" }
FROM      = { "FROM" }
TO        = { "TO" }
ABOUT     = { "ABOUT" }

// ── Multi-agent keywords ─────────────────────────────────────

SHARE     = { "SHARE" }
DELEGATE  = { "DELEGATE" }
NEGOTIATE = { "NEGOTIATE" }
WITH_AGENT = { "WITH" }
TO_AGENT   = { "TO" }
POLICY     = { "POLICY" }

// ── Subjects ─────────────────────────────────────────────────

subject = {
    trace_range          // FROM "X" TO "Y"
  | type_with_content    // Belief:"quantum"
  | self_ref             // @self
  | agent_ref            // agent:"eva-2"
  | results_ref          // @results, @results[0], @last_dream
  | text                 // "some text"
  | type_filter          // Belief, Experience, Pattern, Signal, Intention
}

trace_range       = { FROM ~ string ~ TO ~ string }
type_with_content = { epistemic_type ~ ":" ~ string }
self_ref          = { "@self" }
agent_ref         = { "agent:" ~ string }
results_ref       = { "@results" ~ ("[" ~ number ~ "]")? | "@last_dream" | "@delegate.result" }
text              = { string }
type_filter       = { epistemic_type }

epistemic_type = {
    "Belief" | "Experience" | "Pattern" | "Signal" | "Intention"
}

// ── Qualifiers (v2 — expanded) ───────────────────────────────

qualifier = {
    confidence_q | recency_q | depth_q | within_q
  | as_q | linking_q | novelty_q | limit_q
  | magnitude_q | curvature_q | radius_q
  | valence_q | arousal_q | mood_q
  | evidence_q
  | with_agent_q | to_agent_q | policy_q
}

confidence_q  = { "CONFIDENCE" ~ number }
recency_q     = { "RECENCY" ~ recency_degree }
depth_q       = { "DEPTH" ~ integer }
within_q      = { "WITHIN" ~ (scope | string) }
as_q          = { "AS" ~ epistemic_type }
linking_q     = { "LINKING" ~ (string | results_ref) }
novelty_q     = { "NOVELTY" ~ novelty_degree }
limit_q       = { "LIMIT" ~ integer }

// v2 qualifiers
magnitude_q   = { "MAGNITUDE" ~ number_range }
curvature_q   = { "CURVATURE" ~ curvature_degree }
radius_q      = { "RADIUS" ~ number }
valence_q     = { "VALENCE" ~ (valence_polarity | number) }
arousal_q     = { "AROUSAL" ~ (arousal_level | number) }
mood_q        = { "MOOD" ~ mood_state }
evidence_q    = { "EVIDENCE" ~ integer }
with_agent_q  = { "WITH" ~ agent_ref }
to_agent_q    = { "TO" ~ agent_ref }
policy_q      = { "POLICY" ~ policy_name }

// ── Qualifier values ─────────────────────────────────────────

recency_degree   = { "fresh" | "recent" | "distant" | "ancient" }
novelty_degree   = { "high" | "medium" | "low" }
scope            = { "session" | "collection" | "graph" }
curvature_degree = { "high" | "medium" | "low" | "flat" }

valence_polarity = { "positive" | "negative" | "neutral" }
arousal_level    = { "high" | "medium" | "low" | "calm" }
mood_state       = { "creative" | "analytical" | "anxious" | "focused" | "exploratory" | "conservative" }
policy_name      = { "weighted_average" | "keep_higher" | "replace_always" | "create_conflict" }

number_range     = { number ~ ".." ~ number }

// ── Literals ─────────────────────────────────────────────────

string  = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
number  = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
integer = @{ ASCII_DIGIT+ }
ident   = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

WHITESPACE = _{ " " | "\t" }
COMMENT    = _{ "#" ~ (!NEWLINE ~ ANY)* }
```

---

## 7. Sistema de Tipos Epistemicos

### Tipos core (v1.0, mantidos)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpistemicType {
    Belief,      // Conhecimento declarativo (Semantic no NietzscheDB)
    Experience,  // Memoria episodica (Episodic)
    Pattern,     // Padrao extraido de multiplas experiencias (Semantic)
    Signal,      // Informacao transitoria (Semantic, high decay)
    Intention,   // Objectivo ou plano (Concept)
}

impl EpistemicType {
    /// Energia inicial por tipo.
    pub fn initial_energy(&self) -> f32 {
        match self {
            Self::Belief     => 0.6,
            Self::Experience => 0.5,
            Self::Pattern    => 0.8,
            Self::Signal     => 0.3,
            Self::Intention  => 0.7,
        }
    }

    /// Boost de energia quando acedido.
    pub fn access_energy_boost(&self) -> f32 {
        match self {
            Self::Belief     => 0.05,
            Self::Experience => 0.03,
            Self::Pattern    => 0.02,
            Self::Signal     => 0.10,
            Self::Intention  => 0.04,
        }
    }

    /// Taxa de decay natural.
    pub fn decay_rate(&self) -> f64 {
        match self {
            Self::Belief     => 0.001,
            Self::Experience => 0.005,
            Self::Pattern    => 0.0005,
            Self::Signal     => 0.05,
            Self::Intention  => 0.01,
        }
    }

    /// Mapeamento para NodeType do NietzscheDB.
    pub fn to_nietzsche_node_type(&self) -> &str {
        match self {
            Self::Belief     => "Semantic",
            Self::Experience => "Episodic",
            Self::Pattern    => "Semantic",
            Self::Signal     => "Semantic",
            Self::Intention  => "Concept",
        }
    }
}
```

### v2.0: Dimensao Afectiva por Tipo

```rust
impl EpistemicType {
    /// Valence default por tipo (v2.0).
    pub fn default_valence(&self) -> f32 {
        match self {
            Self::Belief     =>  0.0,  // neutro
            Self::Experience =>  0.0,  // depende do conteudo
            Self::Pattern    =>  0.1,  // ligeiramente positivo (descoberta)
            Self::Signal     =>  0.0,  // neutro
            Self::Intention  =>  0.3,  // positivo (motivacao)
        }
    }

    /// Arousal default por tipo (v2.0).
    pub fn default_arousal(&self) -> f32 {
        match self {
            Self::Belief     => 0.3,  // baixo
            Self::Experience => 0.5,  // medio
            Self::Pattern    => 0.4,  // medio-baixo
            Self::Signal     => 0.8,  // alto (urgencia)
            Self::Intention  => 0.6,  // medio-alto
        }
    }
}
```

---

## 8. AST Completo (v2.0)

```rust
/// Um programa AQL completo.
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Um statement pode ser simples, chain, paralelo, atomico, ou reactivo.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Verbo simples com qualifiers.
    Simple(SimpleStatement),
    /// Cadeia THEN: step1 THEN step2 THEN ...
    Chain(ChainStatement),
    /// Execucao paralela: step1 AND step2 AND ... [THEN join_step]
    Parallel(ParallelStatement),
    /// Bloco atomico: ATOMIC { ... }
    Atomic(AtomicBlock),
    /// Watch reactivo: WATCH subject ON_CHANGE ...
    Watch(WatchStatement),
    /// Explain: EXPLAIN statement
    Explain(ExplainStatement),
}

#[derive(Debug, Clone)]
pub struct SimpleStatement {
    pub verb:       Verb,
    pub subject:    Subject,
    pub qualifiers: Vec<Qualifier>,
    pub condition:  Option<WhenClause>,
    pub else_stmt:  Option<Box<SimpleStatement>>,
}

#[derive(Debug, Clone)]
pub struct ChainStatement {
    pub steps: Vec<SimpleStatement>,
}

#[derive(Debug, Clone)]
pub struct ParallelStatement {
    pub branches:  Vec<SimpleStatement>,  // executam em paralelo
    pub join_step: Option<SimpleStatement>, // THEN apos todas completarem
}

#[derive(Debug, Clone)]
pub struct AtomicBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct WatchStatement {
    pub subject:  Subject,
    pub trigger:  WatchTrigger,
    pub reaction: Box<Statement>,
}

#[derive(Debug, Clone)]
pub enum WatchTrigger {
    OnChange,
    OnInsert,
}

#[derive(Debug, Clone)]
pub struct ExplainStatement {
    pub inner: Box<SimpleStatement>,
}

#[derive(Debug, Clone)]
pub struct WhenClause {
    pub field:    String,        // ex: "count", "avg_energy"
    pub op:       CompOp,
    pub value:    ConditionValue,
}

#[derive(Debug, Clone)]
pub enum ConditionValue {
    Float(f64),
    Int(i64),
    Str(String),
}

// ── 13 Verbos ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verb {
    // v1.0 core
    Recall,
    Resonate,
    Reflect,
    Trace,
    Imprint,
    Associate,
    Distill,
    Fade,
    // v2.0 geometric
    Descend,
    Ascend,
    Orbit,
    // v2.0 altered states
    Dream,
    Imagine,
}

// ── Subjects ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Subject {
    Text(String),
    TypeFilter(EpistemicType),
    TypeWithContent { etype: EpistemicType, content: String },
    SelfRef,
    AgentRef(String),
    ResultsRef { index: Option<usize> },
    LastDream,
    DelegateResult,
    TraceRange { from: String, to: String },
}

// ── Qualifiers (v2.0 expanded) ───────────────────────────────

#[derive(Debug, Clone)]
pub enum Qualifier {
    // v1.0
    Confidence(f32),
    Recency(RecencyDegree),
    Depth(u8),
    Within(ContextScope),
    As(EpistemicType),
    Linking(LinkTarget),
    Novelty(NoveltyDegree),
    Limit(u32),

    // v2.0 geometric
    Magnitude(f32, f32),           // range: min..max
    Curvature(CurvatureDegree),
    Radius(f32),                    // for ORBIT

    // v2.0 affective
    Valence(ValenceSpec),
    Arousal(ArousalSpec),
    Mood(MoodState),

    // v2.0 epistemic
    Evidence(u32),                  // number of supporting observations

    // v2.0 multi-agent
    WithAgent(String),
    ToAgent(String),
    Policy(ConflictPolicy),
}

#[derive(Debug, Clone)]
pub enum LinkTarget {
    Text(String),
    ResultsRef { index: Option<usize> },
    SelfRef,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecencyDegree { Fresh, Recent, Distant, Ancient }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoveltyDegree { High, Medium, Low }

#[derive(Debug, Clone, PartialEq)]
pub enum ContextScope {
    Session,
    Collection,
    Graph,
    Named(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurvatureDegree { High, Medium, Low, Flat }

#[derive(Debug, Clone)]
pub enum ValenceSpec {
    Positive,
    Negative,
    Neutral,
    Exact(f32),    // -1.0 to 1.0
}

#[derive(Debug, Clone)]
pub enum ArousalSpec {
    High,
    Medium,
    Low,
    Calm,
    Exact(f32),    // 0.0 to 1.0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoodState {
    Creative,      // NOVELTY high, DEPTH unlimited, RESONATE amplified
    Analytical,    // CONFIDENCE high, DEPTH limited, TRACE preferred
    Anxious,       // RECENCY fresh, CONFIDENCE high, short chains
    Focused,       // LIMIT small, WITHIN session, suppress noise
    Exploratory,   // NOVELTY high, DEPTH high, FADE suppressed
    Conservative,  // CONFIDENCE high, NOVELTY low, IMPRINT restricted
}
```

---

## 9. Os 13 Verbos Cognitivos

### RECALL

**Semantica:** Recuperar memoria relevante e reforcar a energia dos nos acedidos.

```aql
RECALL "quantum physics"
RECALL "quantum physics" CONFIDENCE 0.8
RECALL Belief:"quantum" RECENCY recent LIMIT 5
RECALL "trauma" VALENCE negative AROUSAL high
```

**Side-effects implicitos:**
- `BoostAccessedNodes` — incrementa energy dos resultados
- `CreateTemporalEdge` — cria edge session → no
- `RecordAccessPattern` — regista no planner adaptativo

---

### RESONATE

**Semantica:** Encontrar por ressonancia semantica. O servidor decide o metodo — KNN, diffusao, ou combinacao.

```aql
RESONATE "consciousness emerges from complexity"
RESONATE "quantum entanglement" NOVELTY high DEPTH 3
RESONATE "beauty" MOOD creative VALENCE positive
```

**Side-effects:** `BoostAccessedNodes`, `RecordResonancePattern`

---

### REFLECT

**Semantica:** Meta-cognicao. O agente questiona o seu proprio estado ou o estado do grafo.

```aql
REFLECT @self
REFLECT @self WITHIN session
REFLECT Pattern WITHIN "physics-collection"
```

**Side-effects:** `RecordAccessPattern`

---

### TRACE

**Semantica:** Seguir uma narrativa causal entre dois conceitos.

```aql
TRACE FROM "observation" TO "conclusion"
TRACE FROM "initial-hypothesis" TO "validated-theory" DEPTH 5
```

**Side-effects:** `BoostPathNodes`, `CreateTemporalEdge`

---

### IMPRINT

**Semantica:** Escrever novo conhecimento. Inclui logica de upsert epistemico (Seccao 26).

```aql
IMPRINT "nova hipotese sobre colapso de funcao de onda"
IMPRINT "X e verdadeiro" CONFIDENCE 0.6 LINKING "quantum" AS Belief
IMPRINT "descoberta emocionante!" VALENCE positive AROUSAL high CONFIDENCE 0.8
```

**Side-effects:** `AssociateToSessionContext`, `BoostLinkedNodes`

---

### ASSOCIATE

**Semantica:** Criar ou reforcar uma associacao entre conceitos.

```aql
ASSOCIATE "quantum" LINKING "consciousness"
ASSOCIATE Belief:"entanglement" LINKING "non-locality" CONFIDENCE 0.9
```

**Side-effects:** `CreateTemporalEdge`, `BoostLinkedNodes`

---

### DISTILL

**Semantica:** Extrair um Pattern de multiplos episodios/experiencias.

```aql
DISTILL Experience WITHIN session
DISTILL Experience:"fisica quantica" DEPTH 4 CONFIDENCE 0.7
```

**Side-effects:** `CreatePatternNode`, `LinkSourceEpisodes`

---

### FADE

**Semantica:** Esquecimento intencional. Reduz energy ou elimina nos abaixo de threshold.

```aql
FADE Signal CONFIDENCE 0.2
FADE Experience RECENCY ancient WITHIN session
FADE Experience VALENCE negative RECENCY distant
```

**Comportamento:** Se a energy apos decremento cair abaixo de 0.05, o no e eliminado.

**Side-effects:** `RecordFadeEvent`

---

### DESCEND (v2.0 — Geometrico)

**Semantica:** Navegar para filhos na hierarquia hiperbolica. Nos com **maior magnitude** (mais profundos no Poincare ball) sao mais especificos.

```aql
DESCEND "physics" DEPTH 3
DESCEND "mathematics" DEPTH 2 MAGNITUDE 0.3..0.7
```

**Requisito:** `geometry == Hyperbolic`. Sem backend hiperbolico → `AqlError::UnsupportedVerb`.

**Logica:**
- Encontra no(s) que match "physics"
- Filtra vizinhos com `magnitude > source.magnitude`
- Recursivamente ate `DEPTH`

---

### ASCEND (v2.0 — Geometrico)

**Semantica:** Subir para conceitos mais abstractos (menor magnitude = mais perto da origem do Poincare ball).

```aql
ASCEND "quark" DEPTH 2
ASCEND "photosynthesis" DEPTH 3 CURVATURE low
```

**Requisito:** `geometry == Hyperbolic`.

**Logica:**
- Encontra no(s) que match "quark"
- Filtra vizinhos com `magnitude < source.magnitude`
- Recursivamente ate `DEPTH`

---

### ORBIT (v2.0 — Geometrico)

**Semantica:** Encontrar nos na mesma "camada" hiperbolica — conceitos com nivel de abstraccao similar.

```aql
ORBIT "consciousness" RADIUS 0.1
ORBIT "neuron" RADIUS 0.05 NOVELTY high
```

**Requisito:** `geometry == Hyperbolic`.

**Logica:**
- Encontra no(s) que match "consciousness"
- Filtra nos com `|magnitude - source.magnitude| < RADIUS`
- Retorna nos na mesma camada de abstraccao

---

### DREAM (v2.0 — Estados Alterados)

**Semantica:** Activar dream cycle sobre um tema. Producao criativa nao-deterministica.

```aql
DREAM ABOUT "quantum consciousness"
DREAM ABOUT "mathematics" DEPTH 5 NOVELTY high
DREAM ABOUT @self
```

**Requisito:** `has_dream == true` (apenas NietzscheDB).

**Logica:**
- Activa dream cycle do NietzscheDB (`nietzsche-dream`)
- Consolidacao de memorias: reforco de edges frequentes
- Producao de novos Pattern nodes a partir de activacoes aleatorias
- Resultado acessivel via `@last_dream`

---

### IMAGINE (v2.0 — Estados Alterados)

**Semantica:** Raciocinio contrafactual. "E se X fosse verdade?"

```aql
IMAGINE "what if gravity were repulsive"
IMAGINE "quantum = consciousness" DEPTH 3
```

**Logica:**
- Cria branch temporario no grafo (sandbox)
- Aplica a premissa contrafactual
- Propaga consequencias via edges causais
- Retorna as implicacoes sem alterar o grafo real

---

## 10. Verbos Geometricos Hiperbolicos — Detalhes

### Porque e Unico

Nenhuma linguagem de query no mundo oferece navegacao hiperbolica nativa. O Poincare ball tem uma propriedade que grafos euclidianos nao tem: **a magnitude de um vector codifica a profundidade na hierarquia**.

```
Magnitude ≈ 0.0  →  Conceito raiz (ex: "Knowledge")
Magnitude ≈ 0.3  →  Dominio (ex: "Physics")
Magnitude ≈ 0.5  →  Sub-dominio (ex: "Quantum Mechanics")
Magnitude ≈ 0.7  →  Conceito especifico (ex: "Heisenberg Uncertainty")
Magnitude ≈ 0.9  →  Facto concreto (ex: "h-bar = 1.054e-34 J·s")
```

### Implementacao no Backend NietzscheDB

```rust
impl AqlBackend for NietzscheBackend {
    async fn descend(&self, plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        // 1. Encontra no-fonte
        let source = self.find_by_content(&plan.content).await?;
        let source_mag = self.magnitude(&source.coords);

        // 2. KNN centrado no source
        let neighbors = self.knn(&source.coords, plan.depth * 20).await?;

        // 3. Filtra: so nos com magnitude MAIOR (mais profundos)
        let descendants: Vec<_> = neighbors.into_iter()
            .filter(|n| {
                let mag = self.magnitude(&n.coords);
                mag > source_mag && mag <= source_mag + 0.3 * plan.depth as f32
            })
            .filter(|n| plan.magnitude_range
                .map(|(lo, hi)| { let m = self.magnitude(&n.coords); m >= lo && m <= hi })
                .unwrap_or(true))
            .collect();

        Ok(CognitiveResult::from_nodes(descendants))
    }

    async fn ascend(&self, plan: &AscendPlan) -> Result<CognitiveResult, AqlError> {
        let source = self.find_by_content(&plan.content).await?;
        let source_mag = self.magnitude(&source.coords);

        let neighbors = self.knn(&source.coords, plan.depth * 20).await?;

        // Filtra: so nos com magnitude MENOR (mais abstractos)
        let ancestors: Vec<_> = neighbors.into_iter()
            .filter(|n| self.magnitude(&n.coords) < source_mag)
            .collect();

        Ok(CognitiveResult::from_nodes(ancestors))
    }

    async fn orbit(&self, plan: &OrbitPlan) -> Result<CognitiveResult, AqlError> {
        let source = self.find_by_content(&plan.content).await?;
        let source_mag = self.magnitude(&source.coords);

        let neighbors = self.knn(&source.coords, 100).await?;

        // Filtra: nos na mesma "camada" (magnitude similar)
        let peers: Vec<_> = neighbors.into_iter()
            .filter(|n| {
                let mag = self.magnitude(&n.coords);
                (mag - source_mag).abs() < plan.radius
            })
            .collect();

        Ok(CognitiveResult::from_nodes(peers))
    }

    fn magnitude(&self, coords: &[f32]) -> f32 {
        coords.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}
```

### Fallback para Backends Euclidianos

Backends sem geometria hiperbolica podem **simular** DESCEND/ASCEND com metadata:

```rust
impl AqlBackend for QdrantFallback {
    async fn descend(&self, plan: &DescendPlan) -> Result<CognitiveResult, AqlError> {
        // Sem magnitude geometrica → usa campo "depth" no payload
        let source = self.find_by_content(&plan.content).await?;
        let source_depth = source.payload.get("depth")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let results = self.search_with_filter(
            &plan.content,
            json!({ "must": [{ "key": "depth", "range": {
                "gt": source_depth,
                "lte": source_depth + plan.depth as i64
            }}]})
        ).await?;

        Ok(CognitiveResult::from_nodes(results))
    }
}
```

A simulacao funciona mas perde a elegancia geometrica. E o tipo de coisa que faz um user querer migrar para NietzscheDB.

---

## 11. Qualifiers v2

### CONFIDENCE (v1.0, mantido)

```rust
pub fn confidence_to_energy_floor(confidence: f32) -> f32 {
    confidence * 0.5
}

pub fn confidence_to_initial_energy(confidence: f32) -> f32 {
    confidence  // 1:1 para criacao
}
```

### RECENCY (v1.0, mantido)

```rust
pub enum RecencyDegree {
    Fresh,    // < 5 minutos
    Recent,   // < 1 hora
    Distant,  // < 24 horas
    Ancient,  // sem limite
}

impl RecencyDegree {
    pub fn to_time_window_secs(&self) -> Option<i64> {
        match self {
            Self::Fresh   => Some(300),
            Self::Recent  => Some(3_600),
            Self::Distant => Some(86_400),
            Self::Ancient => None,
        }
    }

    pub fn to_energy_floor(&self) -> f32 {
        match self {
            Self::Fresh   => 0.70,
            Self::Recent  => 0.40,
            Self::Distant => 0.20,
            Self::Ancient => 0.05,
        }
    }
}
```

### MAGNITUDE (v2.0 — Novo)

Filtro por profundidade hiperbolica. So funciona com `Geometry::Hyperbolic`.

```aql
RECALL "physics" MAGNITUDE 0.2..0.5      -- nos entre 0.2 e 0.5 de magnitude
DESCEND "math" MAGNITUDE 0.5..0.9        -- so folhas profundas
ASCEND "quark" MAGNITUDE 0.0..0.3        -- so raizes abstractas
```

```rust
pub struct MagnitudeRange {
    pub min: f32,  // >= 0.0
    pub max: f32,  // < 1.0 (Poincare ball boundary)
}
```

### CURVATURE (v2.0 — Novo)

Regioes de alta curvatura = clusters densos no Poincare ball.

```aql
RECALL "physics" CURVATURE high    -- nos em regioes densas (clusters)
RESONATE "X" CURVATURE low         -- nos em regioes esparsas (isolados)
```

```rust
pub enum CurvatureDegree {
    High,    // > 20 vizinhos em r < 0.1
    Medium,  // 10-20 vizinhos
    Low,     // 3-10 vizinhos
    Flat,    // < 3 vizinhos (isolado)
}
```

### VALENCE (v2.0 — Novo)

Dimensao emocional do no. Range: -1.0 (negativo) a +1.0 (positivo).

```aql
RECALL "memory" VALENCE positive            -- so memorias positivas
RECALL "experience" VALENCE negative        -- so experiencias negativas
IMPRINT "discovery!" VALENCE 0.9            -- marca como muito positivo
FADE Experience VALENCE negative RECENCY distant  -- esquece memorias negativas antigas
```

### AROUSAL (v2.0 — Novo)

Nivel de activacao. Range: 0.0 (calmo) a 1.0 (excitado).

```aql
RECALL "urgent" AROUSAL high                -- itens de alta activacao
IMPRINT "alarme!" AROUSAL 0.95 VALENCE negative
```

### MOOD (v2.0 — Novo)

Modifica o comportamento do Planner como um todo.

```aql
RECALL "physics" MOOD creative
# → Planner usa NOVELTY high implicito, DEPTH unlimited, RESONATE amplified

RECALL "physics" MOOD analytical
# → Planner usa CONFIDENCE high, DEPTH limited, prefere TRACE a RESONATE

RECALL "physics" MOOD anxious
# → Planner usa RECENCY fresh, CONFIDENCE high, cadeias curtas
```

```rust
impl MoodState {
    pub fn apply_to_planner(&self, config: &mut PlannerConfig) {
        match self {
            Self::Creative => {
                config.default_knn_k *= 2;
                config.default_diffuse_depth += 2;
                config.novelty_bias = 0.8;
            }
            Self::Analytical => {
                config.default_limit = 5;
                config.max_chain_depth = 3;
                config.confidence_floor = 0.7;
            }
            Self::Anxious => {
                config.default_limit = 3;
                config.max_chain_depth = 2;
                config.recency_bias = RecencyDegree::Fresh;
            }
            Self::Focused => {
                config.default_limit = 5;
                config.scope_override = Some(ContextScope::Session);
            }
            Self::Exploratory => {
                config.default_knn_k *= 3;
                config.max_chain_depth = 10;
                config.fade_suppressed = true;
            }
            Self::Conservative => {
                config.confidence_floor = 0.8;
                config.novelty_bias = 0.1;
                config.imprint_restricted = true;
            }
        }
    }
}
```

### EVIDENCE (v2.0 — Novo)

Quantas observacoes suportam uma crenca. Complementa CONFIDENCE.

```aql
IMPRINT "water boils at 100C" CONFIDENCE 0.99 EVIDENCE 10000
IMPRINT "dark matter exists" CONFIDENCE 0.7 EVIDENCE 3
```

```rust
pub struct EvidenceWeight {
    pub count:      u32,
    pub confidence: f32,
    // Combined weight: confidence * log2(evidence + 1)
}

impl EvidenceWeight {
    pub fn combined_weight(&self) -> f32 {
        self.confidence * (self.count as f32 + 1.0).log2()
    }
}
```

---

## 12. Execucao Paralela (AND / FORK / JOIN)

### Sintaxe

```aql
# Duas buscas em paralelo, depois combina
RECALL "quantum" AND RECALL "consciousness"
THEN ASSOCIATE @results[0] LINKING @results[1]

# Tres buscas em paralelo
RECALL "physics" AND RESONATE "beauty" AND REFLECT @self
THEN DISTILL @results WITHIN session

# Paralelo sem join
IMPRINT "A" CONFIDENCE 0.8 AND IMPRINT "B" CONFIDENCE 0.7
```

### Semantica

```rust
pub struct ParallelExecutor {
    max_concurrent: usize,  // default: 8
}

impl ParallelExecutor {
    pub async fn execute_parallel(
        &self,
        branches: &[SimpleStatement],
        backend: &dyn AqlBackend,
        planner: &mut CognitivePlanner,
    ) -> Result<Vec<CognitiveResult>, AqlError> {
        let futures: Vec<_> = branches.iter()
            .map(|stmt| {
                let plan = planner.plan(stmt)?;
                Ok(backend.execute_plan(plan))
            })
            .collect::<Result<Vec<_>, AqlError>>()?;

        // Executa todos em paralelo com limite de concorrencia
        let results = futures::stream::iter(futures)
            .buffer_unordered(self.max_concurrent)
            .collect::<Vec<_>>()
            .await;

        results.into_iter().collect()
    }
}
```

### @results — Referencia a Resultados Paralelos

```rust
pub enum ResultsRef {
    All,                    // @results — todos os resultados combinados
    Index(usize),           // @results[0] — resultado do branch N
    LastDream,              // @last_dream — resultado do ultimo DREAM
    DelegateResult,         // @delegate.result — resultado de DELEGATE
}
```

---

## 13. Condicionais (WHEN / ELSE)

### Sintaxe

```aql
# Condicional simples
RECALL "quantum"
WHEN @results.count > 0 THEN ASSOCIATE @results LINKING "physics"

# Com ELSE
RECALL "quantum"
WHEN @results.count > 0 THEN DISTILL @results
ELSE IMPRINT "quantum" CONFIDENCE 0.5

# Condicional sobre energia
RECALL "X"
WHEN @results.avg_energy > 0.7 THEN DISTILL @results
ELSE FADE @results CONFIDENCE 0.1

# Condicional sobre contagem
RECALL Experience WITHIN session
WHEN @results.count >= 10 THEN DISTILL Experience WITHIN session
```

### Campos condicionais disponeis

| Campo | Tipo | Descricao |
|-------|------|-----------|
| `@results.count` | u32 | Numero de resultados |
| `@results.avg_energy` | f32 | Energia media |
| `@results.max_energy` | f32 | Energia maxima |
| `@results.min_energy` | f32 | Energia minima |
| `@results.avg_confidence` | f32 | Confianca media |
| `@results.avg_valence` | f32 | Valence media |
| `@results.avg_arousal` | f32 | Arousal medio |

### Implementacao

```rust
pub struct ConditionEvaluator;

impl ConditionEvaluator {
    pub fn evaluate(
        when: &WhenClause,
        results: &CognitiveResult,
    ) -> bool {
        let actual = match when.field.as_str() {
            "count"          => results.nodes.len() as f64,
            "avg_energy"     => results.avg_energy() as f64,
            "max_energy"     => results.max_energy() as f64,
            "min_energy"     => results.min_energy() as f64,
            "avg_confidence" => results.avg_confidence() as f64,
            "avg_valence"    => results.avg_valence() as f64,
            "avg_arousal"    => results.avg_arousal() as f64,
            _ => return false,
        };

        let expected = match &when.value {
            ConditionValue::Float(f) => *f,
            ConditionValue::Int(i)   => *i as f64,
            _ => return false,
        };

        match when.op {
            CompOp::Gt  => actual > expected,
            CompOp::Lt  => actual < expected,
            CompOp::Gte => actual >= expected,
            CompOp::Lte => actual <= expected,
            CompOp::Eq  => (actual - expected).abs() < f64::EPSILON,
            CompOp::Neq => (actual - expected).abs() >= f64::EPSILON,
        }
    }
}
```

---

## 14. Reactividade (WATCH / SUBSCRIBE)

### Sintaxe

```aql
# Reagir a mudancas num conceito
WATCH "quantum" ON_CHANGE RECALL "quantum" THEN REFLECT @self

# Reagir a insercoes numa collection
SUBSCRIBE Collection:"memories" ON_INSERT ASSOCIATE @new LINKING @self

# Watch com condicional
WATCH Signal ON_CHANGE
WHEN @results.avg_energy < 0.1 THEN FADE Signal CONFIDENCE 0.05

# Watch termodinamico
WATCH Collection:"memories" ON_CHANGE
WHEN @results.temperature > 0.8 THEN DISTILL Experience WITHIN "memories"
```

### WatchHandle

```rust
pub struct WatchHandle {
    pub id:        String,
    pub subject:   Subject,
    pub trigger:   WatchTrigger,
    pub active:    bool,
}

impl WatchHandle {
    /// Cancela a subscription.
    pub async fn cancel(&mut self) {
        self.active = false;
    }
}
```

### Implementacao NietzscheDB

No NietzscheDB, WATCH integra-se com o Agency Engine tick loop:

```rust
impl NietzscheBackend {
    async fn watch(&self, plan: &WatchPlan) -> Result<WatchHandle, AqlError> {
        // Regista um callback no agency engine
        // que dispara a cada tick quando a condicao e verdadeira.
        let handle = self.agency.register_watch(
            plan.subject.clone(),
            plan.trigger.clone(),
            plan.reaction.clone(),
        ).await?;

        Ok(handle)
    }
}
```

---

## 15. Multi-Agent (SHARE / DELEGATE / NEGOTIATE)

### Sintaxe

```aql
# Partilhar conhecimento com outro agente
SHARE "quantum-discovery" WITH agent:"eva-2" CONFIDENCE 0.8

# Delegar tarefa complexa a especialista
DELEGATE DISTILL Experience TO agent:"specialist"
THEN RECALL @delegate.result

# Negociar crenca conflituosa
NEGOTIATE Belief:"earth-is-round" WITH agent:"skeptic" POLICY weighted_average

# Perspectiva de outro agente
RECALL "quantum" WITH agent:"physicist"
```

### Protocolo Multi-Agent

```rust
/// Mensagem entre agentes no protocolo AQL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub from:       String,           // agent_id do emissor
    pub to:         String,           // agent_id do receptor
    pub verb:       MultiAgentVerb,
    pub payload:    AgentPayload,
    pub confidence: f32,
    pub timestamp:  i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MultiAgentVerb {
    Share,                    // partilha unidireccional
    Delegate(DelegateSpec),   // delegacao com retorno
    Negotiate(NegotiateSpec), // negociacao bidireccional
    Query,                    // consulta a perspectiva do outro
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateSpec {
    pub task:       Statement,        // o que delegar
    pub timeout_ms: u64,              // timeout
    pub fallback:   Option<Statement>,// o que fazer se timeout
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiateSpec {
    pub belief:     String,           // crenca em negociacao
    pub policy:     ConflictPolicy,   // como resolver
    pub max_rounds: u8,               // limite de rounds
}
```

---

## 16. Dimensao Afectiva (Valence / Arousal)

### Modelo Circumplex

O AQL v2.0 adopta o modelo circumplex de Russell para emocoes:

```
         High Arousal
              │
     Tense    │    Excited
              │
  ────────────┼──────────── Valence
              │
     Sad      │    Content
              │
         Low Arousal
```

### Uso no AQL

```aql
# Buscar memorias por estado emocional
RECALL Experience VALENCE positive AROUSAL high    -- memorias excitantes
RECALL Experience VALENCE negative AROUSAL low     -- memorias tristes
RECALL Experience VALENCE positive AROUSAL low     -- memorias serenas

# Imprimir com emocao
IMPRINT "eureka moment!" VALENCE 0.95 AROUSAL 0.9 CONFIDENCE 0.8

# Esquecer selectivamente memorias negativas
FADE Experience VALENCE negative RECENCY distant

# Mood afecta o Planner globalmente
RECALL "physics" MOOD creative
```

### Mapeamento para NietzscheDB

```rust
impl NietzscheBackend {
    fn apply_valence_filter(
        &self,
        nodes: Vec<Node>,
        valence: &ValenceSpec,
    ) -> Vec<Node> {
        nodes.into_iter().filter(|n| {
            let v = n.meta.valence.unwrap_or(0.0);
            match valence {
                ValenceSpec::Positive => v > 0.2,
                ValenceSpec::Negative => v < -0.2,
                ValenceSpec::Neutral  => v.abs() <= 0.2,
                ValenceSpec::Exact(target) => (v - target).abs() < 0.15,
            }
        }).collect()
    }
}
```

---

## 17. DREAM / IMAGINE — Estados Alterados

### DREAM — Integracao com nietzsche-dream

```aql
DREAM ABOUT "quantum consciousness" DEPTH 5
```

**Pipeline interno (NietzscheDB):**

1. Activa dream cycle no `nietzsche-dream` crate
2. Random activation: nos com energy > 0.3 sao activados aleatoriamente
3. Spreading activation: energia propaga por edges (como em sonho real)
4. Consolidacao: edges com activacao alta sao reforçados
5. Pattern emergence: novos Pattern nodes criados de co-activacoes
6. Resultado guardado em `@last_dream`

```rust
pub struct DreamResult {
    pub consolidated_edges:  Vec<EdgeId>,    // edges reforcadas
    pub emergent_patterns:   Vec<NodeId>,    // novos Pattern nodes
    pub activations:         Vec<(NodeId, f32)>, // nos activados + nivel
    pub dream_narrative:     String,         // narrativa gerada
    pub duration_ms:         u64,
}
```

### IMAGINE — Counterfactual Sandbox

```aql
IMAGINE "what if gravity were repulsive" DEPTH 3
```

**Pipeline:**

1. Cria branch temporario (sandbox, nao afecta o grafo real)
2. Encontra nos relacionados a premissa
3. Inverte/modifica as relacoes causais conforme a premissa
4. Propaga consequencias via edges causais ate DEPTH
5. Retorna implicacoes (sem alterar o grafo)

```rust
pub struct ImagineResult {
    pub implications: Vec<Implication>,
    pub contradictions: Vec<Contradiction>,
    pub confidence:   f32,   // quao plausivel e o cenario
}

pub struct Implication {
    pub node:       CognitiveNode,
    pub chain:      Vec<String>,  // caminho causal
    pub confidence: f32,
}

pub struct Contradiction {
    pub existing:    String,  // crenca actual
    pub implied:     String,  // o que o counterfactual implica
    pub conflict_at: String,  // onde o conflito ocorre
}
```

---

## 18. EXPLAIN / WHY — Proveniencia

### Sintaxe

```aql
# Explicar uma query
EXPLAIN RECALL "quantum" CONFIDENCE 0.8

# Explicar um trace
EXPLAIN TRACE FROM "A" TO "B"
```

### Resultado

```rust
pub struct Explanation {
    /// Backend usado.
    pub backend:       String,
    /// Estrategia escolhida pelo Planner.
    pub strategy:      String,
    /// Porque esta estrategia foi escolhida.
    pub reasoning:     String,
    /// Capabilities usadas.
    pub caps_used:     Vec<String>,
    /// Capabilities em falta (degradacao).
    pub caps_missing:  Vec<String>,
    /// Plano de execucao (NAQ ou Cypher ou SQL...).
    pub execution_plan: String,
    /// Metricas de execucao.
    pub metrics:       ExplainMetrics,
    /// Proveniencia dos resultados.
    pub provenance:    Vec<ProvenanceEntry>,
}

pub struct ExplainMetrics {
    pub nodes_scanned:    u64,
    pub nodes_returned:   u64,
    pub edges_traversed:  u64,
    pub time_ms:          u64,
    pub energy_debited:   f32,
}

pub struct ProvenanceEntry {
    pub node_id:    String,
    pub created_by: String,    // agent que criou
    pub created_at: i64,       // timestamp
    pub accessed:   u32,       // vezes acedido
    pub confidence: f32,
    pub evidence:   u32,
}
```

### Exemplo de output

```json
{
  "backend": "NietzscheDB",
  "strategy": "HybridSearch (KNN + Diffusion)",
  "reasoning": "RESONATE with NOVELTY high → HybridSearch to maximize discovery. Backend supports diffusion (has_diffusion=true).",
  "caps_used": ["vector_search", "diffusion", "graph_algos"],
  "caps_missing": [],
  "execution_plan": "K:S?c~\"quantum\"/20 | X:result_ids/0.7/3",
  "metrics": {
    "nodes_scanned": 1240,
    "nodes_returned": 8,
    "edges_traversed": 342,
    "time_ms": 12,
    "energy_debited": 0.10
  },
  "provenance": [
    { "node_id": "abc-123", "created_by": "eva-1", "created_at": 1741871234000, "accessed": 14, "confidence": 0.85, "evidence": 3 }
  ]
}
```

---

## 19. Uncertainty Propagation

### Confidence Nao E Um Ponto — E Uma Distribuicao

```aql
# v1.0: ponto unico
IMPRINT "X" CONFIDENCE 0.6

# v2.0: com intervalo de incerteza
IMPRINT "X" CONFIDENCE 0.6 EVIDENCE 3     -- 3 observacoes → alta incerteza
IMPRINT "X" CONFIDENCE 0.6 EVIDENCE 1000  -- 1000 observacoes → baixa incerteza
```

### Propagacao em Cadeias

```rust
/// Propaga confidence ao longo de uma cadeia THEN.
pub fn propagate_confidence(chain: &[StepResult]) -> f32 {
    // Confidence compound: c1 * c2 * ... * cn
    // Cada step reduz a certeza do resultado final.
    chain.iter()
        .map(|step| step.avg_confidence())
        .fold(1.0_f32, |acc, c| acc * c)
}

/// Combina confidence de resultados paralelos (AND).
pub fn combine_parallel_confidence(results: &[CognitiveResult]) -> f32 {
    // Media ponderada por evidence count.
    let total_evidence: u32 = results.iter()
        .map(|r| r.total_evidence())
        .sum();

    if total_evidence == 0 {
        return results.iter().map(|r| r.avg_confidence()).sum::<f32>()
            / results.len() as f32;
    }

    results.iter()
        .map(|r| r.avg_confidence() * r.total_evidence() as f32)
        .sum::<f32>()
        / total_evidence as f32
}
```

---

## 20. Transaccoes (ATOMIC)

### Sintaxe

```aql
ATOMIC {
    IMPRINT "A" CONFIDENCE 0.8
    ASSOCIATE "A" LINKING "B"
    ASSOCIATE "A" LINKING "C"
}
# Tudo ou nada — se qualquer step falhar, rollback completo.
```

### Implementacao

```rust
pub struct AtomicExecutor;

impl AtomicExecutor {
    pub async fn execute_atomic(
        &self,
        block: &AtomicBlock,
        backend: &dyn AqlBackend,
        planner: &mut CognitivePlanner,
    ) -> Result<Vec<CognitiveResult>, AqlError> {
        if !backend.capabilities().supports_atomic {
            return Err(AqlError::UnsupportedFeature {
                feature: "ATOMIC".into(),
                reason: "backend does not support transactions".into(),
            });
        }

        // Executa com rollback on error
        let mut results = Vec::new();
        let mut rollback_log: Vec<RollbackAction> = Vec::new();

        for stmt in &block.statements {
            match self.execute_one(stmt, backend, planner).await {
                Ok((result, undo)) => {
                    results.push(result);
                    rollback_log.push(undo);
                }
                Err(e) => {
                    // Rollback tudo
                    for action in rollback_log.into_iter().rev() {
                        action.execute(backend).await.ok();
                    }
                    return Err(AqlError::AtomicRollback {
                        failed_step: results.len(),
                        cause: Box::new(e),
                    });
                }
            }
        }

        Ok(results)
    }
}

pub enum RollbackAction {
    DeleteNode(String),
    DeleteEdge(String),
    RestoreEnergy { node_id: String, original: f32 },
}
```

---

## 21. Atencao e Focus

### Attention Budget

O agente tem um budget de atencao que e consumido a cada operacao. Complementa o Energy Model.

```rust
pub struct AttentionModel {
    pub budget:       f32,           // 0.0 a 1.0
    pub focus:        Option<ContextScope>,  // onde esta focado
    pub salience_map: HashMap<String, f32>,  // nos com alta saliencia
}

impl AttentionModel {
    pub fn cost_for_verb(&self, verb: &Verb) -> f32 {
        match verb {
            Verb::Recall    => 0.02,
            Verb::Resonate  => 0.08,
            Verb::Reflect   => 0.03,
            Verb::Trace     => 0.06,
            Verb::Imprint   => 0.05,
            Verb::Associate => 0.04,
            Verb::Distill   => 0.15,
            Verb::Fade      => 0.01,
            Verb::Descend   => 0.04,
            Verb::Ascend    => 0.04,
            Verb::Orbit     => 0.05,
            Verb::Dream     => 0.20,
            Verb::Imagine   => 0.18,
        }
    }

    pub fn apply_focus_bias(&self, results: &mut Vec<CognitiveNode>) {
        if let Some(scope) = &self.focus {
            // Boost nodes que estao no scope focado
            for node in results.iter_mut() {
                if self.node_in_scope(node, scope) {
                    node.relevance_score *= 1.5;
                }
            }
            results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        }
    }
}
```

---

## 22. Cognitive Planner v2

### Planner Backend-Aware

```rust
pub struct CognitivePlanner {
    session_id:      String,
    session_history: Vec<SessionEntry>,
    config:          PlannerConfig,
    self_resolver:   SelfResolver,
    energy_hooks:    EnergyHooks,
    attention:       AttentionModel,
    capabilities:    BackendCapabilities,  // v2: backend-aware
}

impl CognitivePlanner {
    pub fn plan(&mut self, stmt: &SimpleStatement) -> Result<ExecutionPlan, AqlError> {
        // 1. Verificar energia + atencao
        self.energy_hooks.check_can_execute(&stmt.verb)?;
        self.attention.check_budget(&stmt.verb)?;

        // 2. Aplicar MOOD se presente
        if let Some(mood) = stmt.mood() {
            mood.apply_to_planner(&mut self.config);
        }

        // 3. Escolher estrategia (backend-aware)
        let strategy = self.choose_strategy(stmt);

        // 4. Construir instrucoes
        let instructions = self.lower(stmt, &strategy)?;

        // 5. Side-effects
        let side_effects = stmt.verb.implicit_side_effects();

        Ok(ExecutionPlan { instructions, side_effects, strategy })
    }

    fn choose_strategy(&self, stmt: &SimpleStatement) -> Strategy {
        match stmt.verb {
            // v1.0 verbs (agora backend-aware)
            Verb::Resonate => {
                if self.capabilities.has_diffusion && self.capabilities.has_graph_algos {
                    Strategy::HybridSearch
                } else if self.capabilities.has_vector_search {
                    Strategy::VectorSearch
                } else {
                    Strategy::TextSearch
                }
            }
            Verb::Distill => {
                if self.capabilities.has_graph_algos {
                    Strategy::PatternExtraction
                } else {
                    Strategy::ClientSideClustering
                }
            }
            Verb::Trace => {
                if self.capabilities.has_traversal {
                    Strategy::CausalTrace
                } else {
                    Strategy::ClientSideBFS
                }
            }
            Verb::Reflect => match &stmt.subject {
                Subject::SelfRef => Strategy::SessionIntrospection,
                _ if self.capabilities.has_graph_algos => Strategy::PageRank,
                _ => Strategy::BasicStats,
            },

            // v2.0 geometric verbs
            Verb::Descend | Verb::Ascend | Verb::Orbit => {
                match self.capabilities.geometry {
                    Geometry::Hyperbolic { .. } => Strategy::HyperbolicNavigation,
                    _ => Strategy::MetadataDepthFilter,
                }
            }

            // v2.0 altered states
            Verb::Dream => {
                if self.capabilities.has_dream {
                    Strategy::DreamCycle
                } else {
                    Strategy::RandomActivation
                }
            }
            Verb::Imagine => Strategy::CounterfactualSandbox,

            // v1.0 unchanged
            Verb::Recall | Verb::Associate => self.strategy_recall(stmt),
            Verb::Imprint  => Strategy::CreateWithAssociation,
            Verb::Fade     => Strategy::EnergyDecrement,
        }
    }
}
```

---

## 23. Lowering AQL → Backend

### NietzscheDB (via NAQ)

```
RECALL "quantum" CONFIDENCE 0.8
→ K:S?c~"quantum"&e>0.4/10

RESONATE "consciousness" NOVELTY high DEPTH 3
→ K:S?c~"consciousness"/20 | X:result_ids/0.7/3

DESCEND "physics" DEPTH 3
→ K:S?c~"physics"/60 | filter(mag > source_mag) | limit(20)

ASCEND "quark" DEPTH 2
→ K:S?c~"quark"/40 | filter(mag < source_mag) | limit(20)

ORBIT "consciousness" RADIUS 0.1
→ K:S?c~"consciousness"/100 | filter(|mag - source_mag| < 0.1) | limit(20)

DREAM ABOUT "quantum" DEPTH 5
→ dream:trigger{seed:"quantum",depth:5}

IMPRINT "X" CONFIDENCE 0.6 VALENCE 0.9 AROUSAL 0.8
→ C:S{c:"X",e:0.6,v:0.9,a:0.8}
```

### Neo4j (via Cypher)

```
RECALL "quantum" CONFIDENCE 0.8
→ CALL db.index.vector.queryNodes('embedding_idx', 10, $embedding)
  YIELD node WHERE node.energy > 0.4

TRACE FROM "A" TO "B" DEPTH 5
→ MATCH path = shortestPath((a)-[*..5]->(b))
  WHERE a.content CONTAINS 'A' AND b.content CONTAINS 'B'
  RETURN path

DISTILL Experience WITHIN session
→ CALL gds.louvain.stream('session_graph')
  YIELD nodeId, communityId
```

### Qdrant (via REST)

```
RECALL "quantum" CONFIDENCE 0.8
→ POST /collections/{name}/points/search
  { "vector": [...], "limit": 10, "filter": { "must": [
    { "key": "energy", "range": { "gte": 0.4 } }
  ]}}

IMPRINT "X" CONFIDENCE 0.6
→ PUT /collections/{name}/points
  { "points": [{ "id": uuid, "vector": [...],
    "payload": { "content": "X", "energy": 0.6 } }]}
```

### pgvector (via SQL)

```
RECALL "quantum" CONFIDENCE 0.8
→ SELECT * FROM nodes
  WHERE energy > 0.4
  ORDER BY embedding <=> $embedding
  LIMIT 10;

ASSOCIATE "A" LINKING "B" CONFIDENCE 0.9
→ INSERT INTO edges (from_id, to_id, type, weight)
  VALUES ($a_id, $b_id, 'Association', 0.9)
  ON CONFLICT (from_id, to_id, type) DO UPDATE
  SET weight = edges.weight + 0.1;
```

---

## 24. State Management — WorkingMemory

```rust
pub struct WorkingMemory {
    pub result_node_ids:    Vec<String>,
    pub result_contents:    Vec<String>,
    pub avg_energy:         f32,
    pub avg_valence:        f32,       // v2.0
    pub avg_arousal:        f32,       // v2.0
    pub created_node_id:    Option<String>,
    pub session_id:         String,
    pub chain_depth:        u8,
    pub parallel_results:   Vec<CognitiveResult>,  // v2.0: resultados AND
    pub last_dream:         Option<DreamResult>,    // v2.0: ultimo DREAM
    pub delegate_result:    Option<CognitiveResult>,// v2.0: resultado DELEGATE
}

impl WorkingMemory {
    pub fn resolve_ref(&self, r: &ResultsRef) -> Vec<String> {
        match r {
            ResultsRef::All => self.result_node_ids.clone(),
            ResultsRef::Index(i) => {
                self.parallel_results.get(*i)
                    .map(|r| r.node_ids())
                    .unwrap_or_default()
            }
            ResultsRef::LastDream => {
                self.last_dream.as_ref()
                    .map(|d| d.emergent_patterns.clone())
                    .unwrap_or_default()
            }
            ResultsRef::DelegateResult => {
                self.delegate_result.as_ref()
                    .map(|r| r.node_ids())
                    .unwrap_or_default()
            }
        }
    }
}
```

---

## 25. Resolucao de @self

Mantido do v1.0 com adicao de multi-agent context:

```rust
pub struct SelfResolver {
    session_id:        String,
    agent_id:          String,
    self_collection:   String,
    known_agents:      HashMap<String, AgentInfo>,  // v2.0
}

pub struct AgentInfo {
    pub agent_id:    String,
    pub endpoint:    String,   // como contactar
    pub capabilities: Vec<String>, // o que sabe fazer
    pub trust_level: f32,      // 0.0 a 1.0
}
```

---

## 26. Logica de Upsert Epistemico (IMPRINT)

Mantido do v1.0. 4 politicas de conflito:

| Politica | Comportamento |
|----------|---------------|
| `KeepHigherConfidence` | So actualiza se nova confidence > existente |
| `WeightedAverage { weight }` | Media ponderada: `old*(1-w) + new*w` |
| `ReplaceAlways` | Mais recente ganha sempre |
| `CreateConflict` | Cria no de conflito para resolucao futura |

Politica default por tipo:

| Tipo | Politica | Razao |
|------|----------|-------|
| Belief | `WeightedAverage { 0.3 }` | Integra gradualmente |
| Experience | `ReplaceAlways` | Factos nao se contestam |
| Pattern | `WeightedAverage { 0.1 }` | Mudam devagar |
| Signal | `ReplaceAlways` | Recente substitui |
| Intention | `KeepHigherConfidence` | Deliberadas |

---

## 27. Cognitive Energy Model

Mantido do v1.0 com novos verbos:

```rust
impl Default for EnergyConfig {
    fn default() -> Self {
        let mut costs = HashMap::new();
        // v1.0
        costs.insert(Verb::Recall,    0.02);
        costs.insert(Verb::Resonate,  0.10);
        costs.insert(Verb::Reflect,   0.05);
        costs.insert(Verb::Trace,     0.08);
        costs.insert(Verb::Imprint,   0.15);
        costs.insert(Verb::Associate, 0.12);
        costs.insert(Verb::Distill,   0.25);
        costs.insert(Verb::Fade,      0.03);
        // v2.0
        costs.insert(Verb::Descend,   0.04);
        costs.insert(Verb::Ascend,    0.04);
        costs.insert(Verb::Orbit,     0.06);
        costs.insert(Verb::Dream,     0.30);  // mais caro — produccao criativa
        costs.insert(Verb::Imagine,   0.25);  // caro — counterfactual reasoning

        Self { costs, min_free_energy: 0.1 }
    }
}
```

### Interaccao com estados termodinamicos

```
Estado solido (free_energy < 0.2):
  → DREAM, IMAGINE, DISTILL bloqueados
  → RESONATE com DEPTH > 1 bloqueado
  → RECALL, FADE, ASCEND permitidos

Estado liquido (free_energy 0.2..0.7):
  → Todos os verbos permitidos
  → DREAM com DEPTH max 3
  → DISTILL com DEPTH max 3

Estado gasoso (free_energy > 0.7):
  → Sem restriccoes
  → DREAM produz resultados mais amplos
  → IMAGINE com DEPTH irrestrito
```

---

## 28. Executor e Pipeline

```rust
pub struct AqlExecutor<B: AqlBackend> {
    backend:      B,
    planner:      CognitivePlanner,
    parallel_exec: ParallelExecutor,
    atomic_exec:   AtomicExecutor,
    condition_eval: ConditionEvaluator,
}

impl<B: AqlBackend> AqlExecutor<B> {
    pub async fn execute(
        &mut self,
        program: Program,
    ) -> Result<Vec<CognitiveResult>, AqlError> {
        let mut results = Vec::new();

        for stmt in program.statements {
            let result = match stmt {
                Statement::Simple(s) =>
                    self.execute_simple(&s).await?,
                Statement::Chain(c) =>
                    self.execute_chain(&c).await?,
                Statement::Parallel(p) =>
                    self.execute_parallel(&p).await?,
                Statement::Atomic(a) =>
                    self.atomic_exec.execute_atomic(
                        &a, &self.backend, &mut self.planner
                    ).await?.pop().unwrap(),
                Statement::Watch(w) => {
                    let handle = self.backend.watch(&w.into()).await?;
                    CognitiveResult::watch_registered(handle)
                }
                Statement::Explain(e) => {
                    let explanation = self.backend.explain(&e.into()).await?;
                    CognitiveResult::explanation(explanation)
                }
            };
            results.push(result);
        }

        Ok(results)
    }
}
```

---

## 29. Erros e Protocolo de Resposta

```rust
#[derive(Debug, thiserror::Error)]
pub enum AqlError {
    // v1.0
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("unknown verb: '{0}'")]
    UnknownVerb(String),

    #[error("unknown epistemic type: '{0}'")]
    UnknownType(String),

    #[error("invalid confidence {0}: must be [0.0, 1.0]")]
    InvalidConfidence(f32),

    #[error("insufficient cognitive energy: need {required:.2}, have {available:.2} ({verb})")]
    InsufficientCognitiveEnergy { required: f32, available: f32, verb: String },

    #[error("THEN chain exceeded max depth {0}")]
    ChainDepthExceeded(u8),

    // v2.0
    #[error("verb '{verb}' not supported: {reason}")]
    UnsupportedVerb { verb: String, reason: String },

    #[error("feature '{feature}' not supported: {reason}")]
    UnsupportedFeature { feature: String, reason: String },

    #[error("ATOMIC rollback at step {failed_step}: {cause}")]
    AtomicRollback { failed_step: usize, cause: Box<AqlError> },

    #[error("insufficient attention budget: need {required:.2}, have {available:.2}")]
    InsufficientAttention { required: f32, available: f32 },

    #[error("parallel execution failed in branch {branch}: {cause}")]
    ParallelError { branch: usize, cause: Box<AqlError> },

    #[error("agent '{agent_id}' unreachable: {reason}")]
    AgentUnreachable { agent_id: String, reason: String },

    #[error("DELEGATE timeout after {timeout_ms}ms")]
    DelegateTimeout { timeout_ms: u64 },

    #[error("NEGOTIATE failed after {rounds} rounds")]
    NegotiateFailed { rounds: u8 },

    #[error("backend error: {0}")]
    BackendError(String),
}
```

---

## 30. SDKs Multi-Linguagem

### Python (PyO3)

```python
from aql import AqlClient, NietzscheBackend, QdrantBackend

# NietzscheDB — experiencia completa
db = AqlClient(NietzscheBackend("136.111.0.47:443"))
result = db.execute('RECALL "quantum physics" CONFIDENCE 0.8')

for node in result.nodes:
    print(f"{node.content} (energy={node.energy}, valence={node.valence})")

# Geometria hiperbolica (exclusivo NietzscheDB)
children = db.execute('DESCEND "physics" DEPTH 3')
parents  = db.execute('ASCEND "quark" DEPTH 2')
peers    = db.execute('ORBIT "consciousness" RADIUS 0.1')

# Dream (exclusivo NietzscheDB)
dream = db.execute('DREAM ABOUT "quantum" DEPTH 5')
insights = db.execute('RECALL @last_dream')

# Mesmo codigo, backend diferente (degradacao graceful)
db2 = AqlClient(QdrantBackend("localhost:6333", collection="knowledge"))
result = db2.execute('RECALL "quantum physics" CONFIDENCE 0.8')  # funciona!
# db2.execute('DESCEND "physics"')  → AqlError: UnsupportedVerb
```

### TypeScript / JavaScript

```typescript
import { AqlClient, NietzscheBackend, PineconeBackend } from '@aql/core';

const ndb = new AqlClient(new NietzscheBackend({ host: '136.111.0.47:443' }));

const result = await ndb.execute(`
  RECALL "machine learning" CONFIDENCE 0.7
  THEN IMPRINT "ML is subset of AI" CONFIDENCE 0.9 LINKING "artificial intelligence"
`);

// Paralelo
const parallel = await ndb.execute(`
  RECALL "quantum" AND RECALL "consciousness"
  THEN ASSOCIATE @results[0] LINKING @results[1]
`);

// WASM (browser)
import { AqlClient } from '@aql/wasm';
const client = await AqlClient.init(wasmUrl, backendConfig);
```

### Go

```go
import "github.com/JoseRFJuniorLLMs/aql-go"

client := aql.NewClient(aql.NietzscheBackend("136.111.0.47:443"))
result, err := client.Execute(`RECALL "quantum" CONFIDENCE 0.8`)
```

---

## 31. Mapeamento Por Backend

| Verbo | NietzscheDB | Neo4j | Qdrant | Pinecone | pgvector | Redis |
|-------|------------|-------|--------|----------|----------|-------|
| RECALL | KNN Poincare + energy | Cypher + vector idx | Search points | Query | SQL + pgvec | FT.SEARCH |
| RESONATE | KNN + Diffusion + graph | Cypher + GDS | Search + rerank | Query (basic) | SQL | FT.SEARCH |
| REFLECT | PageRank + session | GDS PageRank | Collection info | Index stats | SQL COUNT | INFO |
| TRACE | Dijkstra causal | shortestPath() | ✗ client BFS | ✗ | SQL recursive CTE | ✗ |
| IMPRINT | InsertNode + energy | CREATE (n) | Upsert point | Upsert vector | INSERT | JSON.SET |
| ASSOCIATE | InsertEdge + weight | CREATE -[:REL]-> | ✗ payload link | ✗ metadata | INSERT edges | ✗ |
| DISTILL | Louvain + Pattern | GDS Louvain | ✗ client cluster | ✗ | ✗ client | ✗ |
| FADE | energy-- + prune | SET + DELETE | Delete points | Delete vectors | UPDATE/DELETE | DEL |
| **DESCEND** | ★ magnitude nativo | ✗ | ✗ | ✗ | ✗ | ✗ |
| **ASCEND** | ★ magnitude nativo | ✗ | ✗ | ✗ | ✗ | ✗ |
| **ORBIT** | ★ magnitude nativo | ✗ | ✗ | ✗ | ✗ | ✗ |
| **DREAM** | ★ dream cycle | ✗ | ✗ | ✗ | ✗ | ✗ |
| **IMAGINE** | ★ sandbox | ✗ partial | ✗ | ✗ | ✗ | ✗ |
| **WATCH** | ★ agency tick | ✗ | ✗ | ✗ | LISTEN/NOTIFY | Keyspace |
| **ATOMIC** | ★ sim | ★ ACID | ✗ | ✗ | ★ ACID | MULTI |
| **Valence** | ★ nativo | ✗ property | ✗ payload | ✗ metadata | ✗ column | ✗ field |
| **Arousal** | ★ nativo | ✗ property | ✗ payload | ✗ metadata | ✗ column | ✗ field |

**★ = Suporte nativo / completo**
**✗ = Nao suportado ou fallback client-side**

---

## 32. Exemplos Completos v2

### Exemplo 1 — Investigacao cognitiva com paralelo e condicionais

```aql
# Busca paralela + decisao condicional + dream
RECALL "quantum" AND RESONATE "consciousness emerges from complexity"
WHEN @results.count > 5 THEN DISTILL @results WITHIN session
ELSE DREAM ABOUT "quantum consciousness" DEPTH 3
THEN IMPRINT "synthesis" CONFIDENCE 0.7 LINKING @results AS Pattern
THEN REFLECT @self WITHIN session
```

### Exemplo 2 — Navegacao hiperbolica

```aql
# Explorar hierarquia do conhecimento
DESCEND "Science" DEPTH 2 MAGNITUDE 0.2..0.5
THEN ORBIT @results[0] RADIUS 0.1 NOVELTY high
THEN ASCEND @results DEPTH 3
```

### Exemplo 3 — Multi-agent com negociacao

```aql
# Dois agentes discutem uma crenca
IMPRINT "consciousness is computable" CONFIDENCE 0.6 AS Belief
NEGOTIATE Belief:"consciousness is computable" WITH agent:"philosopher" POLICY weighted_average
THEN REFLECT @self
```

### Exemplo 4 — Transaccao atomica com emocao

```aql
ATOMIC {
    IMPRINT "eureka!" VALENCE 0.95 AROUSAL 0.9 CONFIDENCE 0.8 AS Experience
    ASSOCIATE "eureka!" LINKING "quantum" CONFIDENCE 0.9
    ASSOCIATE "eureka!" LINKING "consciousness" CONFIDENCE 0.7
    ASSOCIATE "eureka!" LINKING @self
}
```

### Exemplo 5 — Watch reactivo com MOOD

```aql
WATCH Collection:"memories" ON_CHANGE
WHEN @results.avg_energy < 0.2 THEN FADE Signal RECENCY ancient
WATCH Signal ON_INSERT RECALL @new MOOD analytical THEN REFLECT @self
```

### Exemplo 6 — Counterfactual com IMAGINE

```aql
IMAGINE "what if Newton never existed" DEPTH 5
THEN TRACE FROM "classical mechanics" TO "modern physics"
THEN EXPLAIN TRACE FROM "classical mechanics" TO "modern physics"
```

### Exemplo 7 — EXPLAIN completo

```aql
EXPLAIN RESONATE "consciousness" NOVELTY high DEPTH 3 MOOD creative
# Retorna: estrategia, metricas, proveniencia, caps usadas/missing
```

---

## 33. Testes de Aceitacao

```rust
// ─── Parser v2 ──────────────────────────────────────────────

#[test]
fn parses_all_13_verbs() {
    for verb in ["RECALL","RESONATE","REFLECT","TRACE",
                 "IMPRINT","ASSOCIATE","DISTILL","FADE",
                 "DESCEND","ASCEND","ORBIT","DREAM","IMAGINE"] {
        let input = format!("{} \"test\"\n", verb);
        let prog = parse_aql(&input).unwrap();
        assert_eq!(prog.statements.len(), 1);
    }
}

#[test]
fn parses_parallel_and() {
    let q = "RECALL \"A\" AND RECALL \"B\" AND RECALL \"C\"\n";
    let prog = parse_aql(q).unwrap();
    match &prog.statements[0] {
        Statement::Parallel(p) => assert_eq!(p.branches.len(), 3),
        _ => panic!("expected Parallel"),
    }
}

#[test]
fn parses_when_else() {
    let q = "RECALL \"A\"\nWHEN @results.count > 0 THEN DISTILL @results\nELSE IMPRINT \"A\"\n";
    let stmt = parse_one(q);
    assert!(stmt.condition.is_some());
    assert!(stmt.else_stmt.is_some());
}

#[test]
fn parses_atomic_block() {
    let q = "ATOMIC {\n  IMPRINT \"A\"\n  IMPRINT \"B\"\n}\n";
    let prog = parse_aql(q).unwrap();
    match &prog.statements[0] {
        Statement::Atomic(a) => assert_eq!(a.statements.len(), 2),
        _ => panic!("expected Atomic"),
    }
}

#[test]
fn parses_watch() {
    let q = "WATCH \"quantum\" ON_CHANGE RECALL \"quantum\"\n";
    let prog = parse_aql(q).unwrap();
    assert!(matches!(&prog.statements[0], Statement::Watch(_)));
}

#[test]
fn parses_magnitude_range() {
    let s = parse_one("DESCEND \"physics\" MAGNITUDE 0.2..0.5\n");
    assert!(s.qualifiers.iter().any(|q| matches!(q, Qualifier::Magnitude(_, _))));
}

#[test]
fn parses_valence_arousal() {
    let s = parse_one("RECALL \"X\" VALENCE positive AROUSAL high\n");
    assert!(s.qualifiers.iter().any(|q| matches!(q, Qualifier::Valence(_))));
    assert!(s.qualifiers.iter().any(|q| matches!(q, Qualifier::Arousal(_))));
}

#[test]
fn parses_mood() {
    let s = parse_one("RECALL \"X\" MOOD creative\n");
    assert!(s.qualifiers.iter().any(|q| matches!(q, Qualifier::Mood(MoodState::Creative))));
}

#[test]
fn parses_explain() {
    let q = "EXPLAIN RECALL \"quantum\" CONFIDENCE 0.8\n";
    let prog = parse_aql(q).unwrap();
    assert!(matches!(&prog.statements[0], Statement::Explain(_)));
}

#[test]
fn parses_multi_agent() {
    let s = parse_one("SHARE \"discovery\" WITH agent:\"eva-2\" CONFIDENCE 0.8\n");
    assert!(s.qualifiers.iter().any(|q| matches!(q, Qualifier::WithAgent(_))));
}

// ─── Planner v2 ─────────────────────────────────────────────

#[test]
fn descend_uses_hyperbolic_on_nietzsche() {
    let mut planner = test_planner(BackendCapabilities::nietzschedb());
    let plan = planner.plan(&parse_one("DESCEND \"physics\" DEPTH 3\n")).unwrap();
    assert_eq!(plan.strategy, Strategy::HyperbolicNavigation);
}

#[test]
fn descend_uses_metadata_on_qdrant() {
    let mut planner = test_planner(BackendCapabilities::qdrant());
    let plan = planner.plan(&parse_one("DESCEND \"physics\" DEPTH 3\n")).unwrap();
    assert_eq!(plan.strategy, Strategy::MetadataDepthFilter);
}

#[test]
fn dream_blocked_on_qdrant() {
    let mut planner = test_planner(BackendCapabilities::qdrant());
    let result = planner.plan(&parse_one("DREAM ABOUT \"quantum\"\n"));
    assert!(matches!(result, Err(AqlError::UnsupportedVerb { .. })));
}

#[test]
fn mood_creative_amplifies_knn_k() {
    let mut planner = test_planner(BackendCapabilities::nietzschedb());
    let default_k = planner.config.default_knn_k;
    planner.plan(&parse_one("RECALL \"X\" MOOD creative\n")).unwrap();
    assert!(planner.config.default_knn_k > default_k);
}

// ─── Energy + Attention ─────────────────────────────────────

#[test]
fn dream_blocked_when_insufficient_energy() {
    let mut hooks = EnergyHooks::with_energy(0.05);
    let result = hooks.check_can_execute(&Verb::Dream);
    assert!(matches!(result, Err(AqlError::InsufficientCognitiveEnergy { .. })));
}

#[test]
fn parallel_costs_sum_of_branches() {
    let mut hooks = EnergyHooks::with_energy(0.10);
    // RECALL (0.02) + RECALL (0.02) = 0.04 — should succeed
    assert!(hooks.check_can_execute(&Verb::Recall).is_ok());
}

// ─── Conditional ────────────────────────────────────────────

#[test]
fn when_count_gt_0_true_for_nonempty() {
    let result = CognitiveResult::with_nodes(3);
    let when = WhenClause { field: "count".into(), op: CompOp::Gt, value: ConditionValue::Int(0) };
    assert!(ConditionEvaluator::evaluate(&when, &result));
}

#[test]
fn when_count_gt_0_false_for_empty() {
    let result = CognitiveResult::empty();
    let when = WhenClause { field: "count".into(), op: CompOp::Gt, value: ConditionValue::Int(0) };
    assert!(!ConditionEvaluator::evaluate(&when, &result));
}
```

---

## 34. Roadmap de Implementacao

### Fase 1 — Core Parser + AST (semanas 1-3)

- [ ] `grammar.pest` com 13 verbos, todos os qualifiers, AND, WHEN/ELSE, ATOMIC
- [ ] Parser pest → AST v2.0
- [ ] Todos os tipos: `EpistemicType`, `RecencyDegree`, `MoodState`, `ValenceSpec`, etc.
- [ ] `BackendCapabilities` struct
- [ ] `AqlBackend` trait com defaults
- [ ] Testes de parser para todos os casos

### Fase 2 — Planner Backend-Aware (semanas 4-6)

- [ ] `CognitivePlanner` v2 com seleccao de estrategia por capabilities
- [ ] Lowering para NietzscheDB (NAQ)
- [ ] Lowering para Neo4j (Cypher)
- [ ] Lowering para Qdrant (REST)
- [ ] `SelfResolver` com multi-agent context
- [ ] `MoodState::apply_to_planner()`
- [ ] `ImprintPlanner` com todas as politicas

### Fase 3 — Runtime: Paralelo + Condicionais + Atomic (semanas 7-9)

- [ ] `ParallelExecutor` com AND/FORK/JOIN
- [ ] `ConditionEvaluator` para WHEN/ELSE
- [ ] `AtomicExecutor` com rollback
- [ ] `WorkingMemory` v2 com `@results[N]`, `@last_dream`, `@delegate.result`
- [ ] `AqlExecutor<B: AqlBackend>` generico

### Fase 4 — Geometria + Dream + Affect (semanas 10-12)

- [ ] DESCEND/ASCEND/ORBIT implementacao NietzscheDB
- [ ] DESCEND/ASCEND fallback metadata para Qdrant/pgvector
- [ ] DREAM integracao com `nietzsche-dream`
- [ ] IMAGINE sandbox counterfactual
- [ ] Valence/Arousal filtros e MOOD state

### Fase 5 — EXPLAIN + Multi-Agent + Reactive (semanas 13-15)

- [ ] `Explanation` struct e provenance tracking
- [ ] `EXPLAIN` para NietzscheDB e Neo4j
- [ ] Multi-agent protocol: SHARE, DELEGATE, NEGOTIATE
- [ ] WATCH/SUBSCRIBE integracao com agency tick
- [ ] Uncertainty propagation em cadeias

### Fase 6 — SDKs + WASM (semanas 16-18)

- [ ] Python SDK (PyO3) com pip package
- [ ] TypeScript SDK com npm package
- [ ] Go SDK
- [ ] WASM build para browser
- [ ] CLI REPL (`aql-cli`)

### Fase 7 — Backends adicionais (semanas 19-22)

- [ ] `aql-pgvector` (PostgreSQL)
- [ ] `aql-redis` (Redis Stack)
- [ ] `aql-pinecone`
- [ ] Documentacao e exemplos por backend

### Fase 8 — Testes de aceitacao + Release (semanas 23-24)

- [ ] Todos os testes da Seccao 33 a verde
- [ ] Round-trip: AQL → plan → execute → resultado esperado (por backend)
- [ ] Benchmarks de performance: NietzscheDB vs Neo4j vs Qdrant
- [ ] Publicacao: crates.io, PyPI, npm
- [ ] Documentacao publica

---

## Comparacao: AQL v1.0 vs v2.0

| Feature | v1.0 | v2.0 |
|---------|------|------|
| Verbos | 8 | **13** (+DESCEND, ASCEND, ORBIT, DREAM, IMAGINE) |
| Qualifiers | 8 | **15** (+MAGNITUDE, CURVATURE, RADIUS, VALENCE, AROUSAL, MOOD, EVIDENCE) |
| Execucao | Sequential (THEN) | **Parallel (AND) + Sequential (THEN)** |
| Condicionais | Nenhum | **WHEN/ELSE** |
| Atomicidade | Nenhuma | **ATOMIC blocks** |
| Reactivity | Nenhuma | **WATCH/SUBSCRIBE** |
| Multi-agent | Nenhum | **SHARE/DELEGATE/NEGOTIATE** |
| Explicacao | Nenhuma | **EXPLAIN** |
| Emocao | Nenhuma | **Valence/Arousal/Mood** |
| Backend | NietzscheDB only | **Qualquer (trait)** |
| SDKs | Rust only | **Rust, Python, TypeScript, Go, WASM** |
| Estrategia | Fixed | **Backend-adaptive (capabilities)** |

---

*AQL v2.0 — Agent Cognition Language — Jose R F Junior — 2026 — AGPL-3.0*
*O SQL dos agentes cognitivos.*
*github.com/JoseRFJuniorLLMs/AQL*
