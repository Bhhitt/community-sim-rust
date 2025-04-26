mod log_config;
mod util;
mod ecs; // <-- new idiomatic ECS module
mod simulation; // <-- old simulation module (to be migrated)
mod navigation;
mod graphics;
mod config;
mod sim_profile;

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
use sim_profile::{SimProfile, load_profiles_from_yaml, find_profile};

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

    // --- Unified SimProfile logic ---
    let profile_path = "config/sim_profiles.yaml";
    let sim_profile = if args.profile != "small" || std::path::Path::new(profile_path).exists() {
        // If profile argument is set (not default) or file exists, try to load profile
        let profiles = load_profiles_from_yaml(profile_path);
        match find_profile(&profiles, &args.profile) {
            Some(profile) => Some(profile.clone()),
            None => {
                log::warn!("[WARN] Profile '{}' not found in {}. Falling back to CLI args.", &args.profile, profile_path);
                None
            }
        }
    } else {
        None
    };

    let (map_width, map_height, num_agents, ticks) = if let Some(profile) = &sim_profile {
        let width = profile.map_width.or(profile.map_size).unwrap_or(args.map_size);
        let height = profile.map_height.or(profile.map_size).unwrap_or(args.map_size);
        (width, height, profile.num_agents, profile.ticks)
    } else {
        (args.map_size, args.map_size, args.agents, args.ticks)
    };

    if args.headless {
        log::info!("Running in headless mode");
        // TODO: Refactor headless simulation to use SimProfile if present
        // Placeholder: print sim_profile info if loaded
        if let Some(profile) = &sim_profile {
            log::info!("[INFO] Loaded profile: {:?}", profile);
        }
        // (Headless simulation logic here)
    } else {
        log::info!("Running with graphics");
        let mut world = legion::World::default();
        let mut resources = legion::Resources::default();
        crate::graphics::sim_render::run_sim_render(
            sim_profile.as_ref().unwrap_or(&SimProfile {
                name: "cli_args".to_string(),
                map_width: Some(map_width),
                map_height: Some(map_height),
                map_size: None,
                num_agents,
                ticks,
                benchmark: None,
                quiet: None,
            }),
            &agent_types,
            args.profile_systems,
            &args.profile_csv,
            &mut world,
            &mut resources,
        );
    }
}
