mod log_config;
mod util;
mod ecs; // <-- new idiomatic ECS module
mod simulation; // <-- old simulation module (to be migrated)
mod navigation;
mod graphics;
mod config;

pub mod agent;
pub mod map;
pub mod ecs_components;
pub mod food;
pub mod ecs_simulation;
pub mod render_ascii;

pub mod terrain;
pub mod sim_summary;
pub mod event_log;

use clap::Parser;
use chrono;
use fern;
use log;
use std::sync::{Arc, Mutex};

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
    /// Run YAML-driven benchmark profiles (benchmark: true)
    #[arg(long)]
    benchmark_profiles: bool,
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
fn setup_logging(log_level: log::LevelFilter, event_log: Option<Arc<Mutex<event_log::EventLog>>>) {
    let log_file = "community_sim.log";
    let mut dispatch = fern::Dispatch::new()
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
        .chain(fern::log_file(log_file).unwrap());

    if let Some(event_log) = event_log {
        dispatch = dispatch.chain(Box::new(crate::event_log::EventLogWriter::new(event_log)) as Box<dyn std::io::Write + Send>);
    }

    dispatch.apply().unwrap();
}

fn main() {
    let args = Args::parse();
    let log_level = parse_log_level(&args.log_level);
    let event_log = if args.headless {
        None
    } else {
        let event_log = Arc::new(Mutex::new(event_log::EventLog::new(200)));
        Some(event_log)
    };
    setup_logging(log_level, event_log.clone());
    let agent_types = util::load_agent_types(&args.agent_types);
    let log_config = log_config::LogConfig {
        quiet: args.log_quiet,
        stats: args.log_stats,
    };
    if args.headless {
        log::info!("Running in headless mode");
        if args.scale {
            // TODO: Move run_scaling_benchmarks to ecs module
            ecs::schedule::run_scaling_benchmarks(&agent_types);
        } else if args.benchmark_profiles {
            ecs::schedule::run_benchmark_profiles_from_yaml("config/sim_profiles.yaml", &agent_types, args.profile_systems, &args.profile_csv);
        } else {
            ecs::schedule::run_profiles_from_yaml("config/sim_profiles.yaml", &agent_types, args.profile_systems, &args.profile_csv);
        }
    } else {
        log::info!("Running with graphics");
        // TODO: Move run_profile_from_yaml to ecs module
        ecs::schedule::run_profile_from_yaml(
            "config/sim_profiles.yaml",
            &args.profile,
            &agent_types,
            args.profile_systems,
            &args.profile_csv,
            &log_config,
            event_log.expect("Event log should exist in GUI mode"),
        );
    }
}
