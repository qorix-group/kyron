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
mod udp_client;
mod udp_server;

use udp_client::udp_client_group;
use udp_server::udp_server_group;

use test_scenarios_rust::scenario::{ScenarioGroup, ScenarioGroupImpl};

pub fn udp_scenario_group() -> Box<dyn ScenarioGroup> {
    Box::new(ScenarioGroupImpl::new(
        "udp",
        vec![],
        vec![udp_client_group(), udp_server_group()],
    ))
}
