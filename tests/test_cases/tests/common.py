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
import logging

logger = logging.getLogger(__name__)

try:
    from attribute_plugin import add_test_properties  # type: ignore[import-untyped]
except ImportError:
    # Define no-op decorator if attribute_plugin is not available (outside bazel)
    # Keeps IDE debugging functionality
    logger.info("attribute_plugin import failed. Defining no-op add_test_properties decorator")
    import os

    is_bazel_used = any(env_var.startswith("BAZEL_") for env_var in os.environ)
    if is_bazel_used:
        logger.warning("Bazel usage detected but attribute_plugin could not be imported.")

    def add_test_properties(*args, **kwargs):  # noqa: ARG001
        def decorator(func):
            return func  # No-op decorator

        return decorator
