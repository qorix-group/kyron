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

.. _kyron_foundation_requirements:

Kyron Foundation Component Requirements
=======================================

.. document:: Kyron Foundation Component Requirements
   :id: doc__kyron_foundation_requirements
   :status: draft
   :version: 1
   :safety: QM
   :security: NO
   :realizes: wp__requirements_comp[version==1]

The component requirements (``comp_req``) of this component are derived from the orchestration
executor feature requirements of the S-CORE platform. They are added area by area. Each one
carries ``safety: QM`` and is satisfied by :need:`comp__kyron_foundation`.

Component Requirements
----------------------

.. needtable::
   :types: comp_req
   :filter: "comp__kyron_foundation" in satisfied_by
   :columns: id;title;safety;security;status;derived_from
   :style: table

Parent Feature Requirements
---------------------------

.. needtable::
   :types: feat_req
   :filter: id.startswith("feat_req__orchestration__exec_") and safety == "QM" and status == "valid"
   :columns: id;title;safety;status
   :style: table
