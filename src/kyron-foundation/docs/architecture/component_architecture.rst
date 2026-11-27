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

.. _kyron_foundation_architecture:

Kyron Foundation Component Architecture
=======================================

.. document:: Kyron Foundation Component Architecture
   :id: doc__kyron_foundation_architecture
   :status: draft
   :version: 1
   :safety: QM
   :security: NO
   :realizes: wp__component_arch[version==1]

.. comp:: Kyron Foundation
   :id: comp__kyron_foundation
   :security: NO
   :safety: QM
   :status: valid
   :version: 1
   :belongs_to: feat__orchestration[version==1]

Overview
--------

Kyron Foundation provides the low-level building blocks of the runtime: lock-free and
intrusive containers (SPMC queue, intrusive list, reusable object pools, growable and
static vectors, trigger queue), cell types, synchronisation primitives and threading
helpers.

Static Architecture
-------------------

The static view (``comp_arc_sta``) and the interfaces (``real_arc_int``) are added together
with the component requirements they fulfil.

Requirements Linked to Component Architecture
---------------------------------------------

.. needtable::
   :types: comp_req
   :filter: "comp__kyron_foundation" in satisfied_by
   :columns: id;title;fulfils_back
   :style: table
