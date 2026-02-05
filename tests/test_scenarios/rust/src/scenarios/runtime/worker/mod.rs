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
mod dedicated_worker;
mod num_workers;
mod safety_worker;
mod spawn_methods;
mod thread_affinity;
mod thread_priority;
mod worker_basic;
mod worker_with_blocking_tasks;

use dedicated_worker::dedicated_worker_group;
use num_workers::NumWorkers;
use safety_worker::safety_worker_group;
use spawn_methods::spawn_methods_group;
use test_scenarios_rust::scenario::{ScenarioGroup, ScenarioGroupImpl};
use thread_affinity::ThreadAffinity;
use thread_priority::ThreadPriority;
use worker_basic::BasicWorker;
use worker_with_blocking_tasks::WorkerWithBlockingTasks;

pub fn worker_scenario_group() -> Box<dyn ScenarioGroup> {
    Box::new(ScenarioGroupImpl::new(
        "worker",
        vec![
            Box::new(BasicWorker),
            Box::new(WorkerWithBlockingTasks),
            Box::new(NumWorkers),
            Box::new(ThreadAffinity),
            Box::new(ThreadPriority),
        ],
        vec![dedicated_worker_group(), safety_worker_group(), spawn_methods_group()],
    ))
}
