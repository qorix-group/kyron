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
"""Requirement linkage for component integration tests.

Under Bazel, ``score_py_pytest`` loads the S-CORE ``attribute_plugin``. Its ``add_test_properties``
decorator writes the requirement links (``PartiallyVerifies``/``FullyVerifies``), the test type and the
derivation technique into the JUnit XML, which docs-as-code links to the requirements.
Outside Bazel (for example ``run_tests.py``), the decorator is a no-op.
"""

from collections.abc import Callable
from typing import Any

try:
    from attribute_plugin import add_test_properties  # type: ignore[import-untyped]
except ImportError:

    def add_test_properties(*_args: Any, **_kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            return func

        return decorator


__all__ = ["add_test_properties"]
