..
   # *******************************************************************************
   # Copyright (c) 2024 Contributors to the Eclipse Foundation
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

.. _kyron_documentation:

Kyron
=====

This documentation describes **Kyron**, the functional safety async runtime for Rust, as an
S-CORE module. Its structure follows the
`S-CORE module template <https://eclipse-score.github.io/module_template/main/index.html>`_.
The feature requirements Kyron derives from are maintained in the
`S-CORE platform <https://eclipse-score.github.io/score/main/features/index.html>`_.

.. contents:: Table of Contents
   :depth: 2
   :local:

.. toctree::
   :maxdepth: 1
   :hidden:

   module/index
   components/index

Overview
--------

Kyron is an ``async/await`` runtime for Rust, built with Bazel and Cargo. Its configurable
execution engines give fine-grained control over scheduling, thread management and
workload isolation.

Safety Classification
---------------------

Kyron is designed for safety-critical use, with ASIL B as the long-term target.
Its current classification in the S-CORE release is **QM**. Kyron makes no ASIL claim
yet, and that includes the safety worker. The deferred safety work products and the
conditions that bring them back are listed in :ref:`kyron_qm_tailoring`.

Module Documentation
--------------------

The module view, the safety tailoring, the user manual and the release notes are described in
:ref:`kyron_module`.

.. needtable::
   :filter: docname is not None and docname.startswith("module/")
   :style: table
   :types: document
   :columns: title;id;safety;security;status
   :colwidths: 25,35,15,15,15
   :sort: title

Component Documentation
-----------------------

Requirements, architecture and detailed design are documented per component in
:ref:`kyron_components`.

.. needtable::
   :filter: docname is not None and docname.startswith("components/")
   :style: table
   :types: document
   :columns: title;id;safety;security;status
   :colwidths: 25,35,15,15,15
   :sort: title

Quick Start
-----------

To build the module for the host platform:

.. code-block:: bash

   bazel build //src/...

To build the module for QNX8:

.. code-block:: bash

   ./scripts/build_qnx8.sh

To run the unit tests:

.. code-block:: bash

   bazel test //:unit_tests

To run the component integration tests:

.. code-block:: bash

   ./scripts/run_component_tests.sh
