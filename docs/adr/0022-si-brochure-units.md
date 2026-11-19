# ADR 0022: Complete the SI Brochure unit catalog

## Status

Proposed

## Context

The [BIPM SI Brochure, 9th edition, version 4.01](https://doi.org/10.59161/AUEZ1291)
defines seven base units and 22 specially named derived units. Tables 5 and 6
give coherent examples, and Table 8 lists the non-SI units in scope. Section 3
defines decimal prefixes and lists binary prefixes for information units.
Section 5.4.7 also specifies the unit one, percent and parts per million.

## Decision

Extend the sealed catalog and the generated Python inventory using the
existing quantity, scalar and unit contracts. Use `Prefixed<P, U>` for one
eligible prefix applied to one unit token. Prefix scale and symbol are const
metadata. A prefixed Celsius temperature scales the reading before adding
273.15 K. The same prefix scales a Celsius interval with no offset. Prefixes
for mass attach to gram. Integer powers, products and quotients use ADR 0018.

Retain semantic distinctions where a shared exponent vector would accept an
incorrect conversion. In particular Hz and rad/s require an explicit factor
of 2π, cd and lm describe different quantities, and var denotes reactive
power. Compound expressions keep the dimensional normalization in ADR 0018.

Store logarithmic amplitude and power levels in their respective natural-log
coordinates. Require the caller to supply a same-dimension reference for a
conversion from measurements. The amplitude-to-power mapping is named for
its square-law assumption. Logarithmic coordinates have their own unit
contract and cannot appear in linear compound expressions.

The polynomial unit algebra represents the unlimited set of coherent derived
expressions. The catalog covers current definitions in the cited chapters.
Superseded definitions in the historical appendices and external
realization procedures are historical or metrological reference material.

## Alternatives

A flat list of prefixed markers repeats scale factors and can silently miss
valid combinations. Generic application uses one factor and one spelling per
prefix, and the compiler rejects compound prefixes and ineligible bases.

A single dimensionless scale for dB, B and Np silently assumes a relationship
between amplitudes and power. Signal-kind-specific units make that relationship
an explicit API choice.

## Verification

Test the defining identities and each Table 8 factor, and compare coherent
units to dimensional expressions. Exercise affine temperatures, exponent
distribution, prefix eligibility, logarithmic reference validation and
extreme ratios. Run the existing workspace gates and generated binding tests.
