//! AQL CLI — Interactive REPL for cognitive queries.
//!
//! Usage:
//!   aql-cli                           # Interactive REPL
//!   aql-cli --backend nietzschedb     # Connect to NietzscheDB
//!   aql-cli -e 'RECALL "quantum"'     # Execute single query

use aql_core::{parser, planner::{CognitivePlanner, PlannerConfig}, executor::AqlExecutor, plans::ExecutionPlan};
use aql_nietzschedb::NietzscheBackend;
use clap::Parser as ClapParser;
use colored::*;
use rustyline::DefaultEditor;
use std::sync::Arc;

#[derive(ClapParser)]
#[command(name = "aql", version = aql_core::VERSION, about = "AQL — Agent Cognition Language")]
struct Cli {
    /// Backend to connect to
    #[arg(short, long, default_value = "nietzschedb")]
    backend: String,

    /// Backend endpoint
    #[arg(short, long, default_value = "https://136.111.0.47:443")]
    endpoint: String,

    /// Collection to use
    #[arg(short, long)]
    collection: Option<String>,

    /// Execute a single AQL expression
    #[arg(short = 'e', long)]
    execute: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let backend: Arc<dyn aql_core::AqlBackend> = match cli.backend.as_str() {
        "nietzschedb" | "ndb" => {
            let mut b = NietzscheBackend::new(&cli.endpoint);
            if let Some(ref col) = cli.collection {
                b = b.with_collection(col);
            }
            Arc::new(b)
        }
        other => {
            eprintln!("{}: Unknown backend '{}'. Available: nietzschedb, neo4j, qdrant, pgvector, redis",
                "Error".red().bold(), other);
            std::process::exit(1);
        }
    };

    let mut executor = AqlExecutor::new(backend.clone());
    let mut planner = CognitivePlanner::new(PlannerConfig {
        active_collection: cli.collection,
        ..Default::default()
    });

    // Single expression mode
    if let Some(ref expr) = cli.execute {
        return execute_and_print(expr, &mut planner, &mut executor).await;
    }

    // Interactive REPL
    println!("{}", "╔══════════════════════════════════════════════╗".cyan());
    println!("{}", "║   AQL — Agent Cognition Language v2.0       ║".cyan());
    println!("{}", "║   Backend: NietzscheDB (full experience)    ║".cyan());
    println!("{}", "║   Type 'help' or 'quit' to exit             ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════╝".cyan());
    println!();

    let mut rl = DefaultEditor::new()?;
    let prompt = format!("{} ", "aql>".green().bold());

    loop {
        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                match line {
                    "quit" | "exit" | "q" => break,
                    "help" | "h" => print_help(),
                    "verbs" => print_verbs(),
                    "qualifiers" => print_qualifiers(),
                    _ => {
                        if let Err(e) = execute_and_print(line, &mut planner, &mut executor).await {
                            eprintln!("{}: {}", "Error".red().bold(), e);
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{}: {}", "Error".red().bold(), e);
                break;
            }
        }
    }

    println!("{}", "Goodbye.".dimmed());
    Ok(())
}

async fn execute_and_print(
    input: &str,
    planner: &mut CognitivePlanner,
    executor: &mut AqlExecutor,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse
    let program = parser::parse(input)?;
    println!("{} Parsed {} statement(s)", "✓".green(), program.statements.len());

    // Plan
    let plans = planner.plan_program(&program)?;
    println!("{} Planned {} execution plan(s)", "✓".green(), plans.len());

    // Execute
    for plan in &plans {
        let result = executor.execute(plan).await?;
        println!(
            "{} {} nodes, {} edges ({}ms) [{}]",
            "→".cyan(),
            result.metadata.count,
            result.edges.len(),
            result.metadata.execution_time_ms,
            result.metadata.backend,
        );

        for node in &result.nodes {
            println!(
                "  {} {} (energy: {:.2}, confidence: {:.2})",
                "│".dimmed(),
                node.content.yellow(),
                node.energy,
                node.confidence,
            );
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", "AQL Commands:".bold());
    println!("  {}    — Show this help", "help".green());
    println!("  {}   — List all 13 cognitive verbs", "verbs".green());
    println!("  {} — List all qualifiers", "qualifiers".green());
    println!("  {}    — Exit the REPL", "quit".green());
    println!();
    println!("{}", "Example Queries:".bold());
    println!("  RECALL \"quantum physics\" CONFIDENCE 0.8");
    println!("  RESONATE \"consciousness\" NOVELTY high DEPTH 3");
    println!("  IMPRINT \"new discovery\" AS Belief CONFIDENCE 0.9");
    println!("  TRACE FROM \"hypothesis\" TO \"conclusion\" DEPTH 5");
    println!("  DESCEND \"physics\" DEPTH 3 MAGNITUDE 0.3..0.7");
    println!("  DREAM ABOUT \"quantum consciousness\"");
}

fn print_verbs() {
    println!("{}", "The 13 Cognitive Verbs:".bold());
    println!("  {} — Retrieve relevant memory", "RECALL".cyan());
    println!("  {} — Semantic resonance search", "RESONATE".cyan());
    println!("  {} — Meta-cognition about self/graph", "REFLECT".cyan());
    println!("  {} — Follow causal path between concepts", "TRACE".cyan());
    println!("  {} — Write new knowledge", "IMPRINT".cyan());
    println!("  {} — Create/reinforce association", "ASSOCIATE".cyan());
    println!("  {} — Extract patterns from episodes", "DISTILL".cyan());
    println!("  {} — Intentional forgetting", "FADE".cyan());
    println!("  {} — Navigate deeper in hierarchy", "DESCEND".cyan());
    println!("  {} — Navigate to abstractions", "ASCEND".cyan());
    println!("  {} — Find peers at same depth", "ORBIT".cyan());
    println!("  {} — Creative dream cycle", "DREAM".cyan());
    println!("  {} — Counterfactual reasoning", "IMAGINE".cyan());
}

fn print_qualifiers() {
    println!("{}", "Qualifiers:".bold());
    println!("  {} 0.8     — Epistemic certainty", "CONFIDENCE".cyan());
    println!("  {} fresh   — Temporal window", "RECENCY".cyan());
    println!("  {} 5       — Traversal depth", "DEPTH".cyan());
    println!("  {} session — Scope context", "WITHIN".cyan());
    println!("  {} Belief  — Cast type", "AS".cyan());
    println!("  {} \"X\"   — Link to concept", "LINKING".cyan());
    println!("  {} high    — Novelty preference", "NOVELTY".cyan());
    println!("  {} 10      — Max results", "LIMIT".cyan());
    println!("  {} 0.3..0.7 — Hyperbolic depth", "MAGNITUDE".cyan());
    println!("  {} high    — Cluster density", "CURVATURE".cyan());
    println!("  {} 0.1     — Orbit radius", "RADIUS".cyan());
    println!("  {} positive — Emotion polarity", "VALENCE".cyan());
    println!("  {} high    — Activation level", "AROUSAL".cyan());
    println!("  {} creative — Planner mood", "MOOD".cyan());
    println!("  {} 10      — Observation count", "EVIDENCE".cyan());
}
