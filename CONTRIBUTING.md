# Contributing

This document contains information related to contributing to the project and the guidelines you must follow for your contribution to be reviewed and considered.

## Picking a change

It's recommended that you consult the [Issues](https://github.com/pxp-lang/pxp/issues) page before starting work.

Issues are labelled based on the project that they target and the priority of the change. If you fancy a challenge, pick a "high" priority issue. Want something more casual to learn the ropes and discover the project, go for a "low" priority change.

In some cases issues will be dependent on other issues. This will normally be noted on the issue itself with some text similar to "Related to X" or "Depending on X". If this _is_ the case, we recommend leaving the issue alone until the dependency has been resolved.

## Commit messages

* All commit messages must be written in English.
* Commit messages must be prefixed with the name of the crate(s) that is being worked on, i.e. `AST: ...`. When modifying multiple crates in a single commit message, concatenate the names with a `+` character, e.g. `AST+Parser+Type: ...`, ensuring the crate names are ordered alphabetically.
* Keep commit messages as short as possible while still accurately describing the changes made.
* Write commits messages in the imperative mood, e.g. `Foo: Change the way X is parsed`, not `Foo: Changed the way X is parsed`.

## Pull requests

* Keep pull requests scoped to a single change.
* Include a brief, but clear, description of the changes.
* If the pull request closes a particular issue, add `Closes #X` to the very top of the pull request description.
