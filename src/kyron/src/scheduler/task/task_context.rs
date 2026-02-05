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

use crate::{
    core::types::TaskId,
    scheduler::{
        context::{ctx_get_running_task_id, ctx_get_worker_id, ctx_set_running_task, ctx_unset_running_task},
        workers::worker_types::WorkerId,
    },
    TaskRef,
};

/// Provides access to context information about the currently executing task.
pub struct TaskContext {}

impl TaskContext {
    /// Get the worker id of the currently executing task. Usage of this outside runtime causes panic.
    pub fn worker_id() -> WorkerId {
        ctx_get_worker_id()
    }

    /// Get the TaskId of the currently executing task, if any. Usage of this outside runtime causes panic.
    pub fn task_id() -> Option<TaskId> {
        ctx_get_running_task_id()
    }
}

/// A guard that sets the task on creation and unsets it on drop.
pub(crate) struct TaskContextGuard {}

impl TaskContextGuard {
    /// Sets the given task for the current context.
    pub(crate) fn new(task: TaskRef) -> Self {
        ctx_set_running_task(task);
        Self {}
    }
}

impl Drop for TaskContextGuard {
    fn drop(&mut self) {
        ctx_unset_running_task();
    }
}
