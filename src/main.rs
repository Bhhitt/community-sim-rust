mod log_config;
mod simulation;
mod util;

use clap::Parser;
use chrono;
use fern;
use log;
use std::sync::{Arc, Mutex};

pub mod terrain;
pub mod sim_summary;
pub mod event_log;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run in headless mode (no graphics)
    #[arg(long)]
    headless: bool,
    /// Map size (NxN)
    #[arg(long, default_value_t = 20)]
    map_size: i32,
    /// Number of agents
    #[arg(long, default_value_t = 10)]
    agents: usize,
    /// Number of ticks
    #[arg(long, default_value_t = 10)]
    ticks: usize,
    /// Run scaling benchmarks
    #[arg(long)]
    scale: bool,
    /// YAML file for agent types
    #[arg(long, default_value = "config/agent_types.yaml")]
    agent_types: String,
    /// Simulation profile name (from config/sim_profiles.yaml)
    #[arg(long, default_value = "small")]
    profile: String,
    /// Enable ECS system profiling (timings, CSV output)
    #[arg(long)]
    profile_systems: bool,
    /// CSV file for system profiling output
    #[arg(long, default_value = "system_profile.csv")]
    profile_csv: String,
    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,
    /// Enable stats logs
    #[arg(long)]
    log_stats: bool,
    /// Enable eat logs
    #[arg(long)]
    log_eat: bool,
    /// Enable interact logs
    #[arg(long)]
    log_interact: bool,
    /// Enable quiet logs
    #[arg(long)]
    log_quiet: bool,
}

fn parse_log_level(level: &str) -> log::LevelFilter {
    match level.to_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    }
}

// Logging setup with fern
fn setup_logging(log_level: log::LevelFilter) {
    let log_file = "community_sim.log";
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .level_for("community_sim", log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file).unwrap())
        .apply()
        .unwrap();
}

fn main() {
    let args = Args::parse();
    let log_level = parse_log_level(&args.log_level);
    setup_logging(log_level);
    let agent_types = util::load_agent_types(&args.agent_types);
    let log_config = log_config::LogConfig {
        quiet: args.log_quiet,
        stats: args.log_stats,
        eat: args.log_eat,
        interact: args.log_interact,
    };
    // --- EventLog for piping logs to window ---
    let event_log = Arc::new(Mutex::new(event_log::EventLog::new(200)));
    // Register the custom logger (will pipe info logs to EventLog)
    let _ = event_log::EventLogLogger::init(event_log.clone(), log_level);
    // Insert event_log into ECS resources (if needed elsewhere)
    // ...
    if args.headless {
        log::info!("Running in headless mode");
        if args.scale {
            simulation::run_scaling_benchmarks(&agent_types);
        } else {
            simulation::run_profiles_from_yaml("config/sim_profiles.yaml", &agent_types, args.profile_systems, &args.profile_csv);
        }
    } else {
        log::info!("Running with graphics");
        simulation::run_profile_from_yaml(
            "config/sim_profiles.yaml",
            &args.profile,
            &agent_types,
            args.profile_systems,
            &args.profile_csv,
            &log_config,
            event_log.clone(), // Pass event_log down
        );
    }
}
