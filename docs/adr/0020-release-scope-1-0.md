# ADR 0020: Aequitas release scope for 1.0

- Status: Accepted
- Date: 2026-11-10
- Class: `[arch]` `[minor]`

## Context

Aequitas (`aequitas` 0.3.2) provides the Atlas physical-quantity and
dimensional law over the Eunomia scalar surface. The product surface is
delivered: quantity laws through unit composition (ADRs 0001–0015, 0018),
the Python binding pipeline (ADR 0016), and the Eunomia 0.10 cohort contract
(ADR 0019). The manifest still bounds Eunomia to `<0.13.0`, so the stable
Eunomia 1.0 surface requires a consumer migration here. The only open items
are three deferred documented non-goals and three items blocked on
owner/account actions. No milestone ladder exists to 1.0.

## Decision

Adopt a two-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-eunomia-1-0** (0.4.0): users build quantities over the stable Eunomia
  1.0 scalar surface — bound, lock graph, and no-std/default-off edges moved
  off the pre-1.0 provider with gates green.
- **M-1.0** (1.0.0): stable quantity-law surface (authority-gated) with
  documented contracts and release gates satisfied.

Owner-gated leftovers (PyPI trusted-publisher registration and wheel
publication per AEQ-PY-BINDING-001, the proof-tool decision per
AEQ-LEAN-SPIKE-001, and the third-party recurseml check) move with the
release but do not gate 1.0; they are tracked under **M-ecosystem**. The
three deferred boundary items stay documented non-goals.

## Alternatives

- Leave progress implicit in backlog items: rejected because stack policy
  requires a runnable capability ladder.
- Gate 1.0 on the wheel publication: rejected because the blocker is a
  pypi.org account action outside the merge-mechanics grant; the pipeline
  itself is landed and verified.
- Target 1.0 in one step: rejected because the Eunomia 1.0 consumer
  migration must land and verify first.

## Consequences

- Open items carry `milestone:` assignments and the migration names the exit
  checks it serves.
- Milestones close only when exit checks pass on fetched default commits; 1.0
  release/publication remains authority-gated.
- This ADR is rewritten in place as scope evidence changes.
