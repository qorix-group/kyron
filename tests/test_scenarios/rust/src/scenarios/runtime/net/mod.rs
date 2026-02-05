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
mod tcp;
mod udp;

use crate::scenarios::runtime::net::tcp::tcp_scenario_group;
use crate::scenarios::runtime::net::udp::udp_scenario_group;

use test_scenarios_rust::scenario::{ScenarioGroup, ScenarioGroupImpl};

pub fn net_scenario_group() -> Box<dyn ScenarioGroup> {
    Box::new(ScenarioGroupImpl::new("net", vec![], vec![tcp_scenario_group(), udp_scenario_group()]))
}
