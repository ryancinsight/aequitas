# Book validation and publication

Scope: workflow-only correction `[patch]`; Rust and Python package contracts,
versions, manifests and lockfiles do not change. Atlas owns the build and Pages
pipeline under ADR 0035. Aequitas owns the event that authorizes publication.

## Contract

Relevant pull requests and main pushes still compile the book, run its Rust
samples against `aequitas`, check the output and upload a Pages artifact. A
manual run performs the same validation. The shared build is not gated by the
publication input. The caller's static contract must pass before either path.

Let `B` mean successful book validation, `M` mean `workflow_dispatch`, `R` mean
`refs/heads/main`, and `U` mean the typed `publish` input, default false. Pages
deployment requires `B ∧ M ∧ R ∧ U`. The shared workflow additionally excludes
pull requests. Hence PRs, ordinary dependency pushes, manual validation and
dispatches on other branches/tags cannot deploy. `needs: build` retains the
implicit success gate; failure or cancellation prevents deployment.

| Event | Validation and artifact | Deployment after success |
| --- | --- | --- |
| Relevant pull request | Yes | No |
| Relevant push to main | Yes | No |
| Manual run on main, publish omitted/false | Yes | No |
| Manual run on another branch/tag, publish true | Yes | No |
| Manual run on main, publish true | Yes | Subject to existing environment protection |

Authorization is an operator decision within the applicable release scope;
the checkbox does not remove the `github-pages` environment gate. Permissions,
OIDC action, output directory and per-ref deployment concurrency stay intact.

## Provider and alternatives

The caller pins Atlas `89f24d96c340ac0989145d058beececaac43a3ad`, which adds
the optional `deploy` boolean while preserving other callers with a true
default. Review Atlas before integrating this caller. No other consumer moves.

The pin also includes existing Atlas changes since Aequitas's old `20c9398`:
the repository-scoped Rust cache, a 25-minute build-job bound, a separate
12-minute package-build bound, and `compgen` artifact-presence detection.
Those are inherited provider changes, not new timeout or artifact-policy
changes in this correction. Code-sample staging and Pages action digests remain
intact. The new gate skips Pages configuration when validation alone is wanted.

Removing Cargo/source trigger paths loses sample validation. Gating the entire
caller skips validation. Forking the build into a separate publication workflow
duplicates the pipeline and can publish an artifact different from the tested
one. The caller expression preserves one pipeline and one tested artifact.

## Verification boundary

`scripts/tests/test_book_pages_workflow.py` statically pins the real workflow's
events, paths, typed input, full publication predicate, immutable provider pin,
sample-test inputs, permissions, concurrency and contract prerequisite.
Actionlint checks GitHub workflow syntax. Actual Pages deployment is an
unexecuted release-authorized acceptance step. This change creates no package
release and grants no authorization to merge or publish.

Local verification also evaluates the actual two workflow predicates through
GitHub's `@actions/expressions` 0.3.61. The Cartesian product of five event
names, three refs (main, feature branch, release tag), and three input states
(omitted, false, true) yields 45 Aequitas scenarios. All reject publication
except a true-input main dispatch. The same 45 cases against the previous
shared predicate prove omitted/true inputs preserve existing behavior and
false inputs prevent deployment. The persistent static contracts bind that
reviewed formula to the checked-in workflow; the matrix is local evidence.
