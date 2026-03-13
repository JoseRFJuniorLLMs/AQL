# AQL — Agent Cognition Language: The SQL of Cognitive Agents

**Author:** Jose R F Junior
**Version:** 2.0.0 — 2026
**License:** AGPL-3.0
**Repository:** github.com/JoseRFJuniorLLMs/AQL

---

> *"NQL speaks to those who read. NAQ speaks to those who calculate. AQL speaks to those who think."*

---

## 1. The Problem: Why the World Needs a Cognitive Query Language

### 1.1. The Gap Between Agents and Databases

The AI revolution brought us large language models, autonomous agents, and multi-modal reasoning. But there is a critical gap: **how does an agent interact with its memory?**

Today, when an AI agent needs to store or retrieve knowledge, it faces a fragmented landscape:

- **SQL** was designed for structured tabular data in 1970. It models rows and columns, not beliefs, intentions, or emotional associations.
- **Cypher** (Neo4j) models relationships well but has no concept of epistemic confidence, cognitive energy, or hyperbolic geometry.
- **Vector search APIs** (Qdrant, Pinecone, pgvector) find similar embeddings but have no understanding of what knowledge *means* to the agent.
- **Key-value stores** (Redis) offer speed but no cognitive structure whatsoever.

The result? Developers write ad-hoc translation layers for each database. Every project reinvents the same wheel. There is no standard way for an agent to say: *"I believe this with 80% confidence"* or *"recall what I learned yesterday, but only if it feels relevant."*

### 1.2. The Three Missing Primitives

Traditional query languages lack three fundamental primitives that cognitive agents require:

1. **Intention as Primitive** — The atomic unit should be a cognitive act (recall, associate, dream), not a database instruction (SELECT, INSERT). When an agent says `RECALL "quantum physics"`, the system should understand this as a retrieval act with semantic context, not a keyword search.

2. **Uncertainty as Data Type** — `CONFIDENCE 0.7` is an epistemic statement about the agent's degree of belief. It is not a filter threshold. It participates in Bayesian propagation, evidence weighting, and attention budgeting.

3. **Effects as Automatic Consequences** — When an agent recalls knowledge, the accessed nodes should automatically gain energy, temporal edges should form, and access patterns should be recorded. The agent declares intent; the system applies side-effects.

No existing query language provides these three properties simultaneously.

### 1.3. The Proliferation Problem

Consider the current state of an AI agent that uses multiple backends:

```
Agent needs to store knowledge → writes SQL for PostgreSQL
Agent needs to search semantically → writes REST calls for Qdrant
Agent needs to traverse relationships → writes Cypher for Neo4j
Agent needs fast cache → writes Redis commands
Agent needs to switch database → rewrites everything
```

This is exactly the problem SQL solved for relational databases in the 1970s-80s. Before SQL, every database had its own proprietary query language. SQL became the universal interface.

**AQL does the same for cognitive agents.** One language, any backend, with graceful degradation.

---

## 2. What is AQL?

AQL (Agent Cognition Language) is a **backend-agnostic cognitive intent language** designed for AI agents interacting with knowledge graphs, vector databases, and relational stores.

AQL is not a database. It is a **protocol** — a universal interface between agents and their memory, regardless of the underlying storage engine.

### 2.1. The Core Analogy

```
SQL   : Relational Databases  =  AQL  : Cognitive Memory Systems
```

SQL does not belong to PostgreSQL. It works with MySQL, SQLite, Oracle, SQL Server. Similarly, AQL does not belong to NietzscheDB. It works with Neo4j, Qdrant, pgvector, Redis, MySQL, SQLite, SQL Server, and any future backend that implements the `AqlBackend` trait.

The full cognitive experience — hyperbolic geometry, dream cycles, energy model — **only exists with NietzscheDB**. Other backends function with graceful degradation, providing as much cognitive capability as their architecture allows.

### 2.2. Architecture

```
+-----------------------------------------------------+
|                  Agent / LLM / MCP                   |
+-----------------------------------------------------+
|           AQL -- Agent Cognition Language             |
|  RECALL . RESONATE . REFLECT . TRACE . IMPRINT       |
|  ASSOCIATE . DISTILL . FADE . DESCEND . ASCEND       |
|  DREAM . IMAGINE . ORBIT                             |
|  AND . WHEN . ATOMIC . SHARE . DELEGATE              |
+-----------------------------------------------------+
|              AQL Backend Trait                        |
|    fn recall()  fn resonate()  fn imprint() ...      |
+----------+-----------+-----------+-------------------+
|NietzscheDB|   Neo4j  |  Qdrant   |  pgvector / Redis |
| (Poincare)|  (Cypher)|  (REST)   |  (SQL / FT)       |
|  FULL *   |  Partial |  Partial  |   Minimal          |
+-----------+---------+-----------+--------------------+
|     MySQL / MariaDB |  SQLite   |  SQL Server        |
|     (FULLTEXT)      |  (FTS5)   |  (FREETEXT)        |
|      Relational     | Relational|  Relational        |
+---------------------+-----------+--------------------+
```

---

## 3. The 13 Cognitive Verbs

AQL defines exactly 13 verbs, organized into three categories. Each verb is a cognitive act, not a database command.

### 3.1. Core Verbs (8)

| Verb | Cognitive Act | SQL Equivalent | What Happens |
|------|--------------|----------------|-------------|
| `RECALL` | Retrieve relevant memory | SELECT | Semantic search + energy boost on accessed nodes |
| `RESONATE` | Semantic resonance search | — | Vector similarity + emotional resonance pattern |
| `REFLECT` | Meta-cognition about self/graph | — | Introspective query about the agent's own knowledge state |
| `TRACE` | Follow causal path between concepts | JOIN + CTE | Graph traversal with path energy boosting |
| `IMPRINT` | Write new knowledge | INSERT/UPSERT | Creates node with epistemic type, energy, valence, arousal |
| `ASSOCIATE` | Create/reinforce association | — | Creates or strengthens edges with temporal recording |
| `DISTILL` | Extract patterns from episodes | — | Pattern extraction from multiple nodes, creates Pattern node |
| `FADE` | Intentional forgetting | DELETE | Decreases energy; node is deleted when energy reaches threshold |

### 3.2. Geometric Verbs (3)

These verbs navigate hyperbolic space (Poincare ball model) where **magnitude = depth in hierarchy**.

| Verb | Cognitive Act | Geometry |
|------|--------------|----------|
| `DESCEND` | Navigate deeper in hierarchy | Move toward boundary (higher magnitude) |
| `ASCEND` | Navigate to abstractions | Move toward origin (lower magnitude) |
| `ORBIT` | Find peers at same depth | Filter by similar magnitude |

In the Poincare ball, the origin represents the most abstract concept. As you move toward the boundary (magnitude approaching 1.0), concepts become more specific. This mirrors how human knowledge is organized: "Physics" at the center, "Quantum Chromodynamics" near the edge.

### 3.3. Altered State Verbs (2)

| Verb | Cognitive Act | Nature |
|------|--------------|--------|
| `DREAM` | Creative dream cycle | Stochastic recombination of existing knowledge |
| `IMAGINE` | Counterfactual reasoning | "What if?" explorations with hypothetical modifications |

---

## 4. Epistemic Type System

Unlike traditional databases where all data is treated equally, AQL classifies every piece of knowledge into one of five epistemic types. Each type has distinct energy dynamics, decay rates, and emotional defaults.

### 4.1. The Five Types

| Type | Initial Energy | Decay Rate | Default Arousal | Purpose |
|------|---------------|------------|-----------------|---------|
| **Belief** | 0.6 | 0.001 (slow) | 0.3 | Stable convictions, facts the agent holds as true |
| **Experience** | 0.5 | 0.005 | 0.5 | Episodic memories, specific events |
| **Pattern** | 0.8 | 0.0005 (very slow) | 0.4 | Learned regularities, extracted from many experiences |
| **Signal** | 0.3 | 0.05 (fast) | 0.8 | Transient stimuli, sensor data, real-time events |
| **Intention** | 0.7 | 0.01 | 0.6 | Goals, plans, desired future states |

### 4.2. Why This Matters

A `Signal` decays 100x faster than a `Pattern`. This models how human memory works: you forget the exact temperature reading from yesterday (Signal), but you remember the pattern "summer is hot" (Pattern) for decades.

When an agent does `IMPRINT "market crash detected" AS Signal CONFIDENCE 0.9 AROUSAL high`, AQL knows this is high-activation, fast-decaying information. The energy model ensures it will be prominent in near-term recalls but will naturally fade unless reinforced.

---

## 5. Uncertainty Propagation

AQL treats confidence not as a filter but as a **first-class epistemic data type** that propagates through operations.

### 5.1. Propagation Rules

```
RECALL "quantum" CONFIDENCE 0.8
THEN RECALL "consciousness" CONFIDENCE 0.7
THEN ASSOCIATE @results[0] LINKING @results[1]
```

The final association inherits combined confidence: `0.8 * 0.7 = 0.56`. This is Bayesian propagation — the chain is only as strong as its weakest link.

### 5.2. Evidence Weighting

```
RECALL "climate change" EVIDENCE 50
```

The `EVIDENCE` qualifier combines observation count with confidence using logarithmic weighting:

```
combined_weight = confidence * log2(count + 1)
```

This means 50 observations provide ~5.7x more weight than a single observation, but 1000 observations provide only ~10x. Diminishing returns, modeling real-world evidence accumulation.

---

## 6. Affective Computing: Valence and Arousal

AQL models the emotional dimension of knowledge through two orthogonal axes:

### 6.1. Valence (-1.0 to +1.0)

Emotional polarity. Positive memories (joy, success) have high valence. Negative memories (failure, trauma) have low valence.

```aql
RECALL "project outcomes" VALENCE positive
IMPRINT "eureka moment!" VALENCE positive AROUSAL high AS Experience
```

### 6.2. Arousal (0.0 to 1.0)

Activation level. High arousal = urgent, intense. Low arousal = calm, background.

### 6.3. Mood States

The `MOOD` qualifier globally modifies planner behavior:

| Mood | Effect |
|------|--------|
| `creative` | NOVELTY high, DEPTH unlimited, RESONATE amplified |
| `analytical` | CONFIDENCE high, DEPTH limited, TRACE preferred |
| `anxious` | RECENCY fresh, CONFIDENCE high, short chains |
| `focused` | LIMIT small, WITHIN session, suppress noise |
| `exploratory` | NOVELTY high, DEPTH high, FADE suppressed |
| `conservative` | CONFIDENCE high, NOVELTY low, IMPRINT restricted |

A creative agent in exploratory mood will retrieve more diverse, novel results. An anxious agent will stick to recent, high-confidence knowledge. **The same query returns different results depending on the agent's emotional state** — exactly as human cognition works.

---

## 7. Cognitive Energy Model

Every knowledge node has an energy level between 0.0 and 1.0. Energy models the **activation level** of a memory.

### 7.1. Energy Dynamics

- **Creation**: Node starts with energy from its epistemic type (e.g., Belief = 0.6)
- **Access Boost**: Every RECALL/RESONATE that touches a node boosts its energy
- **Natural Decay**: Energy decays exponentially over time at a rate determined by epistemic type
- **Intentional Fading**: `FADE` explicitly reduces energy
- **Deletion Threshold**: When energy drops below 0.05, the node is eligible for deletion

### 7.2. Example Lifecycle

```
t=0:  IMPRINT "important meeting" AS Experience    → energy = 0.5
t=1h: Natural decay                                 → energy = 0.491
t=2h: RECALL "important meeting"                    → energy = 0.521 (boost +0.03)
t=24h: No access, continued decay                   → energy = 0.461
t=7d: No access                                     → energy = 0.361
t=30d: FADE "important meeting"                     → energy = 0.061
t=60d: Below threshold                              → eligible for deletion
```

This models the **forgetting curve** (Ebbinghaus, 1885) — memories decay unless reinforced by access. Frequently recalled knowledge persists; neglected knowledge fades naturally.

---

## 8. Hyperbolic Geometry: DESCEND, ASCEND, ORBIT

AQL is the first query language to natively support **hyperbolic geometry** through the Poincare ball model.

### 8.1. Why Hyperbolic Space?

Hierarchical data (taxonomies, ontologies, knowledge trees) has an exponential branching structure. Euclidean space cannot represent this efficiently — you need exponentially more dimensions.

The Poincare ball (a hyperbolic manifold) naturally encodes hierarchical relationships through a single property: **magnitude** (distance from origin).

```
Magnitude 0.0-0.2  →  Most abstract concepts (Physics, Mathematics)
Magnitude 0.2-0.5  →  Mid-level categories (Quantum Mechanics, Topology)
Magnitude 0.5-0.8  →  Specific topics (Quark Confinement, Mobius Strips)
Magnitude 0.8-0.99 →  Concrete instances (specific experiments, proofs)
```

### 8.2. Geometric Verbs in Action

```aql
# Navigate from "Physics" deeper into its hierarchy
DESCEND "physics" DEPTH 3 MAGNITUDE 0.3..0.7

# Rise from specifics to abstractions
ASCEND "quark confinement" DEPTH 2

# Find peers at similar depth
ORBIT "quantum mechanics" RADIUS 0.05 CURVATURE high
```

The `CURVATURE` qualifier measures local density — highly curved regions have many neighbors packed closely, indicating rich, well-connected knowledge areas.

### 8.3. Backend Degradation

Only NietzscheDB provides native hyperbolic geometry. On other backends:
- `DESCEND`/`ASCEND`/`ORBIT` → return `UnsupportedGeometry` error
- The planner transparently degrades to flat (Euclidean) alternatives when possible

---

## 9. Parallel Execution and Control Flow

### 9.1. Parallel (AND)

```aql
RECALL "quantum" AND RECALL "consciousness"
THEN ASSOCIATE @results[0] LINKING @results[1]
```

Both recalls execute concurrently (via tokio tasks). Results are collected and available to the THEN step via `@results[index]`.

### 9.2. Sequential (THEN)

```aql
RECALL "machine learning" CONFIDENCE 0.8
THEN DISTILL @results[0] AS Pattern
THEN IMPRINT @results[1] AS Belief
```

Each step has access to previous results through `@results`.

### 9.3. Atomic Transactions

```aql
ATOMIC {
    IMPRINT "critical update" AS Belief CONFIDENCE 0.95
    ASSOCIATE "critical update" LINKING "system state"
    FADE "obsolete data"
}
```

All operations succeed or all are rolled back. Backend-dependent — NietzscheDB, PostgreSQL, MySQL, and SQL Server support ATOMIC; Redis and Qdrant do not.

### 9.4. Conditionals (WHEN/ELSE)

```aql
RECALL "latest data" WHEN @results.confidence >= 0.8
ELSE RECALL "archived data" RECENCY ancient
```

Conditional execution based on result properties — confidence, count, energy level.

---

## 10. Reactive System: WATCH and SUBSCRIBE

```aql
WATCH "system alerts" ON_CHANGE
    RECALL "related incidents" THEN ASSOCIATE @results[0] LINKING "alert"

SUBSCRIBE Belief ON_INSERT
    DISTILL @results AS Pattern
```

AQL supports reactive programming via tokio broadcast channels. `WATCH` triggers on modifications; `SUBSCRIBE` triggers on new inserts. This enables agents to respond to changes in their knowledge base without polling.

---

## 11. Multi-Agent Coordination

### 11.1. SHARE — Publish Knowledge

```aql
SHARE "discovery" WITH agent:"peer_agent"
```

### 11.2. DELEGATE — Offload Computation

```aql
DELEGATE RECALL "complex query" TO agent:"specialist_agent"
THEN IMPRINT @delegate.result AS Belief
```

### 11.3. NEGOTIATE — Resolve Conflicts

```aql
NEGOTIATE "shared_conclusion" WITH agent:"peer"
    POLICY weighted_average
```

Four conflict resolution policies:
- `weighted_average` — merge by confidence weights
- `keep_higher` — highest confidence wins
- `replace_always` — latest writer wins
- `create_conflict` — keep both, create a conflict node

---

## 12. Backend Capabilities and Graceful Degradation

Each backend declares its capabilities through the `BackendCapabilities` struct:

| Capability | NietzscheDB | Neo4j | Qdrant | pgvector | Redis | MySQL | SQLite | SQL Server |
|------------|:-----------:|:-----:|:------:|:--------:|:-----:|:-----:|:------:|:----------:|
| Hyperbolic Geometry | Yes | - | - | - | - | - | - | - |
| Vector Search | Yes | Yes | Yes | Yes | Yes | - | - | - |
| Full-Text Search | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Graph Traversal | Yes | Yes | - | - | - | CTE | CTE | Graph Tables |
| Edge Types | Yes | Yes | - | Yes | - | Yes | Yes | Yes |
| Energy Model | Yes | - | - | - | - | - | - | - |
| Dream Cycles | Yes | - | - | - | - | - | - | - |
| Valence/Arousal | Yes | - | - | - | - | - | - | - |
| ATOMIC Transactions | Yes | Yes | - | Yes | - | Yes | Yes | Yes |
| TTL | Yes | - | - | - | Yes | - | - | - |
| WATCH/SUBSCRIBE | Yes | - | - | - | - | - | - | - |

When an agent sends `DREAM ABOUT "consciousness"` to a MySQL backend, it receives a clear `UnsupportedVerb` error instead of silent failure. The planner can suggest alternatives or fall back to supported operations.

---

## 13. The AQL Pipeline

Every AQL statement flows through a four-stage pipeline:

```
Source Text → Parser → Planner → Executor → Backend
   "RECALL"    PEG      AST→Plan   Plan→Trait   Trait→DB
```

### 13.1. Parser (Pest PEG)

The grammar is defined as a Parsing Expression Grammar with 200 lines covering all 13 verbs, subjects, qualifiers, parallel blocks, atomic transactions, conditionals, and reactive statements.

### 13.2. Cognitive Planner

The planner converts AST nodes into execution plans. It is **mood-aware** — the same AST produces different plans depending on the agent's current mood state. A creative mood amplifies novelty; an anxious mood restricts to recent, high-confidence results.

### 13.3. Executor

The executor dispatches plans to the backend trait implementation. It handles:
- Sequential chains (THEN)
- Parallel branches (AND/FORK/JOIN) via tokio tasks
- Atomic blocks with rollback
- Working memory for `@results` references

### 13.4. Backend Lowering

Each backend translates plans into its native query language:
- NietzscheDB → NQL + gRPC
- Neo4j → Cypher
- Qdrant → JSON REST API
- pgvector → SQL + `<=>` operator
- Redis → RediSearch FT.SEARCH
- MySQL → SQL + MATCH AGAINST
- SQLite → SQL + FTS5 MATCH
- SQL Server → T-SQL + FREETEXTTABLE + Graph MATCH

---

## 14. Working Memory and State Management

AQL maintains a `WorkingMemory` struct across chain execution:

```rust
pub struct WorkingMemory {
    pub chain_results: Vec<CognitiveResult>,
    pub parallel_results: HashMap<usize, CognitiveResult>,
    pub variables: HashMap<String, serde_json::Value>,
    pub last_dream: Option<CognitiveResult>,
    pub delegate_result: Option<CognitiveResult>,
    pub explanation: Option<Explanation>,
}
```

This enables references like `@results[0]`, `@last_dream`, `@delegate.result`, and `@self` — creating a coherent cognitive session where each operation builds on previous results.

---

## 15. Side Effects: Automatic Consequences

Every verb has implicit side effects that the system applies automatically:

| Verb | Side Effects |
|------|-------------|
| RECALL | BoostAccessedNodes, CreateTemporalEdge, RecordAccessPattern |
| RESONATE | BoostAccessedNodes, RecordResonancePattern |
| TRACE | BoostPathNodes, CreateTemporalEdge |
| IMPRINT | AssociateToSessionContext, BoostLinkedNodes |
| ASSOCIATE | CreateTemporalEdge, BoostLinkedNodes |
| DISTILL | CreatePatternNode, LinkSourceEpisodes |
| FADE | RecordFadeEvent |
| DREAM | CreatePatternNode, BoostAccessedNodes |

The agent never needs to manually manage energy, temporal edges, or access patterns. **Intent in, side effects out.**

---

## 16. Advantages of AQL

### 16.1. For Agent Developers

- **One language, any backend** — Write cognitive queries once, deploy on NietzscheDB, PostgreSQL, MySQL, SQLite, Neo4j, Qdrant, Redis, or SQL Server
- **No translation layer** — Stop writing ad-hoc code to convert between agent intent and database commands
- **Cognitive primitives built-in** — Confidence, energy, valence, arousal are native, not bolt-on metadata
- **Graceful degradation** — Capabilities are declared, not assumed. Your agent knows what each backend can do

### 16.2. For AI Researchers

- **Epistemic type system** — Five knowledge types with distinct energy dynamics model human memory categories
- **Uncertainty propagation** — Bayesian confidence flows through operation chains
- **Affective computing** — Emotional state influences retrieval, not just storage
- **Hyperbolic geometry** — Native Poincare ball operations for hierarchical knowledge

### 16.3. For System Architects

- **Backend-agnostic protocol** — Swap databases without rewriting agent logic
- **Trait-based extensibility** — Add new backends by implementing `AqlBackend`
- **Multi-runtime** — Rust core with Python bindings (PyO3), WASM build, CLI REPL
- **Production-ready patterns** — ATOMIC transactions, WATCH reactivity, multi-agent coordination

### 16.4. For the Ecosystem

- **Standardization** — Just as SQL standardized relational access, AQL standardizes cognitive access
- **Interoperability** — Agents speaking AQL can switch backends, share knowledge, coordinate
- **Open source (AGPL-3.0)** — Free to use, study, modify, and distribute

---

## 17. Comparison with Existing Approaches

| Aspect | SQL | Cypher | Vector API | LangChain Memory | **AQL** |
|--------|-----|--------|-----------|-----------------|---------|
| Cognitive Verbs | No | No | No | Partial | **13 verbs** |
| Epistemic Types | No | No | No | No | **5 types** |
| Confidence as Data | No | No | Score only | No | **Bayesian** |
| Energy Model | No | No | No | No | **Full decay/boost** |
| Hyperbolic Geometry | No | No | No | No | **Poincare ball** |
| Emotional Dimensions | No | No | No | No | **Valence + Arousal** |
| Multi-Agent | No | No | No | No | **SHARE/DELEGATE** |
| Dream/Imagine | No | No | No | No | **Altered states** |
| Backend-Agnostic | Partial | No | No | Yes | **Full trait system** |
| Reactive (WATCH) | Triggers | No | No | No | **Built-in** |
| Parallel Execution | No | No | No | Chain only | **AND/FORK/JOIN** |

---

## 18. Real-World Examples

### 18.1. Autonomous Research Agent

```aql
# Morning: recall recent papers with high confidence
RECALL "transformer architectures" CONFIDENCE 0.8 RECENCY recent MOOD analytical

# Find semantic connections
RESONATE "attention mechanisms relate to consciousness" MOOD creative

# Trace causal reasoning
TRACE FROM "self-attention" TO "emergent behavior" DEPTH 5

# Store insight
IMPRINT "attention may be a form of selective consciousness"
    AS Belief CONFIDENCE 0.6 VALENCE positive AROUSAL medium

# Dream for creative synthesis
DREAM ABOUT "consciousness and computation" NOVELTY high
```

### 18.2. Medical Diagnosis Agent

```aql
# Parallel symptom recall
RECALL "chest pain" AND RECALL "shortness of breath" AND RECALL "elevated troponin"
THEN DISTILL @results AS Pattern CONFIDENCE 0.85

# Trace differential diagnosis
TRACE FROM "symptoms" TO "diagnosis" DEPTH 3 WITHIN "cardiology"

# Store with uncertainty
IMPRINT "possible acute coronary syndrome"
    AS Belief CONFIDENCE 0.75 EVIDENCE 3

# Atomic treatment plan
ATOMIC {
    IMPRINT "administer aspirin 325mg" AS Intention
    ASSOCIATE "aspirin" LINKING "acute coronary syndrome"
    IMPRINT "order serial troponins" AS Intention
}
```

### 18.3. Multi-Agent Collaboration

```aql
# Agent A discovers something
IMPRINT "anomaly detected in sector 7" AS Signal AROUSAL high

# Share with specialist
SHARE "anomaly detected" WITH agent:"analysis_agent"

# Delegate deep analysis
DELEGATE RECALL "similar anomalies" TO agent:"historian_agent"

# Negotiate consensus
NEGOTIATE "anomaly classification" WITH agent:"analysis_agent"
    POLICY weighted_average
```

---

## 19. Implementation: The Rust Workspace

AQL is implemented as a Rust workspace with 12 crates:

| Crate | Lines | Purpose |
|-------|-------|---------|
| `aql-core` | ~2000 | Parser, AST, Planner, Executor, Types, Energy, Affect, Uncertainty |
| `aql-nietzschedb` | ~400 | NietzscheDB backend (full cognitive experience) |
| `aql-neo4j` | ~200 | Neo4j backend (Cypher lowering) |
| `aql-qdrant` | ~200 | Qdrant backend (REST/gRPC lowering) |
| `aql-pgvector` | ~200 | PostgreSQL+pgvector backend (SQL lowering) |
| `aql-redis` | ~200 | Redis Stack backend (RediSearch lowering) |
| `aql-mysql` | ~200 | MySQL/MariaDB backend (FULLTEXT, recursive CTE) |
| `aql-sqlite` | ~200 | SQLite backend (FTS5, embedded, WAL) |
| `aql-mssql` | ~200 | SQL Server backend (FREETEXT, graph tables) |
| `aql-cli` | ~150 | Interactive REPL with syntax highlighting |
| `aql-wasm` | ~50 | Browser/edge WASM build |
| `aql-python` | ~100 | Python bindings via PyO3 |

### 19.1. Why Rust?

- **Zero-cost abstractions** — The trait system and async/await compile to native performance
- **Memory safety** — No garbage collector, no data races, verified at compile time
- **Async-native** — tokio runtime for parallel execution and reactive WATCH
- **Multi-target** — Same codebase compiles to native, WASM, and Python bindings
- **Pest parser** — PEG grammar compiles to zero-copy parser at build time

### 19.2. Build and Test

```bash
cargo build --workspace     # Compiles all 12 crates
cargo test --workspace      # Runs 15 tests (parser, uncertainty, hyperbolic, doctest)
```

---

## 20. The Vision: Cognitive Infrastructure

AQL is not just a query language. It is the foundation of **cognitive infrastructure** — the layer between thinking agents and persistent memory.

### 20.1. Today

- 13 cognitive verbs covering retrieval, storage, navigation, creativity, and collaboration
- 8 backends spanning graph databases, vector stores, relational databases, and caches
- Epistemic type system with energy model, uncertainty propagation, and affective computing
- Native hyperbolic geometry for hierarchical knowledge
- Multi-agent coordination with conflict resolution

### 20.2. Tomorrow

- **MCP (Model Context Protocol) integration** — AQL as the standard memory protocol for LLM tool-use
- **Federated cognition** — Agents on different backends sharing knowledge through AQL
- **Cognitive migrations** — Move agent memory between backends without losing epistemic metadata
- **AQL Studio** — Visual IDE for cognitive query development and knowledge graph exploration
- **Standardization** — Community-driven specification for cognitive query interoperability

---

## 21. Conclusion

The world of AI agents needs what SQL gave relational databases 50 years ago: **a universal language for interacting with memory.**

AQL provides this by treating cognitive operations as first-class primitives. An agent does not SELECT or INSERT — it RECALLS, RESONATES, DREAMS, and IMAGINES. Confidence is not a filter — it is epistemic. Energy is not metadata — it is the lifecycle of knowledge. Emotion is not decoration — it shapes what the agent remembers and how.

**AQL is the SQL of cognitive agents.** And it speaks to those who think.

---

*AQL v2.0 — Agent Cognition Language*
*Created by Jose R F Junior, 2026*
*AGPL-3.0 — Open Source, Open Mind*
