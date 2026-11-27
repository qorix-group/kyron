# Eclipse Safe Open Vehicle Core (SCORE)
The [Eclipse Safe Open Vehicle Core](https://projects.eclipse.org/projects/automotive.score) project aims to develop an open-source core stack for Software Defined Vehicles (SDVs), specifically targeting embedded high-performance Electronic Control Units (ECUs).
Please check the [documentation](https://eclipse-score.github.io) for more information.
The source code is hosted at [GitHub](https://github.com/eclipse-score).

The communication mainly takes place via the [`score-dev` mailing list](https://accounts.eclipse.org/mailing-list/score-dev) and GitHub issues & pull requests (PR). And we have a chatroom for community discussions here [Eclipse SCORE chatroom](https://chat.eclipse.org/#/room/#automotive.score:matrix.eclipse.org).

Please note that for the project the [Eclipse Foundation’s Terms of Use](https://www.eclipse.org/legal/terms-of-use/) apply.
In addition, you need to sign the [ECA](https://www.eclipse.org/legal/ECA.php) and the [DCO](https://www.eclipse.org/legal/dco/) to contribute to the project.

## Contributing
### Getting the source code & building the project
Please refer to the [README.md](README.md) for further information.

### Getting involved

#### Bug Fixes and Improvements
In case you want to fix a bug or contribute an improvement, please perform the following steps:
1) Open a [GitHub issue](https://github.com/eclipse-score/kyron/issues/new) describing the bug or improvement, unless one already exists.
2) Create a PR using the corresponding template ([Bugfix PR template](.github/PULL_REQUEST_TEMPLATE/bug_fix.md) or [Improvement PR template](.github/PULL_REQUEST_TEMPLATE/improvement.md)) and link the issue. Please mark your PR as draft until it's ready for review by the Committers (see the [Eclipse Foundation Project Handbook](https://www.eclipse.org/projects/handbook/#contributing-committers) for more information on the role definitions). Review of the PR and final merge into the project repository is the responsibility of the Committers.

#### S-CORE Process
Kyron is developed according to the [S-CORE process description](https://eclipse-score.github.io/process_description/main/). It is classified **QM** in the current release and is designed for a later ASIL B qualification (see [`docs/module/qm_tailoring.rst`](docs/module/qm_tailoring.rst)).

The process documentation lives next to what it describes:
- Module documents (module view, safety tailoring, user manual, release notes): [`docs/module`](docs/module)
- Component requirements, architecture and inspection checklists: [`src/kyron/docs`](src/kyron/docs) and [`src/kyron-foundation/docs`](src/kyron-foundation/docs)

When a change affects behaviour, update the related component requirements and architecture in the same PR. Link the implementation with `// req-Id: <requirement id>` comments, and link component integration tests with `@add_test_properties(...)`. Check the documentation locally with `bazel run //:docs_check`.

#### Additional Information
Please note, that all Git commit messages must adhere the rules described in the [Eclipse Foundation Project Handbook](https://www.eclipse.org/projects/handbook/#resources-commit).
