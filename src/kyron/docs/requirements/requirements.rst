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

.. _kyron_runtime_requirements:

Kyron Runtime Component Requirements
====================================

.. document:: Kyron Runtime Component Requirements
   :id: doc__kyron_runtime_requirements
   :status: draft
   :version: 1
   :safety: QM
   :security: YES
   :realizes: wp__requirements_comp[version==1]

The component requirements (``comp_req``) of this component are derived from the orchestration
executor feature requirements of the S-CORE platform. They are added area by area. Each one
carries ``safety: QM`` and is satisfied by :need:`comp__kyron_runtime`.

Component Requirements
----------------------

Executor Core
~~~~~~~~~~~~~

.. comp_req:: Cooperative Async Task Execution
   :id: comp_req__kyron_runtime__async_task_exec
   :reqtype: Functional
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_async_rt[version==1]

   The runtime shall execute tasks provided as Rust futures (``async``/``await``) on its async
   worker threads, polling each task cooperatively until it completes.

.. comp_req:: Fixed Async Worker Pool
   :id: comp_req__kyron_runtime__fixed_worker_pool
   :reqtype: Functional
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_fixed_pool[version==1]

   An execution engine shall run its async tasks on a fixed number of async worker threads. The
   number is configured when the engine is built, in the range 1 to 128, and does not change
   while the engine runs. A configuration outside this range shall be rejected when the engine
   is built.

.. comp_req:: Multiple Isolated Execution Engines
   :id: comp_req__kyron_runtime__multi_engine
   :reqtype: Functional
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_pool_isolation[version==1], feat_req__orchestration__exec_scale_instances[version==1]

   The runtime shall support multiple execution engines, each with its own async worker threads.
   A task shall run only on the worker threads of the execution engine it was spawned into.

Blocking Work
~~~~~~~~~~~~~

.. comp_req:: Dedicated Workers
   :id: comp_req__kyron_runtime__dedicated_worker
   :reqtype: Functional
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_block_threads[version==1]

   An execution engine shall support dedicated workers. Each dedicated worker runs on its own OS
   thread outside the async worker pool and is identified by a unique worker ID. A blocking call
   on a dedicated worker shall not block the async workers, and a blocking call on an async
   worker shall not block the dedicated workers. Registering the same worker ID twice shall be
   rejected when the engine is built.

.. comp_req:: Dedicated Worker Task Locality
   :id: comp_req__kyron_runtime__dedicated_locality
   :reqtype: Functional
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_block_threads[version==1]

   A task spawned on a dedicated worker shall run only on the thread of that dedicated worker,
   including after it yields.

.. comp_req:: Spawn on Unregistered Dedicated Worker
   :id: comp_req__kyron_runtime__dedicated_unknown_id
   :reqtype: Interface
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :satisfied_by: comp__kyron_runtime[version==1]
   :derived_from: feat_req__orchestration__exec_block_threads[version==1]

   Spawning a task on a dedicated worker ID that is not registered in the execution engine shall
   be reported as a non-recoverable error.

Overview
~~~~~~~~

.. needtable::
   :types: comp_req
   :filter: "comp__kyron_runtime" in satisfied_by
   :columns: id;title;safety;security;status;derived_from
   :style: table

Parent Feature Requirements
---------------------------

.. needtable::
   :types: feat_req
   :filter: id.startswith("feat_req__orchestration__exec_") and safety == "QM" and status == "valid"
   :columns: id;title;safety;status
   :style: table
