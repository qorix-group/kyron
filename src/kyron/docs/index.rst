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

.. _component_kyron_runtime:

Kyron Runtime
#############

.. document:: Kyron Runtime
   :id: doc__kyron_runtime
   :status: draft
   :version: 1
   :safety: QM
   :security: YES
   :realizes: wp__cmpt_request[version==1]

Abstract
========

The Kyron runtime is a Rust ``async/await`` runtime. It is built from one or more
execution engines. Each engine has its own pool of async workers, which use a
work-stealing scheduler, and optional dedicated workers that keep a task on its thread.
The runtime also provides a safety worker, an I/O event loop, TCP/UDP networking,
timers and sleep, channels, ``select!``, iceoryx2-based IPC, and the
``#[kyron::main]`` and ``#[kyron::test]`` procedural macros.

The component consists of ``src/kyron`` and ``src/kyron-macros``.

Motivation
==========

The S-CORE orchestration feature needs an executor with deterministic, configurable
scheduling: fixed worker pools, OS priorities, thread affinity, pool isolation and
dedicated threads for blocking work (``feat_req__orchestration__exec_*``). General-purpose
runtimes do not offer this level of control, and they are not designed for a later
safety qualification.

Safety Classification
=====================

The component is designed for safety-critical use, with ASIL B as the long-term target.
In the current release it is classified **QM**, with no ASIL claim. See
:ref:`kyron_qm_tailoring`.

Component Detail Information
============================

.. toctree::
   :maxdepth: 2

   requirements/index
   architecture/index
   detailed_design/index
