# ADR 0020: Preserve exact scalar and runtime exponent boundaries

- Status: Accepted
- Date: 2026-10-19
- Item: AEQ-EXPONENT-RANGE [major]

The user accepts this ADR and the coordinated version plan on 2026-10-19:
Eunomia 0.11, Aequitas 0.4, binding 0.2, NumPy/PyO3 0.30, compatibility
bridge first, with tests and reviews mandatory before release.

## Context

`Quantity::powi<P>` accepts every Typenum integer but sends `P::to_i32()` to
the scalar provider. For `P4294967296` this computes the zeroth scalar power
while the length dimension becomes length to the 4294967296th power.
`TaggedDimension::TAG` likewise reads `Integer::I8` without a range proof:
length to the 128th power becomes exponent -128 on the Python wire.
Both violate the existing requirement that runtime values describe their
type-level dimensions exactly. `unit::Power` already proves its signed
32-bit bound with Typenum comparisons.

## Decision

Apply that same comparison pattern to quantity powers. Require
`P: Integer + IsGreaterOrEqual<N2147483648, Output = B1> +
IsLessOrEqual<P2147483647, Output = B1>` before invoking provider `powi`.
Dimension algebra itself remains unbounded. Require each exponent in the
blanket `TaggedDimension` implementation to lie in `[-128, 127]`, matching
the existing wire format. No cast, clipped exponent or alternative scalar
implementation bridges these boundaries.

These additional public Rust bounds require a pre-1.0 minor change: law
crate 0.4.0 and independently versioned binding crate 0.2.0. Generic callers
of `Quantity::powi` must propagate its two comparison bounds; callers
deriving a tag for generic dimensions must propagate the byte bounds.
Concrete valid exponents keep their existing results. Out-of-range
operations become compile errors. Python names, inventory and wire format
remain unchanged. Existing `D: TaggedDimension` consumers need no new bound.
The binding's Rust dependency advances to current PyO3 0.30.1 with the same
ABI floor and method surface; Rust extension consumers must coordinate that
nominal dependency identity. Its wheel does not require consumer Rust types.

Keep the PR draft until the compatibility bridge lands, coordinated provider
and consumer updates are ready, and final exact-head review and gates pass.
Eunomia 0.11 and NumPy/PyO3 0.30 are the approved shared target; the final
range/lock rebase follows the owner bridge. This acceptance does not override
access denials, held publication actions, or the book owner's separate work.

## Runtime and numerical contracts

For every signed-byte root and exponent, return a normalized tag iff the
quotient is an exactly divisible signed byte. Zero divisors, inexact roots
and the overflowing `-128 / -1` quotient return `TagNotDivisible`, which
documents both representability and divisibility; its diagnostic identifies
the exceptional range case. Preserve ordinary diagnostic text.

Square root and complex-by-real scalar division delegate directly to the
existing provider operations at `T` precision. General real power loses
the sign of negative zero; reciprocal multiplication can overflow before
a representable quotient is computed. Neither substitutes for the specified
operation. No wider accumulator or duplicate numerical kernel is needed.

## Alternatives and verification

Retaining truncation permits dimensionally mislabeled values. Saturation
has the same defect. Growing the wire changes the Python protocol, while
implementing a second arbitrary-size scalar power duplicates Eunomia.

Negative compile tests first reproduce successful compilation of oversized
positive powers and tags. Boundary tests retain both signed endpoints and
zero. Native-operation comparisons cover signed zero, subnormal values,
binade endpoints, infinity and NaN across both primitive float precisions.
The root regression checks all 256 by 256 inputs on all seven axes against
exact signed-16-bit reference arithmetic. Required CI, API classification,
package verification and independent review remain exact-head gates.

## Optimized wheel packaging

The actual arm64 macOS release wheel fails to load with a misaligned Mach-O
LINKEDIT string pool under both `strip = "symbols"` and `"debuginfo"`.
The original compiled dylib and installed wheel payload have identical hashes,
so packaging/copying does not introduce the defect. A controlled rebuild with
stripping disabled loads and passes all 2,365 Python tests. Preserve the
symbol tables through the workspace release profile; downstream consumers
retain control of their own profiles and optimized code still has no debug
information. A package-specific override would warn on the standalone law
archive, where the binding package is absent. The symbol-bearing wheel is
larger. Do not weaken smoke tests or
patch generated binaries; verify ordinary release-wheel output on each target.
