# ADR 0018: Compose linear units through dimensional algebra

## Status

Accepted — 2026-09-21.

## Context

[AEQ-UNIT-COMPOSITION](../../backlog.md#AEQ-UNIT-COMPOSITION) requires users
to express derived units without adding named markers to the SI catalog.
Quantity multiplication and division already normalize seven SI exponents.
`LinearUnit<D>` carries a static symbol and can describe several semantic
dimensions, so neither a unique operand dimension nor an arbitrary compound
symbol can be inferred from that trait alone.

## Decision

`Unit<D>` owns conversion scale and allocation-free symbol formatting for
named and compound linear units. A blanket implementation derives it from
`LinearUnit<D>` for named units; explicit implementations cover `Product`,
`Quotient` and integer `Power`. Coherence remains closed: all these types and
the sealed traits are owned by Aequitas. `LinearUnit` additionally requires a
private, non-generic named-marker trait, excluding compound markers from the
blanket implementation. Sealing `LinearUnit<D>` alone is insufficient because
the generic `D` permits downstream local dimensions under Rust's orphan rules.
The named-marker bound also requires `UnitDimension`, so adding a named unit
without its composition dimension fails compilation.

`UnitDimension` supplies one operand dimension per named marker. Its exponents
match every supported linear reading of that marker. Existing dimensional
algebra normalizes semantic markers in compound expressions, exactly as it
does in quantity arithmetic. Named semantic quantities retain their explicit
named-unit construction paths; dimensional equivalence alone does not grant
a conversion to a distinct semantic marker.

Degree Celsius and Fahrenheit contribute temperature intervals, never offsets.
Compound expressions cannot construct affine absolute temperatures. Integer
powers use signed-32-bit-bounded Typenum exponents and const exponentiation by
squaring. The type bounds prevent truncated scale exponents. A composed
scale outside the finite positive f64 metadata range fails const evaluation;
the scalar conversion still executes through `UnitScalar` at its own precision.
Inverse conversions use provider-owned native division, not multiplication by
a reciprocal that could overflow before a representable result is calculated.

Equivalent expressions produce the same quantity type, not the same marker
type. Formatting preserves the expression's grouping rather than looking up
a preferred catalog spelling. Units with different scales remain convertible,
not numerically interchangeable without conversion.

## Alternatives

- Enumerating every compound marker makes the catalog a prerequisite for algebra.
- Replacing static named symbols with strings would impose allocation or a
  breaking metadata change on consumers and the generated Python inventory.
- Unsealing units would move scale and semantic consistency outside the provider.
- Selecting a named unit from a dimension loses scale information and is
  ambiguous where several units share the dimension.

## Verification

Value tests compare named and composed units, positive/negative/zero powers,
scaled denominators, nested expressions, unnamed dimensions and formatting.
Compile-fail doctests reject dimension and semantic mismatches. Existing
quantity and binding tests guard named-unit behavior. The no-default-features
check guards no-std support; generated-surface freshness guards the unchanged
Python catalog. No performance improvement is claimed.
