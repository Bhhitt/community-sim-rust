use legion::World;
use legion::IntoQuery;
use crate::agent::AgentType;
use legion::Resources;
use std::collections::HashMap;
use std::io::Write;
use crate::render_ascii;

/// Writes a simulation summary and ASCII snapshot to the given file path.
pub fn write_simulation_summary_and_ascii(
    world: &World,
    resources: &Resources,
    map: &crate::map::Map,
    tick: usize,
    output_path: &str,
) {
    let mut agent_type_counts: HashMap<String, usize> = HashMap::new();
    let mut agent_query = <(&AgentType,)>::query();
    for (agent_type,) in agent_query.iter(world) {
        *agent_type_counts.entry(agent_type.r#type.clone()).or_insert(0) += 1;
    }
    let stats = resources.get::<crate::ecs_components::InteractionStats>().expect("No InteractionStats resource");
    // EventLog is now in crate::event_log
    let total_interactions = stats.agent_interactions;
    let avg_interactions_per_tick = if tick > 0 { total_interactions as f64 / tick as f64 } else { 0.0 };
    let mut summary = String::new();
    summary.push_str(&format!("# Simulation Summary\n"));
    summary.push_str(&format!("Total interactions: {}\n", total_interactions));
    summary.push_str(&format!("Average interactions per tick: {:.2}\n", avg_interactions_per_tick));
    summary.push_str("Agent counts at end:\n");
    for (name, count) in agent_type_counts.iter() {
        summary.push_str(&format!("  {}: {}\n", name, count));
    }
    summary.push_str("\n");
    let ascii_snapshot = render_ascii::render_simulation_ascii(world, map);
    let mut file = std::fs::File::create(output_path).expect("Unable to create ascii output file");
    file.write_all(summary.as_bytes()).expect("Unable to write summary");
    file.write_all(ascii_snapshot.as_bytes()).expect("Unable to write ascii output");
    log::info!("[INFO] Simulation summary and final ASCII snapshot written to {}", output_path);
}
