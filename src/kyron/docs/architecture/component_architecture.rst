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

.. _kyron_runtime_architecture:

Kyron Runtime Component Architecture
====================================

.. document:: Kyron Runtime Component Architecture
   :id: doc__kyron_runtime_architecture
   :status: draft
   :version: 1
   :safety: QM
   :security: YES
   :realizes: wp__component_arch[version==1]

.. comp:: Kyron Runtime
   :id: comp__kyron_runtime
   :security: YES
   :safety: QM
   :status: valid
   :version: 1
   :belongs_to: feat__orchestration[version==1]

Overview
--------

The Kyron runtime is a Rust ``async/await`` runtime. It is built from one or more
execution engines. Each engine has its own pool of async workers, which use a
work-stealing scheduler, and optional dedicated workers that keep a task on its thread.
The runtime also provides a safety worker, an I/O event loop, TCP/UDP networking,
timers and sleep, channels, ``select!``, iceoryx2-based IPC, and the
``#[kyron::main]`` and ``#[kyron::test]`` procedural macros.

Static Architecture
-------------------

The static view (``comp_arc_sta``) and the interfaces (``real_arc_int``) are added together
with the component requirements they fulfil.

Requirements Linked to Component Architecture
---------------------------------------------

.. needtable::
   :types: comp_req
   :filter: "comp__kyron_runtime" in satisfied_by
   :columns: id;title;fulfils_back
   :style: table
