#![doc = include_str!("../README.md")]

mod closure;
mod firstset;
mod grammar;
mod lalr;
mod lr;
mod phrase;
mod preprocess;
mod state;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use itertools::Itertools;
use serde::Serialize;
use tracing_subscriber::filter::EnvFilter;

use firstset::FirstSet;
use grammar::Grammar;
use lalr::LalrProblem;
use lalr::LookaheadTable;
use state::Automaton;

#[derive(Parser)]
#[command(author, version, about)]
struct Opt {
    /// Logging format.
    #[arg(
        short,
        long,
        value_enum,
        env = "BEE_LOG_FORMAT",
        default_value = "text"
    )]
    log_format: LogFormat,

    /// Enable reporting.
    #[arg(short, long)]
    report_dir: Option<PathBuf>,

    /// A path to an YAML file defining the syntactic grammar.
    #[arg()]
    grammar: PathBuf,

    /// Goal symbols.
    #[arg()]
    goal_symbols: Vec<String>,
}

#[derive(Clone, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    match opt.log_format {
        LogFormat::Text => {
            tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
    }

    let now = Instant::now();

    tracing::info!("Loading the grammar...");
    let production_rules = serde_yaml::from_reader(File::open(&opt.grammar)?)?;
    let grammar = Grammar::new(production_rules);
    grammar.validate();

    // We must create the augmented grammar before preprocessing.
    let augmented_grammar = grammar.create_augmented_grammar(&opt.goal_symbols);
    augmented_grammar.validate();

    // Preprocess the syntactic grammar for making subsequent translations easier.
    // The ECMA-262 specification uses non-tail lookahead notations.
    tracing::info!("Preprocessing the grammar...");
    let preprocessed_grammar = preprocess::preprocess(&augmented_grammar);
    preprocessed_grammar.validate();
    if let Some(ref dir) = opt.report_dir {
        report_preprocessed_grammar(dir, &preprocessed_grammar)?;
    }

    // Check the maximum number of lookahead tokens in the grammar.
    let max_lookahead_tokens = preprocessed_grammar.max_lookahead_tokens();
    if max_lookahead_tokens > 1 {
        tracing::error!(max_lookahead_tokens, "The grammar is not LALR(1)");
        std::process::exit(1);
    }

    tracing::info!("Collecting the first set of each non-terminal symbol...");
    // The collected sets will be used in computation of closure of an LR item set.
    let first_set = firstset::collect_first_set(&preprocessed_grammar, 1);
    if let Some(ref dir) = opt.report_dir {
        report_first_set(dir, &first_set)?;
    }

    tracing::info!("Building LR(0) automaton...");
    let automaton = state::build_lr0_automaton(&preprocessed_grammar, &first_set);
    tracing::info!("The size of the LR(0) automaton: {}", automaton.size());
    if let Some(ref dir) = opt.report_dir {
        report_lr0_automaton(dir, &automaton)?;
    }

    tracing::info!("Building a lookahead table for each LR(0) state...");
    let lookahead_tables =
        lalr::build_lookahead_tables(&preprocessed_grammar, &first_set, &automaton);
    if let Some(ref dir) = opt.report_dir {
        report_lalr_lookahead_tables(dir, &lookahead_tables)?;
    }

    tracing::info!("Building LALR(1) states...");
    let (lalr1_states, problems) = lalr::build_lalr_states(&automaton, &lookahead_tables);
    if let Some(ref dir) = opt.report_dir {
        report_lalr_problems(dir, &problems)?;
    }
    if !problems.is_empty() {
        tracing::error!("Problems occur while generating LALR(1) parsing tables");
        std::process::exit(1);
    }

    tracing::info!(elapsed = %humantime::format_duration(now.elapsed()), "Done");

    serde_json::to_writer(
        std::io::stdout(),
        &lalr::LalrSpec {
            goal_symbols: opt.goal_symbols,
            non_terminals: augmented_grammar
                .non_terminals()
                .filter(|non_terminal| !non_terminal.is_goal_of_augmented_grammar())
                .map(|non_terminal| non_terminal.symbol())
                .unique()
                .map(|symbol| symbol.to_owned())
                .sorted()
                .collect(),
            production_rules: augmented_grammar
                .rules()
                .iter()
                .filter(|rule| !rule.is_goal_of_augmented_grammar())
                .map(|rule| format!("{rule}"))
                .collect(),
            starts: automaton
                .start_states()
                .map(|(symbol, state)| (symbol.to_owned(), state.id.index()))
                .collect(),
            states: lalr1_states,
        },
    )?;

    Ok(())
}

// reporters

fn report_preprocessed_grammar(dir: &PathBuf, grammar: &Grammar) -> Result<()> {
    let rules = grammar
        .rules()
        .iter()
        .map(|rule| RuleReport {
            name: format!("{}", rule.name),
            production: rule
                .production
                .iter()
                .map(|term| format!("{term}"))
                .join(" "),
        })
        .collect_vec();
    let path = dir.join("preprocessed.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &rules)?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct RuleReport {
    name: String,
    production: String,
}

fn report_first_set(dir: &PathBuf, first_set: &FirstSet) -> Result<()> {
    let report = FirstSetReport {
        max_tokens: first_set.max_tokens,
        entries: first_set
            .table
            .iter()
            .map(|(non_terminal, phrase_set)| FirstSetEntryReport {
                non_terminal: format!("{non_terminal}"),
                first_set: phrase_set
                    .iter()
                    .map(|phrase| format!("{phrase}"))
                    .collect_vec(),
            })
            .collect_vec(),
    };
    let path = dir.join("first_set.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &report)?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct FirstSetReport {
    max_tokens: usize,
    entries: Vec<FirstSetEntryReport>,
}

#[derive(Debug, Serialize)]
struct FirstSetEntryReport {
    non_terminal: String,
    first_set: Vec<String>,
}

fn report_lr0_automaton(dir: &PathBuf, automaton: &Automaton) -> Result<()> {
    let report = Lr0AutomatonReport {
        starts: automaton
            .starts
            .iter()
            .map(|(symbol, id)| (symbol.to_owned(), format!("State({})", id.index())))
            .collect_vec(),
        states: automaton
            .states
            .iter()
            .map(|state| StateReport {
                state: format!("State({})", state.id.index()),
                kernel_items: state
                    .internal_kernel_items()
                    .map(|item| format!("{item}"))
                    .collect_vec(),
                non_kernel_items: state
                    .internal_non_kernel_items()
                    .map(|item| format!("{item}"))
                    .collect_vec(),
                transitions: state
                    .transitions
                    .iter()
                    .map(|(symbol, next_id)| TransitionReport {
                        symbol: format!("{symbol}"),
                        next_state: format!("State({})", next_id.index()),
                    })
                    .collect_vec(),
            })
            .collect_vec(),
    };
    let path = dir.join("lr0_automaton.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &report)?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct Lr0AutomatonReport {
    starts: Vec<(String, String)>,
    states: Vec<StateReport>,
}

#[derive(Debug, Serialize)]
struct StateReport {
    state: String,
    kernel_items: Vec<String>,
    non_kernel_items: Vec<String>,
    transitions: Vec<TransitionReport>,
}

#[derive(Debug, Serialize)]
struct TransitionReport {
    symbol: String,
    next_state: String,
}

fn report_lalr_lookahead_tables(dir: &PathBuf, lookahead_tables: &[LookaheadTable]) -> Result<()> {
    let report = lookahead_tables
        .iter()
        .enumerate()
        .map(|(i, table)| LookaheadTableReport {
            state: format!("State({i})"),
            entries: table
                .iter()
                .map(|(item, phrase_set)| LookaheadReport {
                    item: format!("{item}"),
                    lookaheads: phrase_set
                        .iter()
                        .map(|phrase| format!("{phrase}"))
                        .collect_vec(),
                })
                .collect_vec(),
        })
        .collect_vec();
    let path = dir.join("lalr_lookahead_tables.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &report)?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct LookaheadTableReport {
    state: String,
    entries: Vec<LookaheadReport>,
}

#[derive(Debug, Serialize)]
struct LookaheadReport {
    item: String,
    lookaheads: Vec<String>,
}

fn report_lalr_problems(dir: &PathBuf, problems: &[LalrProblem]) -> Result<()> {
    let path = dir.join("problems.json");
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, problems)?;
    Ok(())
}
