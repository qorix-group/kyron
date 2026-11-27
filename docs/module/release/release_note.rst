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

.. _kyron_release_note:

Kyron Release Notes
===================

.. document:: Kyron Release Notes
   :id: doc__kyron_release_note
   :status: draft
   :version: 1
   :safety: QM
   :security: YES
   :realizes: wp__module_sw_release_note[version==1]
   :belongs_to: mod__kyron[version==1]

Unreleased
----------

Included
~~~~~~~~

* Kyron runtime (``src/kyron``, ``src/kyron-macros``) and Kyron Foundation
  (``src/kyron-foundation``).

Changes
~~~~~~~

* Kyron is classified QM, designed for a later ASIL B qualification
  (:ref:`kyron_qm_tailoring`).
* The S-CORE process documentation (module, components, checklists) is set up.

Known Issues
~~~~~~~~~~~~

Known issues are tracked in `GitHub issues <https://github.com/eclipse-score/kyron/issues>`_.
Because Kyron makes no ASIL claim, none of them violates a safety requirement.

Build Configuration
~~~~~~~~~~~~~~~~~~~

``bazel build //src/...``, with the platform configurations defined in ``.bazelrc``.
