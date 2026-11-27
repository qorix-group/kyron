..
   # *******************************************************************************
   # Copyright (c) 2026 Contributors to the Eclipse Foundation
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

.. _kyron_user_manual:

Kyron User Manual
=================

.. document:: Kyron User Manual
   :id: doc__kyron_user_manual
   :status: draft
   :version: 1
   :safety: QM
   :security: YES
   :realizes: wp__module_user_manual[version==1]
   :belongs_to: mod__kyron[version==1]

Environment
-----------

* Operating systems: Linux (x86_64, aarch64) and QNX 8 (x86_64, aarch64).
* Toolchain: Rust (``rust-toolchain.toml``). Bazel builds use the Ferrocene toolchains from
  ``score_toolchains_rust``.

Dependencies
------------

Third-party Rust crates come from ``score_crates`` under Bazel and from ``Cargo.lock`` under
Cargo. IPC support depends on iceoryx2. It is enabled by default and controlled by the Bazel
flag ``//src/flags:k_enable_ipc``.

Integration
-----------

Add the module to ``MODULE.bazel``:

.. code-block:: python

   bazel_dep(name = "score_kyron", version = "<version>")

Then depend on ``@score_kyron//src/kyron:libkyron`` from a ``rust_library`` or ``rust_binary``.

Configuration and Usage
-----------------------

The runtime is built with ``RuntimeBuilder``. It consists of one or more execution engines,
each configured through ``ExecutionEngineBuilder``: number of async workers, task queue size,
thread priority and affinity, dedicated workers, and the safety worker. The
``#[kyron::main]`` macro sets up a runtime with default parameters.

Runnable examples are in ``src/kyron/examples`` and are run with ``bazel run //examples:<name>``
(``select``, ``main_macro``, ``mpmc``, ``safety_task``). See ``examples/README.md``.

Interface Documentation
-----------------------

The public API is documented in rustdoc (``cargo doc -p kyron --open``).

Safety
------

Kyron makes no ASIL claim in this release. An integrator must not rely on the safety worker
or any other Kyron feature to achieve a safety goal. See :ref:`kyron_qm_tailoring`.
