# Milestone ladder ([ADR 0020](../docs/adr/0020-release-scope-1-0.md))
<a id="M-published-eunomia"></a> **M-published-eunomia** (0.4.0) — complete. [Release checks and consumer verification](../checklist.md).
- exit: resolve Eunomia and Eunomia Derive 0.12.0 from crates.io with one package identity each. Record the published artifact revision and verify a fresh registry consumer.
- exit: `cargo fmt --check`, strict clippy, native/binding tests, doctests, rustdoc, no-std and default-off edges pass against the published provider.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-published-eunomia — stable quantity-law release (authority-gated): gates as above plus SemVer and publish dry-run, current book/rustdoc, no open correctness item.
<a id="M-ecosystem"></a> **M-ecosystem** needs: M-1.0 — owner-gated leftovers (do not gate 1.0): PyPI publisher registration + wheel publication (AEQ-PY-BINDING-001), proof-tool decision (AEQ-LEAN-SPIKE-001), third-party recurseml verdict; deferred trio stays documented non-goals.
