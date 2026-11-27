<!-- markdownlint-disable MD013 Line breaks on the bullet list lines are also present on the github renderer, therefore no line length limitation -->
<!-- markdownlint-disable MD041 On the github PR template we want to start with '## Headline' -->

## Notes for Reviewer
<!-- Items in addition to the checklist below that the reviewer should look for -->

## Pre-Review Checklist for the PR Author

* [ ] PR title is short, expressive and meaningful
* [ ] Commits are properly organized
* [ ] Relevant issues are linked in the [References](#references) section
* [ ] Tests are conducted
* [ ] Unit tests are added

## S-CORE Process Checklist

<!-- Kyron is classified QM; this checklist is the inspection record for QM artifacts. See docs/module/qm_tailoring.rst -->

* [ ] Affected component requirements (`comp_req`) and architecture are updated, or the change has no requirement impact
* [ ] New or changed implementation is linked with `// req-Id: ...` where it realizes a requirement
* [ ] Component integration tests are linked to requirements with `@add_test_properties(...)`
* [ ] Every new or changed `unsafe` block has a `SAFETY:` justification
* [ ] `bazel run //:docs_check` passes

## Checklist for the PR Reviewer

* [ ] Commits are properly organized and messages are according to the guideline
* [ ] Unit tests have been written for new behavior
* [ ] Public API is documented
* [ ] PR title describes the changes
* [ ] Requirements, architecture and code changes are consistent (requirements and implementation inspection checklists in the component docs)

## Post-review Checklist for the PR Author

* [ ] All open points are addressed and tracked via issues

## References

<!-- Use either 'Closes #123' or 'Relates to #123' to reference the corresponding issue. -->

Closes # <!-- Add issue number after '#' -->

<!-- markdownlint-enable MD041 -->
<!-- markdownlint-enable MD013 -->
