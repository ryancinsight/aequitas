# Milestone ladder ([ADR 0020](../docs/adr/0020-release-scope-1-0.md))
<a id="M-eunomia-1-0"></a> **M-eunomia-1-0** (0.4.0) needs: (none) — quantities over the stable Eunomia 1.0 surface.
- exit: move the Eunomia bound off `<0.13.0` onto the stable 1.0 surface (canonical URL, no revision alias); refresh the lock graph and record the provider head.
- exit: `cargo fmt --check`, strict clippy, native/binding tests, doctests, rustdoc, no-std and default-off edges pass against the stable provider.
<a id="M-1.0"></a> **M-1.0** (1.0.0) needs: M-eunomia-1-0 — stable quantity-law release (authority-gated): gates as above plus SemVer and publish dry-run, current book/rustdoc, no open correctness item.
<a id="M-ecosystem"></a> **M-ecosystem** needs: M-1.0 — owner-gated leftovers (do not gate 1.0): PyPI publisher registration + wheel publication (AEQ-PY-BINDING-001), proof-tool decision (AEQ-LEAN-SPIKE-001), third-party recurseml verdict; deferred trio stays documented non-goals.
