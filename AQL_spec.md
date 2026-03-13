# AQL — Agent Query Language
## Especificação Formal Completa v1.0

**Projecto:** NietzscheDB  
**Autor:** Jose R F Junior  
**Licença:** AGPL-3.0  
**Repositório:** github.com/JoseRFJuniorLLMs/NietzscheDB  
**Versão:** 1.0.0 — 2026

---

> *O NQL fala para quem lê. O NAQ fala para quem calcula. O AQL fala para quem pensa.*

---

## Índice

1. [Filosofia e Posicionamento](#1-filosofia-e-posicionamento)
2. [Stack Completa](#2-stack-completa)
3. [Layout da Crate](#3-layout-da-crate)
4. [Gramática Formal (Pest PEG)](#4-gramática-formal-pest-peg)
5. [Sistema de Tipos Epistémicos](#5-sistema-de-tipos-epistémicos)
6. [AST Completo](#6-ast-completo)
7. [Os 8 Verbos Cognitivos](#7-os-8-verbos-cognitivos)
8. [Qualifiers — Especificação Completa](#8-qualifiers--especificação-completa)
9. [Cognitive Planner](#9-cognitive-planner)
10. [Lowering AQL → NAQ (Mapeamento Completo)](#10-lowering-aql--naq-mapeamento-completo)
11. [State Management — WorkingMemory](#11-state-management--workingmemory)
12. [Resolução de @self](#12-resolução-de-self)
13. [Lógica de Upsert Epistémico (IMPRINT)](#13-lógica-de-upsert-epistémico-imprint)
14. [Cognitive Energy Model — Ganchos](#14-cognitive-energy-model--ganchos)
15. [Executor e Pipeline](#15-executor-e-pipeline)
16. [Erros e Protocolo de Resposta](#16-erros-e-protocolo-de-resposta)
17. [Exemplos Completos](#17-exemplos-completos)
18. [Testes de Aceitação](#18-testes-de-aceitação)
19. [Roadmap de Implementação](#19-roadmap-de-implementação)

---

## 1. Filosofia e Posicionamento

### O que o AQL NÃO é

O AQL **não é** uma query language com açúcar sintáctico.  
O AQL **não é** NAQ comprimido.  
O AQL **não é** NQL para agentes.

### O que o AQL É

O AQL é uma **Cognitive Intent Language** — uma linguagem que modela como um agente *usa* memória, não como a consulta.

A distinção é radical:

```
Query language:   O agente descreve COMO executar
Cognitive lang:   O agente descreve O QUE está a tentar pensar
```

Um humano que abre um terminal e escreve um query sabe o que quer. Um agente LLM tem uma *intenção* que precisa de ser executada no grafo hiperbólico. O AQL modela essa intenção directamente.

### Os três axiomas do AQL

**Axioma 1 — Intenção como primitiva:**  
A unidade atómica do AQL é um acto cognitivo, não uma instrução de base de dados.

**Axioma 2 — Incerteza como tipo de dados:**  
`CONFIDENCE 0.7` não é um filtro. É uma declaração epistémica sobre o estado do agente.

**Axioma 3 — Efeitos como consequências automáticas:**  
Cada acto cognitivo modifica o grafo de forma implícita. O agente declara a intenção; o servidor aplica os side-effects.

---

## 2. Stack Completa

```
┌─────────────────────────────────────┐
│         Humano / Dev                │
│         NQL (legível)               │
├─────────────────────────────────────┤
│         Agente LLM (precisão)       │
│         NAQ (compacto)              │
├─────────────────────────────────────┤
│         Agente LLM (cognição)       │
│         AQL ← esta crate            │
└─────────────────────────────────────┘
```

**Pipeline de execução:**

```
AQL source text
      │
      ▼
  [Parser] ← pest PEG grammar
      │
      ▼
  AQL AST (Statement, Verb, Subject, Qualifier)
      │
      ▼
  [CognitivePlanner]
  ├── escolhe estratégia por verb + context
  ├── resolve @self → colecção real
  ├── converte RECENCY → timestamp range
  ├── aplica lógica de upsert para IMPRINT
  └── verifica energia disponível (Energy Model hooks)
      │
      ▼
  ExecutionPlan
  ├── Vec<NaqInstruction>  ← instruções NAQ
  ├── Vec<SideEffect>      ← efeitos implícitos
  └── Option<Box<ExecutionPlan>>  ← THEN chain
      │
      ▼
  WorkingMemory (estado entre THEN)
      │
      ▼
  [NietzscheDB Executor]
      │
      ▼
  CognitiveResult
```

---

## 3. Layout da Crate

```
crates/nietzsche-aql/
│
├── Cargo.toml
│
└── src/
    ├── lib.rs                  ← re-exports + constantes de versão
    │
    ├── grammar.pest            ← gramática PEG completa e versionada
    │
    ├── types/
    │   ├── mod.rs
    │   ├── epistemic.rs        ← EpistemicType + metadados por tipo
    │   ├── recency.rs          ← RecencyDegree + conversão para timestamps
    │   ├── scope.rs            ← ContextScope + resolução de @self
    │   └── novelty.rs          ← NoveltyDegree
    │
    ├── ast/
    │   ├── mod.rs
    │   ├── statement.rs        ← Statement + chain
    │   ├── verb.rs             ← Verb + side-effects por verbo
    │   ├── subject.rs          ← Subject (Text, TypeFilter, SelfRef, TraceRange)
    │   └── qualifier.rs        ← Qualifier (todos os 8)
    │
    ├── parser/
    │   ├── mod.rs
    │   └── parser.rs           ← pest → AST
    │
    ├── planner/
    │   ├── mod.rs
    │   ├── cognitive_planner.rs ← estratégia adaptativa
    │   ├── lowering.rs          ← AQL → NaqInstruction (todos os 8 verbos)
    │   ├── imprint.rs           ← lógica de upsert epistémico
    │   ├── self_resolver.rs     ← resolução de @self
    │   └── energy_hooks.rs      ← ganchos para Cognitive Energy Model
    │
    ├── runtime/
    │   ├── mod.rs
    │   ├── executor.rs          ← execução do ExecutionPlan
    │   ├── working_memory.rs    ← estado entre THEN
    │   └── result.rs            ← CognitiveResult
    │
    └── error.rs                 ← AqlError + AqlErrorCode
```

---

## 4. Gramática Formal (Pest PEG)

A gramática é o contrato do protocolo. Cada símbolo tem um teste de parser associado. Mudanças de semântica requerem bump de versão.

```pest
// grammar.pest — AQL v1.0
// Versão do protocolo: byte 0x01 em cada query

// ─── Programa ────────────────────────────────────────────────────

program   = { SOI ~ statement+ ~ EOI }
statement = { verb ~ subject ~ qualifier* ~ chain? ~ NEWLINE* }

// ─── THEN chaining ───────────────────────────────────────────────

chain = { "THEN" ~ statement }

// ─── Verbos — os 8 actos cognitivos ─────────────────────────────

verb = {
    "RECALL"
  | "RESONATE"
  | "REFLECT"
  | "TRACE"
  | "IMPRINT"
  | "ASSOCIATE"
  | "DISTILL"
  | "FADE"
}

// ─── Subject ─────────────────────────────────────────────────────

subject = {
    self_ref
  | type_with_content    // Belief:"quantum"
  | type_filter          // Belief (sem conteúdo)
  | trace_range          // FROM "A" TO "B"
  | quoted_string        // "qualquer texto"
}

self_ref         = { "@self" }
type_filter      = { epistemic_type }
type_with_content = { epistemic_type ~ ":" ~ quoted_string }
trace_range      = { "FROM" ~ quoted_string ~ "TO" ~ quoted_string }

epistemic_type = {
    "Belief"
  | "Experience"
  | "Pattern"
  | "Signal"
  | "Intention"
}

// ─── Qualifiers ──────────────────────────────────────────────────

qualifier = {
    confidence_q
  | recency_q
  | depth_q
  | within_q
  | as_q
  | linking_q
  | novelty_q
  | limit_q
}

confidence_q = { "CONFIDENCE" ~ float }
recency_q    = { "RECENCY"    ~ recency_degree }
depth_q      = { "DEPTH"      ~ integer }
within_q     = { "WITHIN"     ~ scope }
as_q         = { "AS"         ~ epistemic_type }
linking_q    = { "LINKING"    ~ quoted_string }
novelty_q    = { "NOVELTY"    ~ novelty_degree }
limit_q      = { "LIMIT"      ~ integer }

recency_degree = { "fresh" | "recent" | "distant" | "ancient" }
novelty_degree = { "high"  | "medium" | "low" }

scope = {
    "session"
  | "collection"
  | "graph"
  | quoted_string        // named scope: "physics-collection"
}

// ─── Primitivos ──────────────────────────────────────────────────

quoted_string = ${ "\"" ~ inner ~ "\"" }
inner         = @{ (!("\"") ~ ANY)* }

float   = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
integer = @{ ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" | "\r" }
NEWLINE    = _{ "\n" }
COMMENT    = _{ "#" ~ (!"\n" ~ ANY)* ~ "\n" }
```

### Versionamento da gramática

Cada query AQL carrega um byte de versão implícito no AST. O parser rejeita queries de versões futuras com `AqlError::VersionMismatch`.

```rust
pub const AQL_VERSION: u8 = 1;
```

---

## 5. Sistema de Tipos Epistémicos

Os tipos do AQL não são taxonomia — são **epistemologia**. Cada tipo carrega propriedades intrínsecas que determinam comportamento no planner e no executor.

### EpistemicType

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EpistemicType {
    /// Proposição que o agente considera verdadeira.
    /// Propriedades: confidence [0..1], source, revision_count
    /// Decay: lento (beliefs são persistentes)
    /// NQL type: Semantic
    Belief,

    /// Evento vivido pelo agente numa sessão.
    /// Propriedades: timestamp, session_id, participants
    /// Decay: médio (memória episódica decai naturalmente)
    /// NQL type: Episodic
    Experience,

    /// Padrão emergente de múltiplas experiências.
    /// Propriedades: support_count, stability, generality
    /// Decay: muito lento (padrões são duráveis)
    /// NQL type: Semantic
    Pattern,

    /// Input de sensores ou contexto externo.
    /// Propriedades: noise_level, source_reliability, freshness
    /// Decay: rápido (sinais são voláteis)
    /// NQL type: Semantic
    Signal,

    /// Objectivo ou plano activo do agente.
    /// Propriedades: priority [0..1], status, deadline
    /// Decay: nenhum até conclusão
    /// NQL type: Concept
    Intention,
}
```

### Metadados por tipo

```rust
impl EpistemicType {
    /// Mapeamento para tipos NQL/NAQ.
    pub fn to_nql_type(&self) -> &'static str {
        match self {
            Self::Belief     => "Semantic",
            Self::Experience => "Episodic",
            Self::Pattern    => "Semantic",
            Self::Signal     => "Semantic",
            Self::Intention  => "Concept",
        }
    }

    /// Taxa de reforço de energia ao ser acedido.
    /// Padrões reforçam muito (confirmação de padrão).
    /// Sinais reforçam pouco (são substituídos por novos).
    pub fn access_energy_boost(&self) -> f32 {
        match self {
            Self::Belief     => 0.15,
            Self::Experience => 0.10,
            Self::Pattern    => 0.20,
            Self::Signal     => 0.05,
            Self::Intention  => 0.12,
        }
    }

    /// Taxa de decaimento de energia por ciclo.
    pub fn decay_rate(&self) -> f32 {
        match self {
            Self::Belief     => 0.005,
            Self::Experience => 0.015,
            Self::Pattern    => 0.002,
            Self::Signal     => 0.050,
            Self::Intention  => 0.000, // não decai até ser completada
        }
    }

    /// Energia inicial ao ser criado via IMPRINT.
    pub fn initial_energy(&self) -> f32 {
        match self {
            Self::Belief     => 0.70,
            Self::Experience => 0.65,
            Self::Pattern    => 0.80,
            Self::Signal     => 0.90, // sinais chegam com energia alta
            Self::Intention  => 0.75,
        }
    }
}
```

### Estruturas de metadados por tipo

```rust
pub struct BeliefMeta {
    pub confidence:     f32,   // [0.0, 1.0]
    pub source:         String,
    pub revision_count: u32,
}

pub struct ExperienceMeta {
    pub session_id:   String,
    pub timestamp:    i64,    // Unix millis
    pub participants: Vec<String>,
}

pub struct PatternMeta {
    pub support_count: u32,   // # de episódios que sustentam
    pub stability:     f32,   // quão consistente é o padrão
    pub generality:    f32,   // quão amplo é o domínio
}

pub struct SignalMeta {
    pub noise_level:       f32,
    pub source_reliability: f32,
}

pub struct IntentionMeta {
    pub priority: f32,
    pub status:   IntentionStatus,
}

pub enum IntentionStatus {
    Active,
    Pending,
    Completed,
    Abandoned,
}
```

---

## 6. AST Completo

```rust
/// Um programa AQL — sequência de statements.
pub struct Program {
    pub version:    u8,
    pub statements: Vec<Statement>,
}

/// Um acto cognitivo completo.
pub struct Statement {
    pub verb:       Verb,
    pub subject:    Subject,
    pub qualifiers: Vec<Qualifier>,
    /// THEN — próximo acto na cadeia.
    /// Recebe WorkingMemory do acto anterior.
    pub next:       Option<Box<Statement>>,
}

/// Os 8 actos cognitivos.
pub enum Verb {
    Recall,
    Resonate,
    Reflect,
    Trace,
    Imprint,
    Associate,
    Distill,
    Fade,
}

/// O alvo do acto cognitivo.
pub enum Subject {
    /// Texto semântico — o planner interpreta.
    Text(String),

    /// Filtro por tipo epistémico sem conteúdo.
    TypeFilter(EpistemicType),

    /// Filtro por tipo com conteúdo semântico.
    TypeWithContent {
        epistemic_type: EpistemicType,
        content:        String,
    },

    /// @self — o agente refere-se ao seu próprio estado.
    SelfRef,

    /// TRACE FROM "A" TO "B" — narrativa causal.
    TraceRange {
        from: String,
        to:   String,
    },
}

/// Todos os qualifiers disponíveis.
pub enum Qualifier {
    /// Threshold epistémico — não é um filtro duro.
    /// CONFIDENCE 0.8 significa "quero resultados
    /// com pelo menos 80% de relevância epistémica".
    Confidence(f32),

    /// Categoria temporal cognitiva.
    /// Convertida para timestamp range pelo Planner.
    Recency(RecencyDegree),

    /// Profundidade de diffusão/exploração no grafo.
    Depth(u8),

    /// Scope de execução da operação.
    Within(ContextScope),

    /// Cast explícito do tipo epistémico do resultado.
    As(EpistemicType),

    /// Associar o resultado/criação a outro conceito.
    Linking(String),

    /// Grau de novidade pretendido nos resultados.
    Novelty(NoveltyDegree),

    /// Limite de resultados.
    Limit(u32),
}
```

---

## 7. Os 8 Verbos Cognitivos

Cada verbo define: semântica, side-effects implícitos, e o que o Planner produz.

### RECALL

**Semântica:** Recuperar memória relevante e reforçar a energia dos nós acedidos.

```
RECALL "quantum physics"
RECALL "quantum physics" CONFIDENCE 0.8
RECALL Belief:"quantum" RECENCY recent LIMIT 5
```

**Side-effects implícitos:**
- `BoostAccessedNodes` — incrementa energy dos resultados
- `CreateTemporalEdge` — cria edge session → nó
- `RecordAccessPattern` — regista no planner adaptativo

**Planner output:** `VectorSearch` ou `EnergyThresholdScan` (veja Secção 10)

---

### RESONATE

**Semântica:** Encontrar por ressonância semântica. O servidor decide o método — pode ser KNN, diffusão, ou combinação. O agente não especifica o algoritmo.

```
RESONATE "consciousness emerges from complexity"
RESONATE "quantum entanglement" NOVELTY high DEPTH 3
```

**Side-effects implícitos:**
- `BoostAccessedNodes`
- `RecordResonancePattern` — para aprendizagem do planner

**Planner output:** `HybridSearch` (vector + graph neighbours)

---

### REFLECT

**Semântica:** Meta-cognição. O agente questiona o seu próprio estado ou o estado do grafo.

```
REFLECT @self
REFLECT @self WITHIN session
REFLECT Pattern WITHIN "physics-collection"
```

**Side-effects implícitos:**
- `RecordAccessPattern`

**Planner output:** `SessionIntrospection` ou `PageRank`

---

### TRACE

**Semântica:** Seguir uma narrativa causal entre dois conceitos. Não é um shortest path — é uma sequência de relações causais/temporais.

```
TRACE FROM "observation" TO "conclusion"
TRACE FROM "initial-hypothesis" TO "validated-theory" DEPTH 5
```

**Side-effects implícitos:**
- `BoostPathNodes`
- `CreateTemporalEdge`

**Planner output:** `CausalTrace`

---

### IMPRINT

**Semântica:** Escrever novo conhecimento no grafo. Inclui lógica de upsert epistémico (veja Secção 13).

```
IMPRINT "nova hipótese sobre colapso de função de onda"
IMPRINT "X é verdadeiro" CONFIDENCE 0.6 LINKING "quantum" AS Belief
```

**Side-effects implícitos:**
- `AssociateToSessionContext`
- `BoostLinkedNodes`

**Planner output:** `CreateWithAssociation` ou `UpsertBelief`

---

### ASSOCIATE

**Semântica:** Criar ou reforçar uma associação entre conceitos. Pode reforçar associações existentes (incrementa edge weight).

```
ASSOCIATE "quantum" LINKING "consciousness"
ASSOCIATE Belief:"entanglement" LINKING "non-locality" CONFIDENCE 0.9
```

**Side-effects implícitos:**
- `CreateTemporalEdge`
- `BoostLinkedNodes`

**Planner output:** `CreateOrReinforceEdge`

---

### DISTILL

**Semântica:** Extrair um Pattern de múltiplos episódios/experiências. Cria um novo nó Pattern que agrega os episódios fonte.

```
DISTILL Experience WITHIN session
DISTILL Experience:"física quântica" DEPTH 4 CONFIDENCE 0.7
```

**Side-effects implícitos:**
- `CreatePatternNode`
- `LinkSourceEpisodes`

**Planner output:** `PatternExtraction`

---

### FADE

**Semântica:** Esquecimento intencional. Reduz energy ou elimina nós abaixo de um threshold.

```
FADE Signal CONFIDENCE 0.2
FADE Experience RECENCY ancient WITHIN session
```

**Comportamento:** Se a energy após o decremento cair abaixo de 0.05, o nó é eliminado automaticamente.

**Side-effects implícitos:**
- `RecordFadeEvent` — para auditoria

**Planner output:** `EnergyDecrement` ou `PruneNodes`

---

## 8. Qualifiers — Especificação Completa

### CONFIDENCE

Não é um filtro de base de dados. É uma declaração epistémica.

```rust
pub fn confidence_to_energy_floor(confidence: f32) -> f32 {
    // Conversão assimétrica deliberada:
    // CONFIDENCE 0.8 → energy floor 0.4
    // O agente declara certeza; o servidor é mais permissivo
    confidence * 0.5
}

pub fn confidence_to_initial_energy(confidence: f32) -> f32 {
    // Para IMPRINT: confidence → energy do nó criado
    // CONFIDENCE 0.6 → energy 0.6 (1:1 para criação)
    confidence
}
```

### RECENCY — Conversão para timestamps reais

```rust
pub enum RecencyDegree {
    Fresh,    // < 5 minutos
    Recent,   // < 1 hora
    Distant,  // < 24 horas
    Ancient,  // sem limite
}

impl RecencyDegree {
    /// Converte para janela temporal em segundos.
    pub fn to_time_window_secs(&self) -> Option<i64> {
        match self {
            Self::Fresh   => Some(300),      // 5 min
            Self::Recent  => Some(3_600),    // 1 h
            Self::Distant => Some(86_400),   // 24 h
            Self::Ancient => None,
        }
    }

    /// Calcula o timestamp mínimo a partir de agora.
    pub fn to_min_timestamp(&self) -> Option<i64> {
        self.to_time_window_secs()
            .map(|w| chrono::Utc::now().timestamp_millis() - w * 1000)
    }

    /// Energy floor correspondente.
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

### WITHIN — Scopes

```rust
pub enum ContextScope {
    Session,              // nodes criados/acedidos nesta sessão
    Collection,           // collection padrão do tipo
    Graph,                // grafo completo
    Named(String),        // "physics-collection"
}

impl ContextScope {
    /// Converte para filtro NAQ.
    pub fn to_naq_filter(&self, session_id: &str) -> Option<NaqWhere> {
        match self {
            Self::Session => Some(NaqWhere {
                field: "session_id".into(),
                op:    NaqWhereOp::Eq,
                value: session_id.to_string(),
            }),
            Self::Named(name) => Some(NaqWhere {
                field: "collection".into(),
                op:    NaqWhereOp::Eq,
                value: name.clone(),
            }),
            Self::Collection | Self::Graph => None,
        }
    }
}
```

### NOVELTY

```rust
pub enum NoveltyDegree { High, Medium, Low }

impl NoveltyDegree {
    /// Distância mínima dos nós já acedidos nesta sessão.
    pub fn distance_floor(&self) -> f32 {
        match self {
            Self::High   => 0.70,
            Self::Medium => 0.40,
            Self::Low    => 0.10,
        }
    }
}
```

---

## 9. Cognitive Planner

O Planner é o intérprete da intenção. Recebe o AST e produz um `ExecutionPlan`.

### Estruturas principais

```rust
pub struct CognitivePlanner {
    session_id:      String,
    session_history: Vec<SessionEntry>,
    config:          PlannerConfig,
    self_resolver:   SelfResolver,
    energy_hooks:    EnergyHooks,
}

pub struct PlannerConfig {
    pub default_limit:             u32,   // 10
    pub default_knn_k:             u32,   // 10
    pub default_diffuse_depth:     u8,    // 3
    pub default_diffuse_threshold: f32,   // 0.5
    pub max_chain_depth:           u8,    // 8
    pub min_free_energy:           f32,   // 0.1 — abaixo disto, RESONATE é bloqueado
}

struct SessionEntry {
    verb:     Verb,
    strategy: Strategy,
    hit_rate: f32,
    timestamp: i64,
}
```

### Seleção de estratégia

```rust
impl CognitivePlanner {

    pub fn plan(&mut self, stmt: &Statement) -> Result<ExecutionPlan, AqlError> {
        // 1. Verificar energia disponível (Energy Model hook)
        self.energy_hooks.check_can_execute(&stmt.verb)?;

        // 2. Escolher estratégia
        let strategy = self.choose_strategy(stmt);

        // 3. Construir instruções NAQ
        let instructions = self.lower(stmt, &strategy)?;

        // 4. Coletar side-effects
        let side_effects = stmt.verb.implicit_side_effects();

        // 5. Planear chain recursivamente (com limite de profundidade)
        let chained = self.plan_chain(stmt)?;

        Ok(ExecutionPlan { instructions, side_effects, chained, strategy })
    }

    fn choose_strategy(&self, stmt: &Statement) -> Strategy {
        match stmt.verb {
            Verb::Reflect  => self.strategy_reflect(stmt),
            Verb::Trace    => Strategy::CausalTrace,
            Verb::Imprint  => Strategy::CreateWithAssociation,
            Verb::Fade     => Strategy::EnergyDecrement,
            Verb::Distill  => Strategy::PatternExtraction,
            Verb::Resonate => Strategy::HybridSearch,
            Verb::Recall | Verb::Associate => self.strategy_recall(stmt),
        }
    }

    fn strategy_recall(&self, stmt: &Statement) -> Strategy {
        // RECENCY → scan temporal
        if stmt.recency().is_some() {
            return Strategy::EnergyThresholdScan;
        }

        // CONFIDENCE alto + texto → KNN
        if stmt.confidence().map(|c| c > 0.6).unwrap_or(false) {
            if matches!(stmt.subject, Subject::Text(_)) {
                return Strategy::VectorSearch;
            }
        }

        // Adaptativo: se últimos 4 queries foram KNN, tenta scan
        let recent_knn = self.session_history.iter().rev().take(5)
            .filter(|e| e.strategy == Strategy::VectorSearch).count();

        if recent_knn >= 4 { Strategy::EnergyThresholdScan }
        else                { Strategy::VectorSearch }
    }

    fn strategy_reflect(&self, stmt: &Statement) -> Strategy {
        match &stmt.subject {
            Subject::SelfRef => Strategy::SessionIntrospection,
            _                => Strategy::PageRank,
        }
    }
}
```

---

## 10. Lowering AQL → NAQ (Mapeamento Completo)

### RECALL → VectorSearch ou EnergyThresholdScan

```
AQL:  RECALL "quantum physics" CONFIDENCE 0.8
NAQ:  K:S?c~"quantum physics"&e>0.4/10
      (KNN Semantic, filtro content, energy floor=conf*0.5, k=10)

AQL:  RECALL "quantum" RECENCY fresh LIMIT 5
NAQ:  M:S?c~"quantum"&e>0.7&t>now-300s>e-5
      (Match, energy floor fresh=0.7, timestamp window 5min, top 5 por energy)

AQL:  RECALL Belief:"quantum" RECENCY recent
NAQ:  M:S?c~"quantum"&e>0.4&t>now-3600s>e-10
      (Beliefs → Semantic, janela 1h)
```

### RESONATE → HybridSearch

```
AQL:  RESONATE "consciousness emerges from complexity"
NAQ:  K:S?c~"consciousness emerges from complexity"/10
      | X:result_id/0.5/2
      (KNN semântico → diffusão dos resultados top, weighted 0.7/0.3)

AQL:  RESONATE "quantum" NOVELTY high DEPTH 3
NAQ:  K:S?c~"quantum"/20
      | X:result_ids/0.7/3
      | filter(distance_from_session > 0.70)
```

### REFLECT → SessionIntrospection ou PageRank

```
AQL:  REFLECT @self
NAQ:  G:pagerank:session_nodes/10
      + M:S?session_id="<current>">t-10
      (PageRank dos nós da sessão + acedidos recentemente)

AQL:  REFLECT @self WITHIN session
NAQ:  M:*?session_id="<current>">e-20
      (todos os nós da sessão, por relevância)

AQL:  REFLECT Pattern
NAQ:  G:pagerank:S/10
      (PageRank sobre Semantic, proxy para Patterns)
```

### TRACE → CausalTrace

```
AQL:  TRACE FROM "observation" TO "conclusion"
NAQ:  P:"observation">>"conclusion"
      (shortest path com preferência por edges causais/temporais)

AQL:  TRACE FROM "A" TO "B" DEPTH 5
NAQ:  P:"A">>"B"/5
      (limite de profundidade 5)
```

### IMPRINT → CreateWithAssociation ou UpsertBelief

```
AQL:  IMPRINT "nova hipótese" CONFIDENCE 0.6
NAQ:  C:S{c:"nova hipótese",e:0.6}
      (cria Semantic, energy=confidence)

AQL:  IMPRINT "X é verdadeiro" CONFIDENCE 0.6 LINKING "quantum"
NAQ:  C:S{c:"X é verdadeiro",e:0.6}
      + C:edge{from:new_id,to:"quantum",type:Association,w:0.6}
      (cria nó + edge)

AQL:  IMPRINT "nova hipótese" AS Belief CONFIDENCE 0.6
NAQ:  C:S{c:"nova hipótese",e:0.6,meta:{type:"Belief",conf:0.6}}
```

### ASSOCIATE → CreateOrReinforceEdge

```
AQL:  ASSOCIATE "quantum" LINKING "consciousness"
NAQ:  U:edge{from:"quantum",to:"consciousness",type:Association}
      IF EXISTS: {w:+0.1}
      ELSE:      C:edge{from:"quantum",to:"consciousness",w:0.5}
      (upsert de edge)
```

### DISTILL → PatternExtraction

```
AQL:  DISTILL Experience WITHIN session
NAQ:  M:E?session_id="<current>">e-50        # recolhe episódios
      | G:louvain:result_ids/0.3              # agrupa por comunidade
      | C:S{c:cluster_summary,e:0.8,meta:{type:"Pattern"}}
      | C:edge{from:pattern_id,to:each_episode,type:Supports}
```

### FADE → EnergyDecrement ou PruneNodes

```
AQL:  FADE Signal CONFIDENCE 0.2
NAQ:  U:S?meta.type="Signal"&e<0.2{e:-0.3}
      + D:S?e<0.05!
      (decrementa; se cair abaixo de 0.05 → delete)

AQL:  FADE Experience RECENCY ancient
NAQ:  U:E?t<now-86400s{e:-0.2}
      + D:E?e<0.05!
```

---

## 11. State Management — WorkingMemory

Este é o buracos mais crítico do spec original. O `THEN` precisa de passar contexto entre actos cognitivos.

### WorkingMemory

```rust
/// Estado partilhado entre steps de uma cadeia THEN.
/// Contém os resultados do step anterior para informar o próximo.
pub struct WorkingMemory {
    /// IDs dos nós retornados pelo step anterior.
    pub result_node_ids:    Vec<String>,

    /// Conteúdo dos nós (para uso em queries subsequentes).
    pub result_contents:    Vec<String>,

    /// Energy média dos resultados (para calibração do próximo step).
    pub avg_energy:         f32,

    /// Nó criado pelo step anterior (para THEN ASSOCIATE).
    pub created_node_id:    Option<String>,

    /// Session ID — propagado por toda a cadeia.
    pub session_id:         String,

    /// Profundidade actual na cadeia (para limite anti-loop).
    pub chain_depth:        u8,
}

impl WorkingMemory {
    pub fn new(session_id: String) -> Self {
        Self {
            result_node_ids:  Vec::new(),
            result_contents:  Vec::new(),
            avg_energy:       0.0,
            created_node_id:  None,
            session_id,
            chain_depth:      0,
        }
    }

    /// Actualiza a memória com os resultados de um step.
    pub fn update_from_result(&mut self, result: &StepResult) {
        self.result_node_ids  = result.node_ids.clone();
        self.result_contents  = result.contents.clone();
        self.avg_energy       = result.avg_energy;
        self.created_node_id  = result.created_id.clone();
        self.chain_depth     += 1;
    }

    /// Injeta os IDs do step anterior no plano do próximo step.
    /// Ex: RECALL "A" THEN ASSOCIATE → ASSOCIATE actua sobre os IDs do RECALL.
    pub fn inject_into_plan(&self, plan: &mut ExecutionPlan) {
        if !self.result_node_ids.is_empty() {
            plan.context_node_ids = self.result_node_ids.clone();
        }
        if let Some(created) = &self.created_node_id {
            plan.context_created_id = Some(created.clone());
        }
    }
}
```

### Exemplo de pipeline THEN com WorkingMemory

```
RECALL "quantum"
THEN ASSOCIATE "quantum" LINKING "consciousness"
THEN IMPRINT "quantum-consciousness link confirmed" CONFIDENCE 0.7

Execução:
  Step 1: RECALL → WorkingMemory { result_ids: [q1, q2, q3], avg_energy: 0.82 }
  Step 2: ASSOCIATE actua sobre [q1, q2, q3] → cria edges com "consciousness"
          WorkingMemory { created_edge_ids: [e1, e2, e3] }
  Step 3: IMPRINT cria nó com LINKING automático aos edge_ids do Step 2
```

---

## 12. Resolução de @self

O `@self` não é apenas um marcador — é uma resolução dinâmica do estado do agente.

### SelfResolver

```rust
pub struct SelfResolver {
    session_id:        String,
    agent_id:          String,
    self_collection:   String, // ex: "agent_self_knowledge"
}

impl SelfResolver {
    /// Resolve @self para um plano de instrospection.
    pub fn resolve(&self, scope: Option<&ContextScope>) -> SelfResolution {
        match scope {
            Some(ContextScope::Session) => SelfResolution::SessionNodes {
                session_id: self.session_id.clone(),
            },
            Some(ContextScope::Collection) | None => SelfResolution::AgentCollection {
                collection: self.self_collection.clone(),
                agent_id:   self.agent_id.clone(),
            },
            Some(ContextScope::Graph) => SelfResolution::GraphWide {
                agent_id: self.agent_id.clone(),
            },
            Some(ContextScope::Named(name)) => SelfResolution::NamedCollection {
                collection: name.clone(),
            },
        }
    }
}

pub enum SelfResolution {
    /// Nodes acedidos/criados nesta sessão.
    SessionNodes { session_id: String },
    /// Collection dedicada ao conhecimento do agente.
    AgentCollection { collection: String, agent_id: String },
    /// Busca em todo o grafo com filtro por agent_id.
    GraphWide { agent_id: String },
    /// Collection nomeada explicitamente.
    NamedCollection { collection: String },
}
```

### Tradução para NAQ

```
REFLECT @self
→ SelfResolution::AgentCollection { collection: "agent_self_knowledge", agent_id: "eva-7" }
→ NAQ: G:pagerank:S?collection="agent_self_knowledge"&agent_id="eva-7"/10

REFLECT @self WITHIN session
→ SelfResolution::SessionNodes { session_id: "sess-abc123" }
→ NAQ: M:*?session_id="sess-abc123">e-20
```

---

## 13. Lógica de Upsert Epistémico (IMPRINT)

Este é o mecanismo que define se um agente é teimoso ou influenciável.

### Política de conflito

Quando `IMPRINT "X"` é executado e "X" já existe no grafo:

```rust
pub enum ConflictPolicy {
    /// Se existe com confidence maior → ignora o IMPRINT.
    /// O agente é conservador — só actualiza se a nova info for melhor.
    KeepHigherConfidence,

    /// Faz média ponderada das confidences.
    /// O agente integra nova evidência gradualmente.
    WeightedAverage { weight: f32 },

    /// Substitui sempre.
    /// O agente é influenciável — a informação mais recente ganha.
    ReplaceAlways,

    /// Cria um nó de conflito explícito.
    /// O agente regista a contradição para resolução futura.
    CreateConflict,
}
```

### Implementação

```rust
pub struct ImprintPlanner {
    policy: ConflictPolicy,
}

impl ImprintPlanner {
    pub fn plan_imprint(
        &self,
        content: &str,
        confidence: f32,
        existing: Option<ExistingNode>,
    ) -> NaqInstruction {
        match existing {
            None => {
                // Não existe → cria novo nó
                NaqInstruction::Create {
                    content:  content.to_string(),
                    energy:   confidence,
                    meta:     NodeMeta::belief(confidence),
                }
            }
            Some(node) => match &self.policy {
                ConflictPolicy::KeepHigherConfidence => {
                    if confidence > node.energy {
                        // Nova info é melhor → actualiza
                        NaqInstruction::Update {
                            id:     node.id,
                            energy: Some(confidence),
                        }
                    } else {
                        // Info existente é melhor → não faz nada
                        NaqInstruction::Noop
                    }
                }
                ConflictPolicy::WeightedAverage { weight } => {
                    let new_energy = node.energy * (1.0 - weight) + confidence * weight;
                    NaqInstruction::Update {
                        id:     node.id,
                        energy: Some(new_energy),
                    }
                }
                ConflictPolicy::ReplaceAlways => {
                    NaqInstruction::Update {
                        id:     node.id,
                        energy: Some(confidence),
                    }
                }
                ConflictPolicy::CreateConflict => {
                    // Cria nó de conflito com edge "conflicts_with"
                    NaqInstruction::CreateConflict {
                        existing_id:      node.id,
                        new_content:      content.to_string(),
                        new_confidence:   confidence,
                    }
                }
            }
        }
    }
}
```

### Política padrão por tipo epistémico

| Tipo | Política padrão | Razão |
|---|---|---|
| Belief | `WeightedAverage { weight: 0.3 }` | Integra nova evidência gradualmente |
| Experience | `ReplaceAlways` | Experiências são factos — não se contestam |
| Pattern | `WeightedAverage { weight: 0.1 }` | Padrões mudam devagar |
| Signal | `ReplaceAlways` | Sinais mais recentes substituem sempre |
| Intention | `KeepHigherConfidence` | Intenções são deliberadas |

---

## 14. Cognitive Energy Model — Ganchos

O AQL tem ganchos (hooks) para o Cognitive Energy Model — o próximo módulo do NietzscheDB. Estes ganchos estão presentes no código mas o modelo completo é implementado separadamente.

### EnergyHooks

```rust
pub struct EnergyHooks {
    /// Energia livre disponível para operações cognitivas.
    /// Abaixo de `min_free_energy`, certas operações são bloqueadas.
    free_energy: f32,
    config:      EnergyConfig,
}

pub struct EnergyConfig {
    /// Custo energético por verbo.
    pub costs: HashMap<Verb, f32>,
    /// Threshold mínimo para operações custosas.
    pub min_free_energy: f32,
}

impl Default for EnergyConfig {
    fn default() -> Self {
        let mut costs = HashMap::new();
        costs.insert(Verb::Recall,    0.02); // leitura simples
        costs.insert(Verb::Resonate,  0.10); // busca híbrida
        costs.insert(Verb::Reflect,   0.05); // meta-cognição
        costs.insert(Verb::Trace,     0.08); // path search
        costs.insert(Verb::Imprint,   0.15); // escrita
        costs.insert(Verb::Associate, 0.12); // criação de relação
        costs.insert(Verb::Distill,   0.25); // extracção de padrão — custoso
        costs.insert(Verb::Fade,      0.03); // esquecimento

        Self { costs, min_free_energy: 0.1 }
    }
}

impl EnergyHooks {
    /// Verifica se há energia suficiente para executar o verbo.
    pub fn check_can_execute(&self, verb: &Verb) -> Result<(), AqlError> {
        let cost = self.config.costs.get(verb).copied().unwrap_or(0.05);
        if self.free_energy < self.config.min_free_energy + cost {
            Err(AqlError::InsufficientCognitiveEnergy {
                required: cost,
                available: self.free_energy,
                verb: format!("{:?}", verb),
            })
        } else {
            Ok(())
        }
    }

    /// Debita energia após execução bem-sucedida.
    pub fn debit(&mut self, verb: &Verb) {
        let cost = self.config.costs.get(verb).copied().unwrap_or(0.05);
        self.free_energy = (self.free_energy - cost).max(0.0);
    }

    /// Credita energia (recuperação natural entre sessões).
    pub fn credit(&mut self, amount: f32) {
        self.free_energy = (self.free_energy + amount).min(1.0);
    }
}
```

### Como o Energy Model interage com o AQL

```
Estado sólido (free_energy < 0.2):
  → DISTILL bloqueado
  → RESONATE com DEPTH > 1 bloqueado
  → RECALL, FADE permitidos

Estado líquido (free_energy 0.2..0.7):
  → Todos os verbos permitidos
  → DISTILL com limite de DEPTH 3

Estado gasoso (free_energy > 0.7):
  → Todos os verbos sem restrições
  → RESONATE produz busca mais ampla
  → DISTILL com DEPTH irrestrito
```

---

## 15. Executor e Pipeline

### AqlExecutor

```rust
pub struct AqlExecutor {
    planner:      CognitivePlanner,
    naq_executor: NaqExecutor,      // interface para o DB engine
}

impl AqlExecutor {

    /// Executa um programa AQL completo.
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

    /// Executa um statement com o seu chain THEN.
    async fn execute_statement(
        &mut self,
        stmt: &Statement,
    ) -> Result<CognitiveResult, AqlError> {
        let mut memory = WorkingMemory::new(self.planner.session_id.clone());
        self.execute_chain(stmt, &mut memory).await
    }

    /// Executa recursivamente a cadeia THEN.
    async fn execute_chain(
        &mut self,
        stmt: &Statement,
        memory: &mut WorkingMemory,
    ) -> Result<CognitiveResult, AqlError> {
        // 1. Planear o step actual
        let mut plan = self.planner.plan(stmt)?;

        // 2. Injectar WorkingMemory do step anterior
        memory.inject_into_plan(&mut plan);

        // 3. Executar instruções NAQ
        let step_result = self.execute_plan(&plan).await?;

        // 4. Executar side-effects
        self.apply_side_effects(&plan.side_effects, &step_result).await?;

        // 5. Debitar energia
        self.planner.energy_hooks.debit(&stmt.verb);

        // 6. Actualizar WorkingMemory
        memory.update_from_result(&step_result);

        // 7. Executar THEN se existir (com limite de profundidade)
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
    /// Nós retornados pela operação.
    pub nodes:            Vec<CognitiveNode>,
    /// Número total de resultados (sem limite).
    pub total:            u32,
    /// Side-effects aplicados.
    pub effects_applied:  Vec<SideEffect>,
    /// Energia debitada.
    pub energy_debited:   f32,
    /// Plano de execução (para debug/auditoria).
    pub plan_label:       String,
}

pub struct CognitiveNode {
    pub id:             String,
    pub content:        String,
    pub energy:         f32,
    pub epistemic_type: Option<EpistemicType>,
    pub confidence:     Option<f32>,
}
```

---

## 16. Erros e Protocolo de Resposta

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
    InsufficientCognitiveEnergy {
        required:  f32,
        available: f32,
        verb:      String,
    },

    #[error("THEN chain exceeded max depth {0}")]
    ChainDepthExceeded(u8),

    #[error("version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: u8, got: u8 },

    #[error("planner error: {0}")]
    PlannerError(String),
}
```

### Formato de erro compacto (protocolo)

```json
{"err": 1, "pos": 0, "msg": "parse_error"}
{"err": 4, "pos": 2, "msg": "confidence_range"}
{"err": 8, "pos": 0, "msg": "insufficient_energy", "required": 0.25, "available": 0.08}
```

| Code | Significado |
|---|---|
| 1 | parse_error |
| 2 | unknown_verb |
| 3 | unknown_type |
| 4 | confidence_range |
| 5 | subject_mismatch |
| 6 | insufficient_energy |
| 7 | chain_depth_exceeded |
| 8 | version_mismatch |
| 9 | planner_error |

---

## 17. Exemplos Completos

### Exemplo 1 — Investigação cognitiva em cadeia

```aql
# O agente está a investigar física quântica
RECALL "quantum mechanics" CONFIDENCE 0.7 RECENCY recent
THEN RESONATE "consciousness emerges from complexity" NOVELTY high
THEN IMPRINT "quantum-consciousness resonance detected" CONFIDENCE 0.6 AS Belief LINKING "quantum"
THEN REFLECT @self WITHIN session
```

**Pipeline:**
```
RECALL → KNN Semantic, energy > 0.35, janela 1h → [q1, q2, q3]
  WorkingMemory: { ids: [q1,q2,q3], avg_energy: 0.78 }

RESONATE → HybridSearch sobre [q1,q2,q3] + "consciousness" → [c1, c2]
  (NOVELTY high → filtra nodes distantes da sessão)
  WorkingMemory: { ids: [c1,c2], avg_energy: 0.65 }

IMPRINT → cria Belief "quantum-consciousness resonance detected" energy=0.6
          + edge → "quantum" weight=0.6
  WorkingMemory: { created_id: "b1" }

REFLECT @self → PageRank da sessão → top 10 nodes mais influentes
```

---

### Exemplo 2 — Auto-aprendizagem com DISTILL

```aql
# O agente extrai padrões das suas experiências da sessão
RECALL Experience WITHIN session RECENCY fresh LIMIT 20
THEN DISTILL Experience DEPTH 3 CONFIDENCE 0.65
THEN ASSOCIATE @self LINKING "session-patterns" CONFIDENCE 0.8
```

---

### Exemplo 3 — Fade selectivo com Energy Model

```aql
# Limpeza cognitiva: remove sinais antigos e ruído baixo
FADE Signal RECENCY ancient CONFIDENCE 0.15
FADE Experience RECENCY distant CONFIDENCE 0.10 WITHIN session
```

**NAQ gerado:**
```
U:S?meta.type="Signal"&t<now-86400s&e<0.15{e:-0.3}
D:S?e<0.05!
U:E?session_id="sess-abc"&t<now-86400s&e<0.10{e:-0.3}
D:E?e<0.05!
```

---

### Exemplo 4 — TRACE narrativo

```aql
TRACE FROM "initial observation: anomalous readings" TO "hypothesis: quantum tunneling" DEPTH 4
```

**NAQ gerado:**
```
P:"initial observation: anomalous readings">>"hypothesis: quantum tunneling"/4
```

O executor procura o caminho de menor resistência (edges com maior weight), preferindo edges do tipo `Causal`, `Temporal`, e `Supports`.

---

### Exemplo 5 — REFLECT com @self completo

```aql
REFLECT @self
```

**Resolução:**
```
SelfResolution::AgentCollection {
  collection: "agent_self_knowledge",
  agent_id: "eva-7"
}

NAQ:
  G:pagerank:S?collection="agent_self_knowledge"&agent_id="eva-7"/10
  M:*?session_id="sess-abc">t-10

Resultado: {
  top_beliefs: [...],
  top_patterns: [...],
  active_intentions: [...],
  session_summary: {
    nodes_accessed: 34,
    nodes_created: 3,
    avg_energy_accessed: 0.71,
    verbs_used: ["RECALL", "RESONATE", "IMPRINT"]
  }
}
```

---

## 18. Testes de Aceitação

Cada feature tem um teste de aceitação canónico. Se o teste falha, a feature não está implementada.

```rust
// ─── Parser ──────────────────────────────────────────────────────

#[test]
fn parses_all_8_verbs() {
    for verb in ["RECALL","RESONATE","REFLECT","TRACE",
                 "IMPRINT","ASSOCIATE","DISTILL","FADE"] {
        let input = format!("{} \"test\"\n", verb);
        let prog = parse_aql(&input).unwrap();
        assert_eq!(prog.statements.len(), 1);
    }
}

#[test]
fn parses_all_qualifiers() {
    let q = r#"RECALL "test" CONFIDENCE 0.8 RECENCY recent DEPTH 3 WITHIN session AS Belief LINKING "other" NOVELTY high LIMIT 5
"#;
    let stmt = parse_one(q);
    assert_eq!(stmt.qualifiers.len(), 7);
}

#[test]
fn parses_trace_range() {
    let s = parse_one("TRACE FROM \"obs\" TO \"conc\"\n");
    assert!(matches!(s.subject, Subject::TraceRange { .. }));
}

#[test]
fn parses_type_with_content() {
    let s = parse_one("RECALL Belief:\"quantum\"\n");
    assert!(matches!(s.subject, Subject::TypeWithContent { .. }));
}

#[test]
fn rejects_confidence_out_of_range() {
    assert!(parse_aql("RECALL \"test\" CONFIDENCE 1.5\n").is_err());
}

// ─── Planner ─────────────────────────────────────────────────────

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
fn trace_uses_causal_trace() {
    let plan = plan_one("TRACE FROM \"A\" TO \"B\"\n");
    assert_eq!(plan.strategy, Strategy::CausalTrace);
}

#[test]
fn distill_uses_pattern_extraction() {
    let plan = plan_one("DISTILL Experience\n");
    assert_eq!(plan.strategy, Strategy::PatternExtraction);
}

// ─── Lowering ────────────────────────────────────────────────────

#[test]
fn recall_confidence_maps_to_energy_floor() {
    let plan = plan_one("RECALL \"test\" CONFIDENCE 0.8\n");
    let instr = &plan.instructions[0];
    // confidence 0.8 → energy floor 0.4 (0.8 * 0.5)
    let energy_filter = instr.where_clauses.iter()
        .find(|w| w.field == "e").unwrap();
    assert!((energy_filter.value.parse::<f32>().unwrap() - 0.4).abs() < 0.01);
}

#[test]
fn recency_fresh_maps_to_300s_window() {
    let plan = plan_one("RECALL \"test\" RECENCY fresh\n");
    let instr = &plan.instructions[0];
    let time_filter = instr.where_clauses.iter()
        .find(|w| w.field == "t").unwrap();
    assert!(time_filter.value.contains("300s"));
}

#[test]
fn imprint_confidence_maps_to_energy() {
    let plan = plan_one("IMPRINT \"nova ideia\" CONFIDENCE 0.65\n");
    let instr = &plan.instructions[0];
    assert!((instr.energy.unwrap() - 0.65).abs() < 0.01);
}

// ─── WorkingMemory ───────────────────────────────────────────────

#[test]
fn chain_passes_ids_to_next_step() {
    let program = "RECALL \"quantum\"\nTHEN ASSOCIATE \"quantum\" LINKING \"consciousness\"\n";
    // Simula execução com mock executor
    let result = run_mock(program).unwrap();
    // O ASSOCIATE deve ter recebido os IDs do RECALL
    assert!(!result.steps[1].context_node_ids.is_empty());
}

// ─── IMPRINT upsert ──────────────────────────────────────────────

#[test]
fn imprint_existing_higher_confidence_keeps_existing() {
    let planner = ImprintPlanner::new(ConflictPolicy::KeepHigherConfidence);
    let existing = ExistingNode { id: "n1".into(), energy: 0.9 };
    let instr = planner.plan_imprint("test", 0.6, Some(existing));
    assert!(matches!(instr, NaqInstruction::Noop));
}

#[test]
fn imprint_existing_lower_confidence_updates() {
    let planner = ImprintPlanner::new(ConflictPolicy::KeepHigherConfidence);
    let existing = ExistingNode { id: "n1".into(), energy: 0.4 };
    let instr = planner.plan_imprint("test", 0.8, Some(existing));
    assert!(matches!(instr, NaqInstruction::Update { .. }));
}

// ─── Energy hooks ────────────────────────────────────────────────

#[test]
fn distill_blocked_when_insufficient_energy() {
    let mut hooks = EnergyHooks::with_energy(0.05);
    let result = hooks.check_can_execute(&Verb::Distill);
    assert!(matches!(result, Err(AqlError::InsufficientCognitiveEnergy { .. })));
}

#[test]
fn recall_permitted_with_minimal_energy() {
    let mut hooks = EnergyHooks::with_energy(0.15);
    assert!(hooks.check_can_execute(&Verb::Recall).is_ok());
}
```

---

## 19. Roadmap de Implementação

### Fase 1 — Core (semanas 1–3)

- [ ] `grammar.pest` completa com todos os 8 verbos, todos os qualifiers, `THEN`
- [ ] Parser pest → AST para todos os casos
- [ ] `EpistemicType` com metadados completos
- [ ] `RecencyDegree::to_min_timestamp()` com `chrono`
- [ ] Testes de parser para todos os casos

### Fase 2 — Planner e Lowering (semanas 4–6)

- [ ] `CognitivePlanner` com todos os 8 estratégias
- [ ] Lowering completo AQL → NAQ para todos os 8 verbos
- [ ] `SelfResolver` com os 4 tipos de resolução
- [ ] `ImprintPlanner` com todas as políticas de conflito
- [ ] `ContextScope::to_naq_filter()`

### Fase 3 — Runtime (semanas 7–9)

- [ ] `WorkingMemory` com `inject_into_plan()`
- [ ] `AqlExecutor` com execução de chain
- [ ] `apply_side_effects()` para todos os 10 side-effects
- [ ] `CognitiveResult` estruturado

### Fase 4 — Energy Model hooks (semanas 10–11)

- [ ] `EnergyHooks` com custos por verbo
- [ ] Integração com o estado sólido/líquido/gasoso do NietzscheDB
- [ ] Testes de bloqueio por energia insuficiente

### Fase 5 — Testes de aceitação (semana 12)

- [ ] Todos os testes de aceitação da Secção 18 a verde
- [ ] Round-trip tests: AQL → NAQ → execução → resultado esperado
- [ ] Documentação de bytecodes como especificação testável

---

*AQL v1.0 — NietzscheDB — Jose R F Junior — 2026 — AGPL-3.0*  
*github.com/JoseRFJuniorLLMs/NietzscheDB*
