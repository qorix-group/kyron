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

use crate::core::types::TaskId;
use crate::TaskRef;

// Thread-local storage to set/get running task in mock context
use ::core::cell::RefCell;
thread_local! {
    static MOCK_TASK_CTX: RefCell<Option<TaskRef>> = const { RefCell::new(None) };
}

///
/// Sets currently running `task`
///
pub(crate) fn ctx_set_running_task(task: TaskRef) {
    let _ = MOCK_TASK_CTX.try_with(|ctx| ctx.replace(Some(task)));
}

///
/// Clears currently running `task`
///
pub(crate) fn ctx_unset_running_task() {
    let _ = MOCK_TASK_CTX.try_with(|ctx| ctx.replace(None));
}

///
/// Returns `true` if the running task resulted in safety error
///
pub(crate) fn ctx_get_task_safety_error() -> bool {
    MOCK_TASK_CTX
        .try_with(|ctx| ctx.borrow().as_ref().is_some_and(|task| task.get_task_safety_error()))
        .unwrap_or_default()
}

///
/// Gets currently running `task id`
///
pub(crate) fn ctx_get_running_task_id() -> Option<TaskId> {
    MOCK_TASK_CTX
        .try_with(|ctx| ctx.borrow().as_ref().map(|task| task.id()))
        .unwrap_or_default()
}
