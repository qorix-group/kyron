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
   :implements: real_arc_int_op__kyron_runtime__rt_builder[version==1],real_arc_int_op__kyron_runtime__eng_builder[version==1],real_arc_int_op__kyron_runtime__block_on[version==1],real_arc_int_op__kyron_runtime__spawn[version==1],real_arc_int_op__kyron_runtime__spawn_ded[version==1],real_arc_int_op__kyron_runtime__join_handle[version==1]

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

.. comp_arc_sta:: Kyron Runtime Static View
   :id: comp_arc_sta__kyron_runtime__static_view
   :security: YES
   :safety: QM
   :status: valid
   :version: 1
   :belongs_to: comp__kyron_runtime[version==1]
   :fulfils: comp_req__kyron_runtime__async_task_exec[version==1],comp_req__kyron_runtime__fixed_worker_pool[version==1],comp_req__kyron_runtime__multi_engine[version==1],comp_req__kyron_runtime__dedicated_worker[version==1],comp_req__kyron_runtime__dedicated_locality[version==1],comp_req__kyron_runtime__dedicated_unknown_id[version==1]

   .. needarch::
      :scale: 50
      :align: center

      {{ draw_component(need(), needs) }}

The runtime is structured into the following parts. Areas not yet covered by component
requirements are listed for orientation and are added together with their requirements.

.. list-table::
   :header-rows: 1
   :widths: 25 75

   * - Part
     - Responsibility
   * - Runtime (``src/runtime``)
     - ``RuntimeBuilder`` and ``Runtime``: build a runtime from one or more execution engines,
       enter it with ``block_on`` and spawn tasks into a selected engine.
   * - Execution engine (``src/scheduler/execution_engine.rs``)
     - ``ExecutionEngineBuilder`` and ``ExecutionEngine``: own a fixed pool of async workers,
       the dedicated workers, and the optional safety worker of one engine.
   * - Scheduler (``src/scheduler/scheduler_mt.rs``, ``src/scheduler/task``)
     - Task representation, per-worker queues, work stealing between async workers, and
       routing of dedicated tasks to the queue of their dedicated worker.
   * - Workers (``src/scheduler/workers``)
     - Async workers, dedicated workers and the safety worker. Each one is an OS thread that
       polls the tasks assigned to it.
   * - I/O, networking, time, channels, IPC
     - Covered by later requirement areas.

Interfaces
----------

.. real_arc_int:: Kyron Runtime API
   :id: real_arc_int__kyron_runtime__runtime_api
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :language: rust
   :fulfils: comp_req__kyron_runtime__async_task_exec[version==1],comp_req__kyron_runtime__fixed_worker_pool[version==1],comp_req__kyron_runtime__multi_engine[version==1],comp_req__kyron_runtime__dedicated_worker[version==1],comp_req__kyron_runtime__dedicated_locality[version==1],comp_req__kyron_runtime__dedicated_unknown_id[version==1]

   The public Rust API of the runtime for building engines and runtimes, entering the runtime
   and spawning tasks.

.. real_arc_int_op:: RuntimeBuilder
   :id: real_arc_int_op__kyron_runtime__rt_builder
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``RuntimeBuilder::new``, ``RuntimeBuilder::with_engine`` and ``RuntimeBuilder::build``: build a runtime from one or more execution engines.

.. real_arc_int_op:: ExecutionEngineBuilder
   :id: real_arc_int_op__kyron_runtime__eng_builder
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``ExecutionEngineBuilder``: configure an execution engine (``workers``, ``task_queue_size``, thread parameters, ``with_dedicated_worker``, ``enable_safety_worker``).

.. real_arc_int_op:: Runtime::block_on
   :id: real_arc_int_op__kyron_runtime__block_on
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``Runtime::block_on`` and ``Runtime::block_on_engine``: run a future to completion inside the runtime.

.. real_arc_int_op:: spawn
   :id: real_arc_int_op__kyron_runtime__spawn
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``kyron::spawn``, ``spawn_from_boxed``, ``spawn_from_reusable`` and ``Runtime::spawn_in_engine``: spawn a task on the async workers.

.. real_arc_int_op:: spawn_on_dedicated
   :id: real_arc_int_op__kyron_runtime__spawn_ded
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``kyron::spawn_on_dedicated``, ``spawn_from_boxed_on_dedicated`` and ``spawn_from_reusable_on_dedicated``: spawn a task on a dedicated worker.

.. real_arc_int_op:: JoinHandle
   :id: real_arc_int_op__kyron_runtime__join_handle
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :included_by: real_arc_int__kyron_runtime__runtime_api[version==1]

   ``JoinHandle``: await the completion and the result of a spawned task.

Requirements Linked to Component Architecture
---------------------------------------------

.. needtable::
   :types: comp_req
   :filter: "comp__kyron_runtime" in satisfied_by
   :columns: id;title;fulfils_back
   :style: table
