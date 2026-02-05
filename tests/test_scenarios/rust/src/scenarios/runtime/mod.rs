// *******************************************************************************
// Copyright (c) 2026 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
// *******************************************************************************
mod channel;
mod execution_engine;
mod net;
mod sleep;
mod worker;

use test_scenarios_rust::scenario::{ScenarioGroup, ScenarioGroupImpl};

use crate::scenarios::runtime::channel::channel_scenario_group;
use crate::scenarios::runtime::execution_engine::execution_engine_scenario_group;
use crate::scenarios::runtime::net::net_scenario_group;
use crate::scenarios::runtime::sleep::sleep_scenario_group;
use crate::scenarios::runtime::worker::worker_scenario_group;

pub fn runtime_scenario_group() -> Box<dyn ScenarioGroup> {
    Box::new(ScenarioGroupImpl::new(
        "runtime",
        vec![],
        vec![
            execution_engine_scenario_group(),
            sleep_scenario_group(),
            worker_scenario_group(),
            channel_scenario_group(),
            net_scenario_group(),
        ],
    ))
}
