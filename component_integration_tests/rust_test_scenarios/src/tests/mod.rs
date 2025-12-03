use test_scenarios_rust::scenario::{ScenarioGroup, ScenarioGroupImpl};

mod runtime;

use runtime::runtime_scenario_group;

pub fn root_scenario_group() -> Box<dyn ScenarioGroup> {
    Box::new(ScenarioGroupImpl::new(
        "root",
        vec![],
        vec![basic_scenario_group(), orchestration_scenario_group(), runtime_scenario_group()],
    ))
}
