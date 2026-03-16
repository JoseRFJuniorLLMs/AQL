# AQL — Agent Query Language
## The SQL of Cognitive Agents

**Author:** Jose R F Junior  
**Version:** 2.0.0 — 2026  
**License:** AGPL-3.0  
**Repository:** github.com/JoseRFJuniorLLMs/NietzscheDB

---

> *"NQL speaks to those who read. NAQ speaks to those who calculate. AQL speaks to those who think."*

---

## Abstract

This article introduces **AQL (Agent Query Language)**, a backend-agnostic cognitive intent language for AI agents interacting with knowledge graphs, vector databases, and relational stores. AQL defines 13 cognitive verbs, a 5-type epistemic type system, native uncertainty propagation, affective dimensions (valence and arousal), and native support for hyperbolic geometry via the Poincaré ball model.

Unlike existing query languages designed for human operators, AQL models how agents *use* memory, not merely how they query it. Every operation carries implicit side-effects: energy boosts, temporal edges, session context. The agent declares intent; the system handles consequences.

AQL is implemented as a Rust workspace targeting NietzscheDB natively, with graceful degradation to Neo4j, Qdrant, pgvector, Redis, MySQL, SQLite, and SQL Server.

---

## Table of Contents

1. [The Problem](#1-the-problem)
2. [What AQL Is](#2-what-aql-is)
3. [Position in the Stack](#3-position-in-the-stack)
4. [The 13 Cognitive Verbs](#4-the-13-cognitive-verbs)
5. [Epistemic Type System](#5-epistemic-type-system)
6. [Full Grammar Specification](#6-full-grammar-specification)
7. [Complete AST](#7-complete-ast)
8. [Qualifiers Reference](#8-qualifiers-reference)
9. [Cognitive Planner](#9-cognitive-planner)
10. [Lowering AQL to NAQ — Complete Mapping](#10-lowering-aql-to-naq--complete-mapping)
11. [Working Memory and THEN Chains](#11-working-memory-and-then-chains)
12. [Self Resolution (@self)](#12-self-resolution-self)
13. [Epistemic Upsert Logic (IMPRINT)](#13-epistemic-upsert-logic-imprint)
14. [Cognitive Energy Model](#14-cognitive-energy-model)
15. [Uncertainty Propagation](#15-uncertainty-propagation)
16. [Affective Computing: Valence and Arousal](#16-affective-computing-valence-and-arousal)
17. [Hyperbolic Geometry Verbs](#17-hyperbolic-geometry-verbs)
18. [Parallel Execution and Control Flow](#18-parallel-execution-and-control-flow)
19. [Multi-Agent Coordination](#19-multi-agent-coordination)
20. [Executor Pipeline](#20-executor-pipeline)
21. [Error Protocol](#21-error-protocol)
22. [Backend Capabilities Matrix](#22-backend-capabilities-matrix)
23. [Crate Architecture](#23-crate-architecture)
24. [Acceptance Tests](#24-acceptance-tests)
25. [Real-World Examples](#25-real-world-examples)
26. [Comparison with Existing Approaches](#26-comparison-with-existing-approaches)
27. [Implementation Roadmap](#27-implementation-roadmap)
28. [Conclusion](#28-conclusion)

---

## 1. The Problem

### 1.1 The Gap Between Agents and Databases

The AI revolution brought autonomous agents, multi-modal reasoning, and large language models capable of complex cognition. Yet a critical gap persists: **how does an agent interact with its own memory?**

Today's landscape is fragmented:

- **SQL (1970)** — designed for tabular data. Models rows and columns, not beliefs, intentions, or confidence levels.
- **Cypher (Neo4j)** — models graph relationships well, but has no concept of epistemic confidence, cognitive energy, or hyperbolic geometry.
- **Vector APIs (Qdrant, Pinecone, pgvector)** — find similar embeddings but understand nothing about what knowledge *means* to the agent or how it decays over time.
- **Key-value stores (Redis)** — offer speed with zero cognitive structure.

The result: every AI project writes ad-hoc translation layers. Every team reinvents the same wheel. There is no standard way for an agent to express: *"I believe this with 80% confidence"*, *"recall what I learned yesterday, but only what feels relevant"*, or *"find what resonates with this idea — decide how."*

### 1.2 The Three Missing Primitives

Every existing query language is missing three fundamental primitives that cognitive agents require:

**Primitive 1 — Intent as Unit:**  
The atomic unit of interaction should be a cognitive act, not a database instruction. `RECALL "quantum physics"` is not a keyword search. It is a retrieval act with epistemic context, energy dynamics, and automatic side-effects. The agent declares *what it is trying to think*; the system handles *how to execute it*.

**Primitive 2 — Uncertainty as Data Type:**  
`CONFIDENCE 0.7` is not a filter threshold. It is an epistemic statement about the agent's degree of belief. It participates in Bayesian propagation through operation chains, evidence weighting, and attention budgeting. No existing language treats uncertainty as first-class.

**Primitive 3 — Effects as Automatic Consequences:**  
When an agent recalls knowledge, accessed nodes should automatically gain energy, temporal edges should form, and access patterns should be logged. The agent does not manage database bookkeeping — it thinks, and the system responds. Intent in, side-effects out.

### 1.3 The Proliferation Problem

Consider a modern AI agent using multiple backends:

```
Store knowledge         → SQL for PostgreSQL
Semantic search         → REST calls for Qdrant
Relationship traversal  → Cypher for Neo4j
Fast cache              → Redis commands
Switch database         → Rewrite everything
```

This is precisely the problem SQL solved for relational databases in the 1970s. Before SQL, every database had its own proprietary query language. SQL became the universal interface that outlasted dozens of competing products.

**AQL does the same for cognitive agents.** One language, any backend, with declared capabilities and graceful degradation.

---

## 2. What AQL Is

AQL (Agent Query Language) is a **backend-agnostic cognitive intent language** — a protocol defining the universal interface between AI agents and any memory system, regardless of the underlying storage engine.

### The Core Analogy

```
SQL  : Relational Databases  =  AQL  : Cognitive Memory Systems
```

SQL does not belong to PostgreSQL. It runs on MySQL, SQLite, Oracle, and SQL Server. Similarly, AQL does not belong to NietzscheDB. It runs on Neo4j, Qdrant, pgvector, Redis, MySQL, SQLite, SQL Server, and any backend implementing the `AqlBackend` trait.

The full cognitive experience — hyperbolic geometry, dream cycles, energy model, valence and arousal — **exists only with NietzscheDB**. Other backends degrade gracefully, providing as much cognitive capability as their architecture allows.

### What AQL Is NOT

AQL is **not** a query language with syntactic sugar over SQL.  
AQL is **not** compressed NAQ or simplified NQL.  
AQL is **not** yet another database driver.  

AQL is a **language for how agents think**, not a language for how databases execute. The shift is philosophical before it is technical.

---

## 3. Position in the Stack

NietzscheDB supports three query interfaces, each targeting a different operator:

```
+----------------------------------------------------------+
|  Human developers and administrators                     |
|  NQL — NietzscheDB Query Language                        |
|  Verbose, readable, MATCH / WHERE / RETURN syntax        |
+----------------------------------------------------------+
|  Agents requiring surgical control                       |
|  NAQ — NietzscheDB Agent Query                           |
|  Compact bytecodes: M:S?e>.5&c~physics>e-10             |
+----------------------------------------------------------+
|  Agents expressing cognitive intent  <- THIS LANGUAGE    |
|  AQL — Agent Query Language                              |
|  RECALL "quantum physics" CONFIDENCE 0.8 RECENCY recent  |
+----------------------------------------------------------+
```

The three layers are complementary, not competing. `NQL` is for humans reading. `NAQ` is for agents that need to specify *how* to execute. `AQL` is for agents that want to specify *what* they are trying to think.

### Full Execution Pipeline

```
AQL source text
      |
      v
  [Parser — Pest PEG grammar]
      |
      v
  AQL AST
  (Program -> Statement -> Verb + Subject + Qualifier*)
      |
      v
  [Cognitive Planner]
  |-- Selects execution strategy from verb + context + mood
  |-- Resolves @self to real collection/session
  |-- Converts RECENCY to timestamp ranges (chrono)
  |-- Applies epistemic upsert logic for IMPRINT
  |-- Checks cognitive energy gate
  +-- Propagates Bayesian confidence through THEN chains
      |
      v
  ExecutionPlan
  |-- Vec<NaqInstruction>         <- compiled NAQ
  |-- Vec<SideEffect>             <- implicit consequences
  +-- Option<Box<ExecutionPlan>>  <- chained THEN
      |
      v
  WorkingMemory (state between THEN steps)
      |
      v
  [Backend Executor — AqlBackend trait]
  (NietzscheDB | Neo4j | Qdrant | pgvector | Redis | MySQL | SQLite | MSSQL)
      |
      v
  CognitiveResult
```

---

## 4. The 13 Cognitive Verbs

AQL defines exactly 13 verbs, organized into three categories.

### 4.1 Core Verbs (8)

| Verb | Cognitive Act | SQL Analogy | Automatic Side-Effects |
|---|---|---|---|
| `RECALL` | Retrieve relevant memory | SELECT | BoostNodes, TemporalEdge, AccessPattern |
| `RESONATE` | Semantic resonance search | — | BoostNodes, ResonancePattern |
| `REFLECT` | Meta-cognition about self/graph | — | AccessPattern |
| `TRACE` | Follow causal narrative | JOIN + recursive CTE | BoostPathNodes, TemporalEdge |
| `IMPRINT` | Write new knowledge | INSERT / UPSERT | SessionContext, BoostLinked |
| `ASSOCIATE` | Create/reinforce association | — | TemporalEdge, BoostLinked |
| `DISTILL` | Extract patterns from episodes | GROUP BY + aggregate | CreatePatternNode, LinkEpisodes |
| `FADE` | Intentional forgetting | DELETE (soft) | RecordFadeEvent |

### 4.2 Geometric Verbs (3) — NietzscheDB only

| Verb | Cognitive Act | Geometry |
|---|---|---|
| `DESCEND` | Navigate deeper into hierarchy | Move toward Poincaré boundary (higher magnitude) |
| `ASCEND` | Navigate toward abstractions | Move toward Poincaré origin (lower magnitude) |
| `ORBIT` | Find peers at same conceptual depth | Filter by similar magnitude within curvature radius |

In the Poincaré ball, the origin represents the most abstract concepts. Moving toward the boundary reveals increasing specificity:

```
Magnitude 0.00–0.20  |  Physics, Mathematics           (most abstract)
Magnitude 0.20–0.50  |  Quantum Mechanics, Topology    (categories)
Magnitude 0.50–0.80  |  Quark Confinement, Möbius      (specific topics)
Magnitude 0.80–0.99  |  Specific experiments, proofs   (concrete instances)
```

### 4.3 Altered State Verbs (2) — NietzscheDB only

| Verb | Cognitive Act | Nature |
|---|---|---|
| `DREAM` | Creative recombination | Stochastic synthesis of existing knowledge nodes |
| `IMAGINE` | Counterfactual reasoning | Hypothetical graph modifications, non-persistent |

```aql
DREAM ABOUT "consciousness and computation" NOVELTY high
IMAGINE "what if quantum coherence persists in neurons?" DEPTH 4
```

### 4.4 Verb Side-Effects — Complete Table

Every verb produces automatic consequences that the agent never needs to manage explicitly:

| Verb | Side-Effects |
|---|---|
| `RECALL` | BoostAccessedNodes, CreateTemporalEdge, RecordAccessPattern |
| `RESONATE` | BoostAccessedNodes, RecordResonancePattern |
| `REFLECT` | RecordAccessPattern |
| `TRACE` | BoostPathNodes, CreateTemporalEdge |
| `IMPRINT` | AssociateToSessionContext, BoostLinkedNodes |
| `ASSOCIATE` | CreateTemporalEdge, BoostLinkedNodes |
| `DISTILL` | CreatePatternNode, LinkSourceEpisodes |
| `FADE` | RecordFadeEvent |
| `DREAM` | CreatePatternNode, BoostAccessedNodes |

---

## 5. Epistemic Type System

Unlike traditional databases where all data is treated identically, AQL classifies every piece of knowledge into one of five epistemic types. These types carry distinct energy dynamics, decay rates, emotional defaults, and conflict resolution behaviors.

### 5.1 The Five Types

| Type | Initial Energy | Decay Rate | Default Arousal | Purpose |
|---|---|---|---|---|
| `Belief` | 0.60 | 0.001 (slow) | 0.3 | Stable convictions, confirmed facts |
| `Experience` | 0.50 | 0.005 | 0.5 | Episodic memories, specific events |
| `Pattern` | 0.80 | 0.0005 (very slow) | 0.4 | Regularities extracted from many experiences |
| `Signal` | 0.30 | 0.050 (fast) | 0.8 | Transient sensor data, real-time events |
| `Intention` | 0.70 | 0.000 (none) | 0.6 | Goals, plans, desired future states |

### 5.2 Why This Matters

A `Signal` decays 100× faster than a `Pattern`. An `Intention` does not decay at all until resolved. This models how human memory works:

- You forget the exact temperature reading from yesterday (`Signal`).
- You remember "summer is hot" (`Pattern`) for decades.
- You remember your dissertation title (`Belief`) for years.
- An unresolved goal (`Intention`) persists until completed or explicitly abandoned.

When an agent executes `IMPRINT "market crash detected" AS Signal CONFIDENCE 0.9 AROUSAL high`, AQL knows this is high-activation, fast-decaying information. It will dominate near-term recalls but will naturally fade unless reinforced.

### 5.3 Structural Metadata per Type

```rust
pub struct BeliefMeta {
    pub confidence:     f32,    // [0.0, 1.0]
    pub source:         String,
    pub revision_count: u32,
}

pub struct ExperienceMeta {
    pub session_id:   String,
    pub timestamp:    i64,      // Unix milliseconds
    pub participants: Vec<String>,
}

pub struct PatternMeta {
    pub support_count: u32,     // episodes supporting this pattern
    pub stability:     f32,     // consistency across observations
    pub generality:    f32,     // breadth of applicable domain
}

pub struct SignalMeta {
    pub noise_level:        f32,
    pub source_reliability: f32,
}

pub struct IntentionMeta {
    pub priority: f32,
    pub status:   IntentionStatus,  // Active | Pending | Completed | Abandoned
}
```

### 5.4 NQL Type Mapping

| Epistemic Type | NQL/NAQ Type | Rationale |
|---|---|---|
| `Belief` | `Semantic` | High conceptual content, persistent |
| `Experience` | `Episodic` | Temporal structure, event-bound |
| `Pattern` | `Semantic` | Abstract, emergent |
| `Signal` | `Semantic` | Transient content, volatile energy |
| `Intention` | `Concept` | Structural, goal-oriented |

---

## 6. Full Grammar Specification

The grammar is the protocol contract. Every symbol has an associated parser test. Semantic changes require an explicit version bump.

```pest
// grammar.pest — AQL v1.0
// Protocol version byte: 0x01 prefixed to every ExecutionPlan

// --- Program ---------------------------------------------------

program   = { SOI ~ statement+ ~ EOI }
statement = { verb ~ subject ~ qualifier* ~ chain? ~ NEWLINE* }

// --- THEN chaining --------------------------------------------

chain = { "THEN" ~ statement }

// --- Verbs — all 13 cognitive acts ----------------------------

verb = {
    "RECALL"    | "RESONATE"  | "REFLECT"   | "TRACE"
  | "IMPRINT"   | "ASSOCIATE" | "DISTILL"   | "FADE"
  | "DESCEND"   | "ASCEND"    | "ORBIT"
  | "DREAM"     | "IMAGINE"
}

// --- Subject --------------------------------------------------

subject = {
    self_ref
  | type_with_content      // Belief:"quantum physics"
  | type_filter            // Belief
  | trace_range            // FROM "A" TO "B"
  | dream_subject          // ABOUT "topic"
  | quoted_string          // "any text"
}

self_ref          = { "@self" }
type_filter       = { epistemic_type }
type_with_content = { epistemic_type ~ ":" ~ quoted_string }
trace_range       = { "FROM" ~ quoted_string ~ "TO" ~ quoted_string }
dream_subject     = { "ABOUT" ~ quoted_string }

epistemic_type = {
    "Belief" | "Experience" | "Pattern" | "Signal" | "Intention"
}

// --- Qualifiers — all 12 --------------------------------------

qualifier = {
    confidence_q | recency_q  | depth_q    | within_q
  | as_q         | linking_q  | novelty_q  | limit_q
  | valence_q    | arousal_q  | mood_q     | evidence_q
}

confidence_q = { "CONFIDENCE" ~ float }
recency_q    = { "RECENCY"    ~ recency_degree }
depth_q      = { "DEPTH"      ~ integer }
within_q     = { "WITHIN"     ~ scope }
as_q         = { "AS"         ~ epistemic_type }
linking_q    = { "LINKING"    ~ quoted_string }
novelty_q    = { "NOVELTY"    ~ novelty_degree }
limit_q      = { "LIMIT"      ~ integer }
valence_q    = { "VALENCE"    ~ valence_value }
arousal_q    = { "AROUSAL"    ~ arousal_level }
mood_q       = { "MOOD"       ~ mood_state }
evidence_q   = { "EVIDENCE"   ~ integer }

recency_degree = { "fresh" | "recent" | "distant" | "ancient" }
novelty_degree = { "high"  | "medium" | "low" }
valence_value  = { "positive" | "negative" | "neutral" | float }
arousal_level  = { "high" | "medium" | "low" | float }
mood_state     = {
    "creative" | "analytical" | "anxious"
  | "focused"  | "exploratory" | "conservative"
}
scope = {
    "session" | "collection" | "graph" | quoted_string
}

// --- Control flow ---------------------------------------------

parallel_block = { statement ~ "AND" ~ statement+ }
atomic_block   = { "ATOMIC" ~ "{" ~ statement+ ~ "}" }
conditional    = {
    statement ~ "WHEN" ~ condition ~ ("ELSE" ~ statement)?
}
condition      = {
    "@results" ~ "." ~ condition_field ~ condition_op ~ float
}
condition_field = { "confidence" | "count" | "energy" }
condition_op    = { ">=" | "<=" | ">" | "<" | "==" }

// --- Primitives -----------------------------------------------

quoted_string = ${ "\"" ~ inner ~ "\"" }
inner         = @{ (!("\"") ~ ANY)* }
float         = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
integer       = @{ ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" | "\r" }
NEWLINE    = _{ "\n" }
COMMENT    = _{ "#" ~ (!"\n" ~ ANY)* ~ "\n" }
```

---

## 7. Complete AST

```rust
/// A complete AQL program.
pub struct Program {
    pub version:    u8,
    pub statements: Vec<Statement>,
}

/// A single cognitive act — the atom of AQL.
pub struct Statement {
    pub verb:       Verb,
    pub subject:    Subject,
    pub qualifiers: Vec<Qualifier>,
    /// THEN — chained next act.
    /// Receives WorkingMemory from the previous step.
    pub next:       Option<Box<Statement>>,
}

/// All 13 cognitive verbs.
pub enum Verb {
    Recall, Resonate, Reflect, Trace,         // core (4)
    Imprint, Associate, Distill, Fade,         // core (4)
    Descend, Ascend, Orbit,                    // geometric (3)
    Dream, Imagine,                            // altered states (2)
}

/// The target of the cognitive act.
pub enum Subject {
    Text(String),
    TypeFilter(EpistemicType),
    TypeWithContent { epistemic_type: EpistemicType, content: String },
    SelfRef,
    TraceRange { from: String, to: String },
    DreamAbout(String),
}

/// All 12 qualifiers.
pub enum Qualifier {
    Confidence(f32),
    Recency(RecencyDegree),
    Depth(u8),
    Within(ContextScope),
    As(EpistemicType),
    Linking(String),
    Novelty(NoveltyDegree),
    Limit(u32),
    Valence(ValenceValue),
    Arousal(ArousalLevel),
    Mood(MoodState),
    Evidence(u32),
}
```

---

## 8. Qualifiers Reference

### CONFIDENCE — Epistemic Threshold

`CONFIDENCE` is not a filter. It is an epistemic declaration with two distinct interpretations depending on the verb:

```rust
// For READ verbs (RECALL, RESONATE): maps to soft energy floor
fn confidence_to_energy_floor(c: f32) -> f32 { c * 0.5 }

// For WRITE verbs (IMPRINT): maps directly to initial node energy
fn confidence_to_initial_energy(c: f32) -> f32 { c }
```

The asymmetry is deliberate. When reading, the system may return nodes below the floor if context justifies. When writing, confidence becomes the exact energy of the new node.

### RECENCY — Temporal Cognitive Categories

| Value | Time Window | Energy Floor | NAQ Filter |
|---|---|---|---|
| `fresh` | < 5 minutes | 0.70 | `t > now-300s` |
| `recent` | < 1 hour | 0.40 | `t > now-3600s` |
| `distant` | < 24 hours | 0.20 | `t > now-86400s` |
| `ancient` | no limit | 0.05 | (none) |

### MOOD — Global Planner Modifier

The `MOOD` qualifier modifies how the planner interprets every other qualifier in the statement. The same query produces different execution plans depending on mood — exactly as human cognition works under different emotional states.

| Mood | Effect |
|---|---|
| `creative` | NOVELTY amplified, DEPTH unlimited, RESONATE preferred |
| `analytical` | CONFIDENCE strict, DEPTH limited, TRACE preferred |
| `anxious` | RECENCY forced to fresh, short chain depth |
| `focused` | LIMIT small, WITHIN session, noise suppressed |
| `exploratory` | NOVELTY high, DEPTH high, FADE suppressed |
| `conservative` | CONFIDENCE high, NOVELTY low, IMPRINT restricted |

### EVIDENCE — Logarithmic Weighting

```
combined_weight = confidence × log₂(count + 1)
```

50 observations provide ~5.7× more weight than 1 observation, but 1,000 observations provide only ~10×. Diminishing returns model real-world evidence accumulation.

### VALENCE — Emotional Polarity

Emotional polarity of the knowledge. Used both for storage (IMPRINT) and retrieval filtering (RECALL, RESONATE).

```aql
RECALL "past decisions" VALENCE negative MOOD analytical
IMPRINT "eureka moment" VALENCE positive AROUSAL high AS Experience
```

### AROUSAL — Activation Intensity

High arousal = urgent, intense, demanding attention. Low arousal = calm, background, available but not prominent.

---

## 9. Cognitive Planner

The Planner is the interpreter of intent. It receives an AST and produces an `ExecutionPlan`. It is mood-aware, context-aware, session-aware, and adaptive.

### Configuration

```rust
pub struct PlannerConfig {
    pub default_limit:             u32,   // 10
    pub default_knn_k:             u32,   // 10
    pub default_diffuse_depth:     u8,    // 3
    pub default_diffuse_threshold: f32,   // 0.5
    pub max_chain_depth:           u8,    // 8
    pub min_free_energy:           f32,   // 0.1
}
```

### Strategy Selection

```rust
fn choose_strategy(&self, stmt: &Statement) -> Strategy {
    match stmt.verb {
        Verb::Reflect  => self.strategy_reflect(stmt),
        Verb::Trace    => Strategy::CausalTrace,
        Verb::Imprint  => Strategy::CreateWithAssociation,
        Verb::Fade     => Strategy::EnergyDecrement,
        Verb::Distill  => Strategy::PatternExtraction,
        Verb::Resonate => Strategy::HybridSearch,
        Verb::Dream    => Strategy::StochasticRecombination,
        Verb::Imagine  => Strategy::CounterfactualExploration,
        Verb::Descend  => Strategy::HyperbolicDescent,
        Verb::Ascend   => Strategy::HyperbolicAscent,
        Verb::Orbit    => Strategy::HyperbolicOrbit,
        Verb::Recall | Verb::Associate => self.strategy_recall_adaptive(stmt),
    }
}

fn strategy_recall_adaptive(&self, stmt: &Statement) -> Strategy {
    // RECENCY present -> scan temporal energy
    if stmt.recency().is_some() {
        return Strategy::EnergyThresholdScan;
    }
    // Analytical mood + text -> vector search
    if stmt.mood() == Some(MoodState::Analytical) {
        return Strategy::VectorSearch;
    }
    // Adaptive rotation: if last 4 queries were KNN, rotate to scan
    let recent_knn = self.session_history.iter().rev().take(5)
        .filter(|e| e.strategy == Strategy::VectorSearch)
        .count();
    if recent_knn >= 4 { Strategy::EnergyThresholdScan }
    else { Strategy::VectorSearch }
}
```

---

## 10. Lowering AQL to NAQ — Complete Mapping

Every AQL statement compiles to one or more NAQ bytecodes. This section documents the complete mapping for all 13 verbs.

### RECALL

```
RECALL "quantum physics"
  -> K:S?c~"quantum physics"/10
     (KNN Semantic, k=10)

RECALL "quantum physics" CONFIDENCE 0.8
  -> K:S?c~"quantum physics"&e>0.4/10
     (energy floor = confidence * 0.5)

RECALL "quantum" RECENCY fresh LIMIT 5
  -> M:S?c~"quantum"&e>0.7&t>now-300s>e-5
     (energy scan, 5-min window, top 5)

RECALL Belief:"quantum" RECENCY recent
  -> M:S?c~"quantum"&e>0.4&t>now-3600s>e-10
     (Beliefs -> Semantic type, 1-hour window)
```

### RESONATE

```
RESONATE "consciousness emerges from complexity"
  -> K:S?c~"consciousness emerges"/10 | X:result_ids/0.5/2
     (KNN -> hyperbolic diffusion, 0.7/0.3 weighted)

RESONATE "quantum" NOVELTY high DEPTH 3
  -> K:S?c~"quantum"/20 | X:result_ids/0.7/3
     + filter(distance_from_session > 0.70)
```

### REFLECT

```
REFLECT @self
  -> G:pagerank:S?collection="agent_self_knowledge"/10
     M:*?session_id="<current>">t-10

REFLECT @self WITHIN session
  -> M:*?session_id="<current>">e-20

REFLECT Pattern
  -> G:pagerank:S/10
```

### TRACE

```
TRACE FROM "observation" TO "conclusion"
  -> P:"observation">>"conclusion"
     (shortest path, prefers Causal and Temporal edge types)

TRACE FROM "A" TO "B" DEPTH 5
  -> P:"A">>"B"/5
```

### IMPRINT

```
IMPRINT "new hypothesis" CONFIDENCE 0.6
  -> C:S{c:"new hypothesis",e:0.6}

IMPRINT "X is true" CONFIDENCE 0.6 LINKING "quantum"
  -> C:S{c:"X is true",e:0.6}
     + C:edge{from:new_id,to:"quantum",type:Association,w:0.6}

IMPRINT "insight" AS Belief CONFIDENCE 0.6 VALENCE positive
  -> C:S{c:"insight",e:0.6,meta:{type:"Belief",conf:0.6,valence:1.0}}
```

### ASSOCIATE

```
ASSOCIATE "quantum" LINKING "consciousness"
  -> IF edge exists: U:edge{from:"quantum",to:"consciousness"}{w:+0.1}
     IF not exists:  C:edge{from:"quantum",to:"consciousness",w:0.5}
     (upsert with reinforcement)
```

### DISTILL

```
DISTILL Experience WITHIN session
  -> M:E?session_id="<current>">e-50       # collect episodes
     | G:louvain:result_ids/0.3            # cluster communities
     | C:S{c:cluster_summary,e:0.8,meta:{type:"Pattern"}}
     | C:edge{from:pattern_id,to:each_episode,type:Supports}
```

### FADE

```
FADE Signal CONFIDENCE 0.2
  -> U:S?meta.type="Signal"&e<0.2{e:-0.3}
     D:S?e<0.05!
     (decrement, then prune below threshold)

FADE Experience RECENCY ancient
  -> U:E?t<now-86400s{e:-0.2}
     D:E?e<0.05!
```

### DESCEND / ASCEND / ORBIT (NietzscheDB only)

```
DESCEND "physics" DEPTH 3 MAGNITUDE 0.3..0.7
  -> H:descent:"physics"/0.3/0.7/3

ASCEND "quark confinement" DEPTH 2
  -> H:ascent:"quark confinement"/2

ORBIT "quantum mechanics" RADIUS 0.05 CURVATURE high
  -> H:orbit:"quantum mechanics"/0.05/high
```

### DREAM / IMAGINE (NietzscheDB only)

```
DREAM ABOUT "consciousness" NOVELTY high
  -> G:pagerank:S/20
     | stochastic_recombine(result_ids, novelty=0.7, temperature=1.2)

IMAGINE "what if quantum coherence persists?" DEPTH 4
  -> K:S?c~"quantum coherence"/10
     | hypothetical_modify(result_ids, hypothesis, depth=4)
     (non-persistent: modifications are sandboxed)
```

---

## 11. Working Memory and THEN Chains

The most critical design gap in ad-hoc agent systems: data flow between chained operations. AQL's `WorkingMemory` makes this explicit and guaranteed.

### The Problem

```aql
RECALL "quantum"
THEN ASSOCIATE "quantum" LINKING "consciousness"
```

The `ASSOCIATE` step needs the actual node IDs retrieved by `RECALL`. Without explicit state management, the second step is blind and operates on nothing.

### WorkingMemory

```rust
pub struct WorkingMemory {
    /// Node IDs returned by the previous step.
    pub result_node_ids:  Vec<String>,
    /// Node contents for semantic use in subsequent steps.
    pub result_contents:  Vec<String>,
    /// Average energy of results (calibrates next step).
    pub avg_energy:       f32,
    /// Node created by the previous step (for THEN ASSOCIATE).
    pub created_node_id:  Option<String>,
    /// Session ID — propagated through entire chain.
    pub session_id:       String,
    /// Current chain depth (anti-loop guard).
    pub chain_depth:      u8,
    /// Parallel branch results by index.
    pub parallel_results: HashMap<usize, StepResult>,
    /// Result from DELEGATE operation.
    pub delegate_result:  Option<StepResult>,
    /// Last DREAM output.
    pub last_dream:       Option<StepResult>,
}
```

### Full Chain Execution Flow

```
RECALL "quantum"
  -> WorkingMemory { ids: [q1, q2, q3], avg_energy: 0.78 }

THEN ASSOCIATE "quantum" LINKING "consciousness"
  -> Receives [q1, q2, q3], creates edges from each to "consciousness"
  -> WorkingMemory { created_edge_ids: [e1, e2, e3] }

THEN IMPRINT "quantum-consciousness link confirmed" CONFIDENCE 0.7
  -> Auto-links to edge_ids from previous step
  -> WorkingMemory { created_node_id: "b1" }

THEN REFLECT @self WITHIN session
  -> Operates on full session context including all above operations
```

### @results References

```aql
# Parallel recall, then associate the two results
RECALL "machine learning" AND RECALL "consciousness"
THEN ASSOCIATE @results[0] LINKING @results[1] CONFIDENCE 0.75

# Delegate to specialist, import result
DELEGATE RECALL "rare pattern" TO agent:"specialist_agent"
THEN IMPRINT @delegate.result AS Belief

# Use last dream output
DREAM ABOUT "physics"
THEN ASSOCIATE @last_dream LINKING "creativity"
```

---

## 12. Self Resolution (@self)

`@self` is not a placeholder — it is a dynamic resolution of the agent's own cognitive state, adapting to the scope qualifier.

### Resolution Matrix

| Statement | Resolution Type | NAQ Output |
|---|---|---|
| `REFLECT @self` | AgentCollection | `G:pagerank:S?collection="agent_self_knowledge"/10` |
| `REFLECT @self WITHIN session` | SessionNodes | `M:*?session_id="sess-abc">e-20` |
| `REFLECT @self WITHIN graph` | GraphWide | `G:pagerank:*/20?agent_id="eva-7"` |
| `REFLECT @self WITHIN "physics"` | NamedCollection | `M:S?collection="physics">e-10` |

### SelfResolver

```rust
pub enum SelfResolution {
    SessionNodes    { session_id: String },
    AgentCollection { collection: String, agent_id: String },
    GraphWide       { agent_id: String },
    NamedCollection { collection: String },
}
```

`REFLECT @self` without a `WITHIN` qualifier defaults to `AgentCollection`, pointing to the agent's dedicated self-knowledge collection. This is where the agent's long-term beliefs, patterns, and intentions live — its persistent cognitive identity.

---

## 13. Epistemic Upsert Logic (IMPRINT)

When `IMPRINT "X"` executes and "X" already exists in the graph, AQL applies a configurable conflict resolution policy. **This determines whether an agent is epistemically stubborn or impressionable.**

### Conflict Policies

```rust
pub enum ConflictPolicy {
    /// Keep existing if it has higher confidence.
    /// Conservative: only update if new info is better.
    KeepHigherConfidence,

    /// Weighted average of confidences.
    /// Integrates new evidence gradually (Bayesian update).
    WeightedAverage { weight: f32 },

    /// Always replace with incoming information.
    /// Impressionable: most recent wins.
    ReplaceAlways,

    /// Create an explicit conflict node.
    /// Preserves contradiction for future resolution.
    CreateConflict,
}
```

### Default Policies by Epistemic Type

| Type | Default Policy | Rationale |
|---|---|---|
| `Belief` | `WeightedAverage { weight: 0.3 }` | Beliefs update gradually |
| `Experience` | `ReplaceAlways` | Experiences are facts |
| `Pattern` | `WeightedAverage { weight: 0.1 }` | Patterns are stable, change slowly |
| `Signal` | `ReplaceAlways` | More recent signals always win |
| `Intention` | `KeepHigherConfidence` | Intentions are deliberate decisions |

### Example

```
IMPRINT "X is true" CONFIDENCE 0.6
  (existing node has energy/confidence 0.8)

KeepHigherConfidence  -> Noop (existing wins)
WeightedAverage(0.3)  -> new_energy = 0.8*0.7 + 0.6*0.3 = 0.74
ReplaceAlways         -> Update to 0.6
CreateConflict        -> Creates conflict node + ConflictsWith edge
```

---

## 14. Cognitive Energy Model

Every knowledge node has an energy level between 0.0 and 1.0. Energy models the **activation level** of a memory: how available it is for retrieval, how strongly it influences its neighbors, whether it persists or fades.

### Energy Dynamics

| Event | Effect |
|---|---|
| Node creation | Starts at epistemic type's `initial_energy` |
| RECALL / RESONATE access | Boosts energy by `access_energy_boost` |
| Natural decay per cycle | Decreases by `decay_rate` (type-dependent) |
| FADE operation | Explicit decrement by configurable amount |
| Energy < 0.05 | Node becomes eligible for automatic pruning |

### Lifecycle Example

```
t=0:    IMPRINT "important meeting" AS Experience  -> energy = 0.50
t=1h:   Natural decay                              -> energy = 0.491
t=2h:   RECALL "important meeting"                 -> energy = 0.521 (+0.030)
t=24h:  No access, continued decay                 -> energy = 0.461
t=7d:   No access                                  -> energy = 0.361
t=30d:  FADE "important meeting"                   -> energy = 0.061
t=60d:  Continued decay                            -> energy = 0.003 (prunable)
```

This models the **Ebbinghaus forgetting curve** (1885): memories decay unless reinforced through access.

### Cognitive Energy States

Energy gating determines which operations the agent can perform based on total available free energy across the session:

| State | Free Energy | Restrictions |
|---|---|---|
| Solid | < 0.20 | DISTILL, DREAM, IMAGINE blocked |
| Liquid | 0.20 – 0.70 | All verbs, DISTILL limited to DEPTH 3 |
| Gaseous | > 0.70 | All verbs unrestricted, DREAM at full temperature |

### Verb Energy Costs

| Verb | Cost | Rationale |
|---|---|---|
| RECALL | 0.02 | Simple retrieval |
| RESONATE | 0.10 | Hybrid search |
| REFLECT | 0.05 | Meta-cognition |
| TRACE | 0.08 | Path search |
| IMPRINT | 0.15 | Knowledge write |
| ASSOCIATE | 0.12 | Relation creation |
| DISTILL | 0.25 | Pattern extraction (expensive) |
| FADE | 0.03 | Forgetting |
| DREAM | 0.30 | Creative synthesis (most expensive) |

---

## 15. Uncertainty Propagation

AQL treats confidence as a first-class epistemic data type that propagates through operation chains according to Bayesian rules.

### Chain Propagation

```aql
RECALL "quantum" CONFIDENCE 0.8
THEN RECALL "consciousness" CONFIDENCE 0.7
THEN ASSOCIATE @results[0] LINKING @results[1]
```

The final association inherits combined confidence: `0.8 × 0.7 = 0.56`. The chain is only as strong as its weakest link — a fundamental property of any reasoning system.

### Uncertainty in IMPRINT

```aql
IMPRINT "quantum-consciousness link detected"
    CONFIDENCE 0.56  # inherited from chain
    AS Belief
```

The new Belief node starts with energy 0.56, reflecting the accumulated uncertainty of the reasoning chain that produced it.

### Evidence Weighting

```aql
RECALL "climate models" EVIDENCE 50 CONFIDENCE 0.8
```

Combined weight: `0.8 × log₂(51) ≈ 4.55`

50 observations provide ~5.7× more evidential weight than 1. 1,000 observations provide only ~10×. Diminishing returns, modeling real-world evidence accumulation.

---

## 16. Affective Computing: Valence and Arousal

AQL models the emotional dimension of knowledge through two orthogonal axes, enabling mood-aware and affect-sensitive retrieval.

### Valence (−1.0 to +1.0)

Emotional polarity. Positive memories (success, discovery) have high valence. Negative memories (failure, loss) have low valence.

```aql
RECALL "project outcomes" VALENCE positive
IMPRINT "breakthrough moment!" VALENCE positive AROUSAL high AS Experience
```

### Arousal (0.0 to 1.0)

Activation intensity. High arousal = urgent, intense. Low arousal = calm, background.

### Affective Filtering in Practice

```aql
# Analytical agent reviewing past mistakes
RECALL "past decisions" VALENCE negative RECENCY distant MOOD analytical

# Creative agent seeking inspiration from positive experiences
RECALL "past successes" VALENCE positive NOVELTY high MOOD creative
```

An `analytical` mood retrieves negative experiences with clinical structure. A `creative` mood surfaces them as sources of insight and recombination fuel. **The same data, different cognitive purpose, different result shape.**

---

## 17. Hyperbolic Geometry Verbs

AQL is the first query language to natively support **hyperbolic geometry** through the Poincaré ball model.

### Why Hyperbolic Space?

Hierarchical data (taxonomies, ontologies, concept trees) has exponential branching structure. Euclidean space requires exponentially more dimensions to represent hierarchies faithfully. The Poincaré ball encodes hierarchical depth through one continuous scalar: **magnitude** (distance from origin).

```
Magnitude 0.00–0.20  |  "Physics", "Mathematics"          (most abstract)
Magnitude 0.20–0.50  |  "Quantum Mechanics", "Topology"   (categories)
Magnitude 0.50–0.80  |  "Quark Confinement", "Möbius"     (specific topics)
Magnitude 0.80–0.99  |  specific experiments, measurements (concrete)
```

### Geometric Operations

```aql
# Navigate from "Physics" deeper, targeting mid-depth nodes
DESCEND "physics" DEPTH 3 MAGNITUDE 0.3..0.7

# From a specific concept, navigate toward its abstractions
ASCEND "quark confinement" DEPTH 2

# Find concepts at the same conceptual depth (peers)
ORBIT "quantum mechanics" RADIUS 0.05 CURVATURE high
```

`CURVATURE` measures local manifold density — highly curved regions have many closely packed neighbors, indicating rich, well-connected knowledge areas.

### Backend Degradation

On non-NietzscheDB backends, geometric verbs return `AqlError::UnsupportedGeometry`. The planner can suggest flat Euclidean alternatives. This is declared, not silent.

---

## 18. Parallel Execution and Control Flow

### AND — Parallel Branches

```aql
RECALL "quantum" AND RECALL "consciousness"
THEN ASSOCIATE @results[0] LINKING @results[1]
```

Both RECALL operations execute concurrently via tokio tasks. Results are collected and available to the THEN step via `@results[index]`.

### ATOMIC — Transactional Block

```aql
ATOMIC {
    IMPRINT "critical update" AS Belief CONFIDENCE 0.95
    ASSOCIATE "critical update" LINKING "system state"
    FADE "obsolete data"
}
```

All operations succeed or all are rolled back. Supported by: NietzscheDB, PostgreSQL, MySQL, SQLite, SQL Server. Not supported by: Redis, Qdrant (declared in BackendCapabilities).

### WHEN / ELSE — Conditional Execution

```aql
RECALL "latest data" WHEN @results.confidence >= 0.8
ELSE RECALL "archived data" RECENCY ancient
```

Conditional execution based on result properties: confidence, count, or energy level.

### WATCH / SUBSCRIBE — Reactive Patterns

```aql
WATCH "system alerts" ON_CHANGE
    RECALL "related incidents"
    THEN ASSOCIATE @results[0] LINKING "alert"

SUBSCRIBE Belief ON_INSERT
    DISTILL @results AS Pattern
```

Reactive operations via tokio broadcast channels. `WATCH` triggers on modifications; `SUBSCRIBE` triggers on new inserts. Agents respond to changes without polling.

---

## 19. Multi-Agent Coordination

### SHARE — Publish Knowledge

```aql
SHARE "discovery" WITH agent:"peer_agent"
```

Publishes a node or result set to another named agent. Recipient agent may RECALL or RESONATE against received knowledge.

### DELEGATE — Offload Computation

```aql
DELEGATE RECALL "complex pattern" TO agent:"specialist_agent"
THEN IMPRINT @delegate.result AS Belief
```

Sends an AQL statement to another agent for execution. The result flows back through WorkingMemory as `@delegate.result`.

### NEGOTIATE — Conflict Resolution

```aql
NEGOTIATE "shared conclusion" WITH agent:"peer_agent"
    POLICY weighted_average
```

Four policies: `weighted_average`, `keep_higher`, `replace_always`, `create_conflict`. The same policies as IMPRINT epistemic upsert — a unified conflict resolution model across local and multi-agent operations.

---

## 20. Executor Pipeline

```rust
pub struct AqlExecutor {
    planner:      CognitivePlanner,
    naq_executor: NaqExecutor,        // trait: any backend
}

impl AqlExecutor {

    pub async fn execute_program(
        &mut self,
        program: Program,
    ) -> Result<Vec<CognitiveResult>, AqlError> {
        let mut results = Vec::new();
        for stmt in program.statements {
            let result = self.execute_statement(&stmt).await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn execute_chain(
        &mut self,
        stmt: &Statement,
        memory: &mut WorkingMemory,
    ) -> Result<CognitiveResult, AqlError> {
        // 1. Plan the current step
        let mut plan = self.planner.plan(stmt)?;

        // 2. Inject WorkingMemory from previous step
        memory.inject_into_plan(&mut plan);

        // 3. Execute NAQ instructions
        let step_result = self.execute_plan(&plan).await?;

        // 4. Apply side-effects automatically
        self.apply_side_effects(&plan.side_effects, &step_result).await?;

        // 5. Debit cognitive energy
        self.planner.energy_hooks.debit(&stmt.verb);

        // 6. Update WorkingMemory
        memory.update_from_result(&step_result);

        // 7. Execute THEN chain with depth guard
        if let Some(next) = &stmt.next {
            if memory.chain_depth >= self.planner.config.max_chain_depth {
                return Err(AqlError::ChainDepthExceeded(memory.chain_depth));
            }
            return self.execute_chain(next, memory).await;
        }

        Ok(CognitiveResult::from_step_result(step_result))
    }
}
```

### CognitiveResult

```rust
pub struct CognitiveResult {
    pub nodes:           Vec<CognitiveNode>,
    pub total:           u32,
    pub effects_applied: Vec<SideEffect>,
    pub energy_debited:  f32,
    pub plan_label:      String,       // human-readable plan description
    pub confidence:      f32,          // propagated from chain
}

pub struct CognitiveNode {
    pub id:             String,
    pub content:        String,
    pub energy:         f32,
    pub epistemic_type: Option<EpistemicType>,
    pub confidence:     Option<f32>,
    pub valence:        Option<f32>,
    pub arousal:        Option<f32>,
}
```

---

## 21. Error Protocol

### AqlError

```rust
#[derive(Debug, thiserror::Error)]
pub enum AqlError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("unknown verb: '{0}'")]
    UnknownVerb(String),

    #[error("unknown epistemic type: '{0}'")]
    UnknownType(String),

    #[error("invalid confidence {0}: must be [0.0, 1.0]")]
    InvalidConfidence(f32),

    #[error("verb '{verb}' requires subject '{expected}'")]
    InvalidSubjectForVerb { verb: String, expected: String },

    #[error("insufficient cognitive energy: need {required:.2}, have {available:.2} ({verb})")]
    InsufficientCognitiveEnergy { required: f32, available: f32, verb: String },

    #[error("THEN chain exceeded max depth {0}")]
    ChainDepthExceeded(u8),

    #[error("version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: u8, got: u8 },

    #[error("geometry unsupported by backend: {verb}")]
    UnsupportedGeometry { verb: String },

    #[error("planner error: {0}")]
    PlannerError(String),
}
```

### Compact Error Format (protocol wire)

```json
{"err": 1, "pos": 0, "msg": "parse_error"}
{"err": 4, "pos": 2, "msg": "confidence_range"}
{"err": 6, "pos": 0, "msg": "insufficient_energy", "required": 0.25, "available": 0.08}
{"err": 9, "pos": 0, "msg": "unsupported_geometry", "verb": "DESCEND"}
```

| Code | Message |
|---|---|
| 1 | parse_error |
| 2 | unknown_verb |
| 3 | unknown_type |
| 4 | confidence_range |
| 5 | subject_mismatch |
| 6 | insufficient_energy |
| 7 | chain_depth_exceeded |
| 8 | version_mismatch |
| 9 | unsupported_geometry |
| 10 | planner_error |

---

## 22. Backend Capabilities Matrix

Each backend declares its capabilities through `BackendCapabilities`. Agents can query capabilities at runtime and plan accordingly.

| Capability | NietzscheDB | Neo4j | Qdrant | pgvector | Redis | MySQL | SQLite | SQL Server |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Hyperbolic Geometry | YES | — | — | — | — | — | — | — |
| Vector Search | YES | YES | YES | YES | YES | — | — | — |
| Full-Text Search | YES | YES | YES | YES | YES | YES | YES | YES |
| Graph Traversal | YES | YES | — | — | — | CTE | CTE | Graph |
| Edge Types | YES | YES | — | YES | — | YES | YES | YES |
| Cognitive Energy Model | YES | — | — | — | — | — | — | — |
| Dream / Imagine | YES | — | — | — | — | — | — | — |
| Valence / Arousal | YES | — | — | — | — | — | — | — |
| ATOMIC Transactions | YES | YES | — | YES | — | YES | YES | YES |
| TTL (auto-expire) | YES | — | — | — | YES | — | — | — |
| WATCH / SUBSCRIBE | YES | — | — | — | — | — | — | — |
| DESCEND / ASCEND | YES | — | — | — | — | — | — | — |

When an agent sends `DREAM ABOUT "consciousness"` to a MySQL backend, it receives `AqlError::UnsupportedGeometry`, not silent failure.

---

## 23. Crate Architecture

AQL is implemented as a Rust workspace with 12 crates:

| Crate | Lines | Purpose |
|---|---|---|
| `aql-core` | ~2000 | Parser, AST, Planner, Executor, Types, Energy, Affect, Uncertainty |
| `aql-nietzschedb` | ~400 | NietzscheDB backend (full cognitive experience) |
| `aql-neo4j` | ~200 | Neo4j backend (Cypher lowering) |
| `aql-qdrant` | ~200 | Qdrant backend (REST/gRPC lowering) |
| `aql-pgvector` | ~200 | PostgreSQL + pgvector backend (SQL lowering) |
| `aql-redis` | ~200 | Redis Stack backend (RediSearch lowering) |
| `aql-mysql` | ~200 | MySQL / MariaDB backend (FULLTEXT + recursive CTE) |
| `aql-sqlite` | ~200 | SQLite backend (FTS5 + WAL mode) |
| `aql-mssql` | ~200 | SQL Server backend (FREETEXT + graph tables) |
| `aql-cli` | ~150 | Interactive REPL with syntax highlighting |
| `aql-wasm` | ~50 | Browser / edge WASM build |
| `aql-python` | ~100 | Python bindings via PyO3 |

### Directory Layout

```
crates/aql-core/src/
|-- grammar.pest            <- PEG grammar (versioned)
|-- lib.rs                  <- re-exports + AQL_VERSION
|-- types/
|   |-- epistemic.rs        <- EpistemicType + metadata
|   |-- recency.rs          <- RecencyDegree + timestamp conversion
|   |-- scope.rs            <- ContextScope + @self resolution
|   +-- affect.rs           <- ValenceValue, ArousalLevel, MoodState
|-- ast/
|   |-- statement.rs        <- Statement + chain
|   |-- verb.rs             <- Verb + side-effects per verb
|   |-- subject.rs          <- Subject (all 6 variants)
|   +-- qualifier.rs        <- Qualifier (all 12)
|-- parser/
|   +-- parser.rs           <- pest -> AST
|-- planner/
|   |-- cognitive_planner.rs   <- strategy selection (adaptive)
|   |-- lowering.rs            <- AQL -> NaqInstruction (all 13 verbs)
|   |-- imprint.rs             <- epistemic upsert logic
|   |-- self_resolver.rs       <- @self resolution
|   +-- energy_hooks.rs        <- energy gating + debit
|-- runtime/
|   |-- executor.rs            <- ExecutionPlan -> CognitiveResult
|   |-- working_memory.rs      <- THEN state management
|   +-- result.rs              <- CognitiveResult + CognitiveNode
+-- error.rs                   <- AqlError + wire error codes
```

### Why Rust?

- **Zero-cost abstractions** — The trait system and async/await compile to native code with no runtime overhead.
- **Memory safety** — No garbage collector, no data races, verified at compile time.
- **Async-native** — tokio runtime for parallel AND execution and reactive WATCH.
- **Multi-target** — Same codebase compiles to native binary, WASM, and Python bindings.
- **Pest parser** — PEG grammar compiles to a zero-copy parser at build time.

---

## 24. Acceptance Tests

Every feature has a canonical acceptance test. Green tests mean the feature is implemented.

```rust
// --- Parser ---------------------------------------------------

#[test]
fn parses_all_13_verbs() {
    for verb in [
        "RECALL","RESONATE","REFLECT","TRACE",
        "IMPRINT","ASSOCIATE","DISTILL","FADE",
        "DESCEND","ASCEND","ORBIT",
        "DREAM","IMAGINE",
    ] {
        let p = parse_aql(&format!("{} \"test\"\n", verb)).unwrap();
        assert_eq!(p.statements.len(), 1);
    }
}

#[test]
fn parses_all_12_qualifiers() {
    let q = r#"RECALL "test"
        CONFIDENCE 0.8 RECENCY recent DEPTH 3 WITHIN session
        AS Belief LINKING "other" NOVELTY high LIMIT 5
        VALENCE positive AROUSAL high MOOD creative EVIDENCE 10
"#;
    assert_eq!(parse_one(q).qualifiers.len(), 12);
}

#[test]
fn parses_trace_range() {
    let s = parse_one("TRACE FROM \"obs\" TO \"conc\"\n");
    assert!(matches!(s.subject, Subject::TraceRange { .. }));
}

#[test]
fn rejects_confidence_out_of_range() {
    assert!(parse_aql("RECALL \"test\" CONFIDENCE 1.5\n").is_err());
}

// --- Planner --------------------------------------------------

#[test]
fn recall_semantic_uses_vector_search() {
    let plan = plan_one("RECALL \"quantum\"\n");
    assert_eq!(plan.strategy, Strategy::VectorSearch);
}

#[test]
fn recall_with_recency_uses_energy_scan() {
    let plan = plan_one("RECALL \"quantum\" RECENCY fresh\n");
    assert_eq!(plan.strategy, Strategy::EnergyThresholdScan);
}

#[test]
fn reflect_self_uses_session_introspection() {
    let plan = plan_one("REFLECT @self\n");
    assert_eq!(plan.strategy, Strategy::SessionIntrospection);
}

#[test]
fn distill_uses_pattern_extraction() {
    let plan = plan_one("DISTILL Experience\n");
    assert_eq!(plan.strategy, Strategy::PatternExtraction);
}

// --- Lowering -------------------------------------------------

#[test]
fn confidence_maps_to_half_energy_floor() {
    let plan = plan_one("RECALL \"test\" CONFIDENCE 0.8\n");
    let e_filter = plan.instructions[0].where_clauses.iter()
        .find(|w| w.field == "e").unwrap();
    assert!((e_filter.value.parse::<f32>().unwrap() - 0.4).abs() < 0.01);
}

#[test]
fn fresh_recency_maps_to_300s_window() {
    let plan = plan_one("RECALL \"test\" RECENCY fresh\n");
    let t_filter = plan.instructions[0].where_clauses.iter()
        .find(|w| w.field == "t").unwrap();
    assert!(t_filter.value.contains("300s"));
}

#[test]
fn imprint_confidence_is_initial_energy() {
    let plan = plan_one("IMPRINT \"new idea\" CONFIDENCE 0.65\n");
    assert!((plan.instructions[0].energy.unwrap() - 0.65).abs() < 0.01);
}

// --- WorkingMemory --------------------------------------------

#[test]
fn chain_passes_ids_to_associate() {
    let result = run_mock(
        "RECALL \"quantum\"\nTHEN ASSOCIATE \"quantum\" LINKING \"consciousness\"\n"
    ).unwrap();
    // ASSOCIATE step must have received IDs from RECALL
    assert!(!result.steps[1].context_node_ids.is_empty());
}

// --- Epistemic upsert -----------------------------------------

#[test]
fn higher_confidence_keeps_existing() {
    let planner = ImprintPlanner::new(ConflictPolicy::KeepHigherConfidence);
    let existing = ExistingNode { id: "n1".into(), energy: 0.9 };
    assert!(matches!(planner.plan_imprint("test", 0.6, Some(existing)),
        NaqInstruction::Noop));
}

#[test]
fn lower_confidence_updates_node() {
    let planner = ImprintPlanner::new(ConflictPolicy::KeepHigherConfidence);
    let existing = ExistingNode { id: "n1".into(), energy: 0.4 };
    assert!(matches!(planner.plan_imprint("test", 0.8, Some(existing)),
        NaqInstruction::Update { .. }));
}

// --- Energy gating --------------------------------------------

#[test]
fn distill_blocked_at_low_energy() {
    let mut hooks = EnergyHooks::with_energy(0.05);
    assert!(matches!(hooks.check_can_execute(&Verb::Distill),
        Err(AqlError::InsufficientCognitiveEnergy { .. })));
}

#[test]
fn recall_permitted_at_minimal_energy() {
    let mut hooks = EnergyHooks::with_energy(0.15);
    assert!(hooks.check_can_execute(&Verb::Recall).is_ok());
}
```

---

## 25. Real-World Examples

### 25.1 Autonomous Research Agent

```aql
# Morning recall: recent papers with high confidence
RECALL "transformer architectures" CONFIDENCE 0.8 RECENCY recent MOOD analytical

# Find semantic connections in creative mode
RESONATE "attention mechanisms relate to consciousness" MOOD creative

# Trace causal reasoning path
TRACE FROM "self-attention" TO "emergent behavior" DEPTH 5

# Store insight with uncertainty from the trace chain
IMPRINT "attention may be a form of selective consciousness"
    AS Belief CONFIDENCE 0.6 VALENCE positive AROUSAL medium

# Creative synthesis: dream and extract patterns
DREAM ABOUT "consciousness and computation" NOVELTY high
THEN DISTILL @last_dream AS Pattern CONFIDENCE 0.5
```

### 25.2 Medical Diagnosis Agent

```aql
# Parallel symptom recall
RECALL "chest pain" AND RECALL "shortness of breath" AND RECALL "elevated troponin"
THEN DISTILL @results AS Pattern CONFIDENCE 0.85

# Trace differential diagnosis path
TRACE FROM "symptoms" TO "diagnosis" DEPTH 3 WITHIN "cardiology"

# Store hypothesis with evidence count
IMPRINT "possible acute coronary syndrome"
    AS Belief CONFIDENCE 0.75 EVIDENCE 3

# Atomic treatment plan — all succeed or all rollback
ATOMIC {
    IMPRINT "administer aspirin 325mg" AS Intention CONFIDENCE 0.9
    ASSOCIATE "aspirin" LINKING "acute coronary syndrome"
    IMPRINT "order serial troponins" AS Intention CONFIDENCE 0.9
}
```

### 25.3 Multi-Agent Knowledge Coordination

```aql
# Agent A discovers an anomaly
IMPRINT "anomaly in sector 7: readings outside 3-sigma" AS Signal AROUSAL high

# Share with specialist agent
SHARE "anomaly in sector 7" WITH agent:"analysis_agent"

# Delegate deep historical analysis
DELEGATE RECALL "similar anomalies in past 90 days" TO agent:"historian_agent"
THEN IMPRINT @delegate.result AS Belief CONFIDENCE 0.7

# Negotiate consensus classification
NEGOTIATE "anomaly classification" WITH agent:"analysis_agent"
    POLICY weighted_average
```

### 25.4 Self-Reflective Learning Session

```aql
# Collect and compress session experiences
RECALL Experience WITHIN session RECENCY fresh LIMIT 20
THEN DISTILL Experience DEPTH 3 CONFIDENCE 0.65
THEN IMPRINT @results AS Pattern CONFIDENCE 0.7 LINKING "session-learnings"

# Review cognitive state after session
REFLECT @self WITHIN session
```

### 25.5 Hierarchical Navigation in Hyperbolic Space

```aql
# Start at high abstraction, descend to find relevant mid-level concepts
DESCEND "physics" DEPTH 3 MAGNITUDE 0.3..0.7
THEN ORBIT @results[0] RADIUS 0.05 CURVATURE high
THEN RECALL @results[0] CONFIDENCE 0.7
```

### 25.6 Selective Cognitive Pruning

```aql
# Prune stale signals and ancient low-energy experiences
FADE Signal RECENCY ancient CONFIDENCE 0.15
FADE Experience RECENCY distant CONFIDENCE 0.10 WITHIN session

# Reinforce what remains important
RECALL "core beliefs" AS Belief RECENCY ancient CONFIDENCE 0.7
THEN ASSOCIATE @results[0] LINKING "long-term-memory"
```

---

## 26. Comparison with Existing Approaches

| Aspect | SQL | Cypher | Vector API | LangChain Memory | AQL |
|---|---|---|---|---|---|
| Cognitive verbs | — | — | — | Partial | **13 verbs** |
| Epistemic types | — | — | — | — | **5 types** |
| Confidence as data | — | — | Score only | — | **Bayesian** |
| Energy model | — | — | — | — | **Full decay/boost** |
| Hyperbolic geometry | — | — | — | — | **Poincaré ball** |
| Emotional dimensions | — | — | — | — | **Valence + Arousal** |
| THEN chains | — | — | — | Chain only | **With WorkingMemory** |
| Multi-agent | — | — | — | — | **SHARE / DELEGATE** |
| Dream / Imagine | — | — | — | — | **Altered states** |
| Backend-agnostic | Partial | — | — | Yes | **Full trait system** |
| Reactive (WATCH) | Triggers | — | — | — | **Built-in** |
| Parallel execution | — | — | — | — | **AND / FORK / JOIN** |
| Atomic transactions | YES | YES | — | — | **With rollback** |
| Uncertainty propagation | — | — | — | — | **Bayesian chains** |

### Where AQL Does NOT Compete

AQL is not a replacement for SQL in transactional systems. If you are managing financial records, inventory, or user accounts, use SQL. AQL is for one specific problem: **how an AI agent interacts with a knowledge memory system**. It does that job and nothing else, by design.

---

## 27. Implementation Roadmap

### Phase 1 — Core (Weeks 1–3)

- [ ] Complete `grammar.pest` with all 13 verbs, all 12 qualifiers, THEN, AND, ATOMIC, WHEN
- [ ] Parser: pest → AST for all grammatical forms
- [ ] `EpistemicType` with decay rates, energy defaults, conflict policies
- [ ] `RecencyDegree::to_min_timestamp()` with chrono
- [ ] Parser acceptance tests for all 50+ cases

### Phase 2 — Planner and Lowering (Weeks 4–6)

- [ ] `CognitivePlanner` with all 13 strategy branches
- [ ] Complete AQL → NAQ lowering for all 13 verbs
- [ ] `SelfResolver` with all 4 resolution types
- [ ] `ImprintPlanner` with all 4 conflict policies
- [ ] `MoodState` modifier in strategy selection
- [ ] Bayesian confidence propagation through THEN chains

### Phase 3 — Runtime (Weeks 7–9)

- [ ] `WorkingMemory` with `inject_into_plan()` and `@results` references
- [ ] `AqlExecutor` with full chain execution and depth guard
- [ ] `apply_side_effects()` for all 10 side-effect types
- [ ] Parallel execution with tokio tasks for AND branches
- [ ] ATOMIC block with rollback on failure

### Phase 4 — Energy and Affect (Weeks 10–11)

- [ ] `EnergyHooks` with per-verb costs and state gating
- [ ] `AffectModel` with valence/arousal filtering in planner
- [ ] Integration with NietzscheDB solid/liquid/gaseous states
- [ ] Acceptance tests for energy blocking behavior

### Phase 5 — Backends (Weeks 12–16)

- [ ] `aql-nietzschedb`: full NQL + gRPC lowering
- [ ] `aql-neo4j`: Cypher lowering for core 8 verbs
- [ ] `aql-qdrant`: REST/gRPC lowering for RECALL, RESONATE
- [ ] `aql-pgvector`: SQL + pgvector lowering
- [ ] `aql-redis`: RediSearch FT.SEARCH lowering
- [ ] `aql-mysql`, `aql-sqlite`, `aql-mssql`: full-text SQL lowering
- [ ] `BackendCapabilities` declaration and runtime query

### Phase 6 — Ecosystem (Weeks 17–20)

- [ ] `aql-cli`: interactive REPL with syntax highlighting and plan explain
- [ ] `aql-python`: PyO3 bindings
- [ ] `aql-wasm`: WASM build for browser and edge runtimes
- [ ] Round-trip tests: AQL → NAQ → execution → expected result
- [ ] All acceptance tests green

---

## 28. Conclusion

The world of AI agents needs what SQL gave relational databases fifty years ago: **a universal language for interacting with memory.**

Every AI team that builds an agent with memory today writes the same ad-hoc translation layers. Every project reinvents the same wheel. There is no standard. There is no portability. There is no expressiveness beyond raw database commands that know nothing about what knowledge means to the entity using it.

AQL provides this universal layer by treating cognitive operations as first-class primitives:

- An agent does not `SELECT` — it `RECALL`s, and the system boosts accessed nodes, records access patterns, and creates temporal edges automatically.
- Confidence is not a filter — it is epistemic data that propagates through reasoning chains according to Bayesian rules.
- Energy is not metadata — it is the lifecycle of knowledge, modeling how memories decay without reinforcement and persist through use.
- Emotion is not decoration — valence and arousal shape what an agent remembers and how aggressively it searches.

The three axioms that define AQL:

1. **Intent as primitive** — `RECALL "quantum physics"` is a cognitive act, not a database instruction.
2. **Uncertainty as data** — `CONFIDENCE 0.7` participates in Bayesian propagation, not threshold filtering.
3. **Effects as consequences** — Accessing memory changes memory. The agent thinks; the system responds.

AQL is designed to be to cognitive agent systems what SQL was to relational databases: the standard that outlasts any particular implementation. NietzscheDB is the first and fullest implementation. But the language belongs to the problem, not the product.

**AQL is the SQL of cognitive agents. And it speaks to those who think.**

---

*AQL v2.0 — Agent Query Language*  
*Created by Jose R F Junior — 2026*  
*AGPL-3.0 — Open Source, Open Mind*  
*github.com/JoseRFJuniorLLMs/NietzscheDB*
