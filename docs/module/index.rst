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

.. _kyron_module:

Kyron Module
============

.. mod:: Kyron Module
   :id: mod__kyron
   :version: 1
   :security: YES
   :safety: QM
   :status: valid
   :maturity: preview
   :includes: comp__kyron_runtime[version==1], comp__kyron_foundation[version==1]

Kyron is classified **QM** in the current release and is designed for a later ASIL B
qualification (see :ref:`kyron_qm_tailoring`). ``security`` is ``YES`` because the runtime
exposes TCP/UDP networking and iceoryx2 IPC.

Module View
-----------

.. mod_view_sta:: Kyron Module Static View
   :id: mod_view_sta__kyron__module
   :version: 1
   :includes: comp__kyron_runtime[version==1], comp__kyron_foundation[version==1]

   .. needarch::
      :scale: 50
      :align: center

      {{ draw_module(need(), needs) }}

Module Documents
----------------

.. toctree::
   :maxdepth: 1

   qm_tailoring
   manuals/user_manual
   release/release_note
