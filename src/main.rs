use clap::Parser;
use community_sim::*;
use chrono;
use fern;
use log;

pub mod terrain;

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
    #[arg(long, default_value = "agent_types.yaml")]
    agent_types: String,
    /// Simulation profile name (from sim_profiles.yaml)
    #[arg(long, default_value = "small")]
    profile: String,
    /// Enable ECS system profiling (timings, CSV output)
    #[arg(long)]
    profile_systems: bool,
    /// CSV file for system profiling output
    #[arg(long, default_value = "system_profile.csv")]
    profile_csv: String,
}

// Logging setup with fern
fn setup_logging() {
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
        .level(log::LevelFilter::Info)
        .level_for("community_sim", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file).unwrap())
        .apply()
        .unwrap();
}

fn main() {
    setup_logging();
    let args = Args::parse();
    let agent_types = util::load_agent_types(&args.agent_types);
    if args.headless {
        log::info!("Running in headless mode");
        if args.scale {
            simulation::run_scaling_benchmarks();
        } else {
            simulation::run_profiles_from_yaml("sim_profiles.yaml", &agent_types, args.profile_systems, &args.profile_csv);
        }
    } else {
        log::info!("Running with graphics");
        simulation::run_profile_from_yaml("sim_profiles.yaml", &args.profile, &agent_types, args.profile_systems, &args.profile_csv);
    }
}
