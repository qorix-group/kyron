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

.. _component_kyron_foundation:

Kyron Foundation
################

.. document:: Kyron Foundation
   :id: doc__kyron_foundation
   :status: draft
   :version: 1
   :safety: QM
   :security: NO
   :realizes: wp__cmpt_request[version==1]

Abstract
========

Kyron Foundation provides the low-level building blocks of the runtime: lock-free and
intrusive containers (SPMC queue, intrusive list, reusable object pools, growable and
static vectors, trigger queue), cell types, synchronisation primitives and threading
helpers.

The component consists of ``src/kyron-foundation``.

Motivation
==========

The runtime needs allocation-aware, bounded data structures with predictable behaviour.
Keeping them in a separate component makes them reusable, and lets them be tested
(including with Loom and Miri) independently of the scheduler.

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
