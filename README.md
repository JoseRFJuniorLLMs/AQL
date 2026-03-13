# AQL — Agent Cognition Language

> *O NQL fala para quem le. O NAQ fala para quem calcula. O AQL fala para quem pensa.*
> *E agora — AQL fala para qualquer banco que queira pensar.*

**AQL is the SQL of cognitive agents.** A universal protocol that models how an agent *uses* memory, not how to query it.

## The Three Axioms

1. **Intention as Primitive** — The atomic unit is a cognitive act, not a database instruction.
2. **Uncertainty as Data Type** — `CONFIDENCE 0.7` is epistemic, not a filter.
3. **Effects as Automatic Consequences** — The agent declares intent; the server applies side-effects.

## The 13 Cognitive Verbs

| Verb | Purpose | Category |
|------|---------|----------|
| `RECALL` | Retrieve relevant memory | Core |
| `RESONATE` | Semantic resonance search | Core |
| `REFLECT` | Meta-cognition about self/graph | Core |
| `TRACE` | Follow causal path between concepts | Core |
| `IMPRINT` | Write new knowledge | Core |
| `ASSOCIATE` | Create/reinforce association | Core |
| `DISTILL` | Extract patterns from episodes | Core |
| `FADE` | Intentional forgetting | Core |
| `DESCEND` | Navigate deeper in hierarchy | Geometric |
| `ASCEND` | Navigate to abstractions | Geometric |
| `ORBIT` | Find peers at same depth | Geometric |
| `DREAM` | Creative dream cycle | Altered States |
| `IMAGINE` | Counterfactual reasoning | Altered States |

## Quick Start

```aql
# Recall with epistemic confidence
RECALL "quantum physics" CONFIDENCE 0.8

# Semantic resonance with mood
RESONATE "consciousness emerges from complexity" MOOD creative

# Trace causal path
TRACE FROM "observation" TO "conclusion" DEPTH 5

# Write knowledge with emotion
IMPRINT "eureka moment!" VALENCE positive AROUSAL high AS Belief

# Navigate hyperbolic hierarchy (NietzscheDB)
DESCEND "physics" DEPTH 3 MAGNITUDE 0.3..0.7

# Dream about a topic
DREAM ABOUT "quantum consciousness" NOVELTY high

# Parallel execution
RECALL "quantum" AND RECALL "consciousness"
THEN ASSOCIATE @results[0] LINKING @results[1]
```

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Agent / LLM / MCP                  │
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
│NietzscheDB│   Neo4j   │  Qdrant   │  pgvector/Redis │
│  (NAQ)    │  (Cypher) │  (REST)   │  (SQL/FT)       │
│ Poincare  │ Euclidean │ Euclidean │  Euclidean      │
│ FULL ★    │  Partial  │  Partial  │   Minimal       │
└───────────┴───────────┴───────────┴─────────────────┘
```

## Workspace Crates

| Crate | Description |
|-------|-------------|
| `aql-core` | Parser, AST, Planner, Executor (zero DB deps) |
| `aql-nietzschedb` | NietzscheDB backend (full experience) |
| `aql-neo4j` | Neo4j backend (Cypher lowering) |
| `aql-qdrant` | Qdrant backend (vector search) |
| `aql-pgvector` | PostgreSQL+pgvector backend |
| `aql-redis` | Redis Stack backend |
| `aql-cli` | Interactive REPL |
| `aql-python` | Python bindings (PyO3) |
| `aql-wasm` | Browser/edge WASM build |

## Backend Capabilities

The full cognitive experience — hyperbolic geometry, dream cycles, energy model — **only exists with NietzscheDB**. Other backends function with graceful degradation.

## Build

```bash
cargo build --workspace
cargo test --workspace
```

## License

AGPL-3.0 — Jose R F Junior
