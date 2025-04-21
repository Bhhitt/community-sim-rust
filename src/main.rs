use clap::Parser;
use community_sim::*;

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
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let agent_types = util::load_agent_types(&args.agent_types);
    if args.headless {
        println!("Running in headless mode");
        if args.scale {
            simulation::run_scaling_benchmarks();
        } else {
            simulation::run_profiles_from_yaml("sim_profiles.yaml", &agent_types, args.profile_systems);
        }
    } else {
        println!("Running with graphics");
        simulation::run_profile_from_yaml("sim_profiles.yaml", &args.profile, &agent_types, args.profile_systems);
    }
}
