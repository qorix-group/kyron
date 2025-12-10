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

#[cfg(not(any(test, feature = "runtime-api-mock")))]
use crate::scheduler::context::{ctx_get_schedule_safety, ctx_get_worker_id, ctx_set_schedule_safety};
use crate::scheduler::workers::worker_types::WorkerType;
#[cfg(any(test, feature = "runtime-api-mock"))]
use crate::testing::mock::{ctx_get_schedule_safety, ctx_get_worker_id, ctx_set_schedule_safety};

/// Contains functions to set/get a flag in worker's context.
/// The flag is used to schedule parent task of failing safety task into safety worker.
pub(crate) struct TaskContext {}

impl TaskContext {
    pub(crate) fn set_flag_to_wake_parent_task_into_safety() {
        ctx_set_schedule_safety(true);
    }

    pub(crate) fn clear_schedule_safety_flag() {
        ctx_set_schedule_safety(false);
    }

    pub(crate) fn should_wake_task_into_safety() -> bool {
        ctx_get_schedule_safety()
    }

    pub(crate) fn is_task_running_on_async_worker() -> bool {
        ctx_get_worker_id().typ() == WorkerType::Async
    }
}
