//
// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// <https://www.apache.org/licenses/LICENSE-2.0>
//
// SPDX-License-Identifier: Apache-2.0
//

use kyron::prelude::*;
use kyron::safety;
use kyron::scheduler::task::task_context::TaskContext;
use kyron::spawn_on_dedicated;
use kyron_foundation::prelude::*;

async fn failing_safety_task() -> Result<(), String> {
    info!(
        "Worker-N: failing_safety_task. Worker ID: {:?}, Task ID: {:?}",
        TaskContext::worker_id(),
        TaskContext::task_id().unwrap()
    );
    Err("Intentional failure".to_string())
}

async fn passing_safety_task() -> Result<(), String> {
    info!(
        "Worker-N: passing_safety_task. Worker ID: {:?}, Task ID: {:?}",
        TaskContext::worker_id(),
        TaskContext::task_id().unwrap()
    );
    Ok(())
}

async fn passing_non_safety_task() -> Result<(), String> {
    info!(
        "Dedicated worker (dw1): passing_non_safety_task. Worker ID: {:?}, Task ID: {:?}",
        TaskContext::worker_id(),
        TaskContext::task_id().unwrap()
    );
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_target(false) // Optional: Remove module path
        .with_max_level(Level::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // Create runtime
    let (builder, _engine_id) = kyron::runtime::RuntimeBuilder::new().with_engine(
        ExecutionEngineBuilder::new()
            .task_queue_size(256)
            .enable_safety_worker(ThreadParameters::default())
            .with_dedicated_worker("dw1".into(), ThreadParameters::default())
            .workers(2),
    );

    let mut runtime = builder.build().unwrap();
    // Put programs into runtime and run them
    runtime.block_on(async move {
        info!(
            "Parent task. Worker ID: {:?}, Task Id: {:?}",
            TaskContext::worker_id(),
            TaskContext::task_id().unwrap()
        );
        let handle1 = safety::spawn(failing_safety_task());
        let handle2 = safety::spawn(passing_safety_task());
        let handle3 = spawn_on_dedicated(passing_non_safety_task(), "dw1".into());

        info!("=============================== Spawned all tasks ===============================");

        let _ = handle1.await;
        info!("Since safety task fails, safety worker may execute parent task from this statement onwards.");

        info!(
            "Parent task. Worker ID: {:?}, Task Id: {:?}",
            TaskContext::worker_id(),
            TaskContext::task_id().unwrap()
        );
        let _ = handle2.await;
        let _ = handle3.await;

        info!("Program finished running.");
    });

    info!("Exit.");
}
