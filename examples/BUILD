# *******************************************************************************
# Copyright (c) 2025 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
# *******************************************************************************
load("@rules_rust//rust:defs.bzl", "rust_binary")

_EXAMPLE_DEPS = [
    "//src/kyron:libkyron",
    "//src/kyron-foundation:libkyron_foundation",
    "//src/logging_tracing:liblogging_tracing",
    "@score_crates//:libc",
    "@score_crates//:tracing_subscriber",
] + select({
    "//conditions:default": [],
    "//src/flags:enable_ipc": [
        "@score_crates//:iceoryx2_qnx8",
        "@score_crates//:iceoryx2_cal_qnx8",
    ],
})

rust_binary(
    name = "select",
    srcs = [
        "select.rs",
    ],
    proc_macro_deps = [
        "//src/kyron-macros:runtime_macros",
    ],
    visibility = ["//visibility:public"],
    deps = _EXAMPLE_DEPS,
)

rust_binary(
    name = "main_macro",
    srcs = [
        "main_macro.rs",
    ],
    proc_macro_deps = [
        "//src/kyron-macros:runtime_macros",
    ],
    visibility = ["//visibility:public"],
    deps = _EXAMPLE_DEPS,
)

rust_binary(
    name = "mpmc",
    srcs = [
        "mpmc.rs",
    ],
    proc_macro_deps = [
        "//src/kyron-macros:runtime_macros",
    ],
    visibility = ["//visibility:public"],
    deps = _EXAMPLE_DEPS,
)

rust_binary(
    name = "playground",
    srcs = [
        "playground.rs",
    ],
    aliases = select({
        "//conditions:default": {},
        "//src/flags:enable_ipc": {
            "@score_crates//:iceoryx2_qnx8": "iceoryx2",
            "@score_crates//:iceoryx2_cal_qnx8": "iceoryx2_cal",
        },
    }),
    proc_macro_deps = [
        "//src/kyron-macros:runtime_macros",
    ],
    visibility = ["//visibility:public"],
    deps = _EXAMPLE_DEPS,
)

rust_binary(
    name = "safety_task",
    srcs = [
        "safety_task.rs",
    ],
    proc_macro_deps = [
        "//src/kyron-macros:runtime_macros",
    ],
    visibility = ["//visibility:public"],
    deps = _EXAMPLE_DEPS,
)
