# Runtime fractional SI dimensions

Accepted for the physical fractional derivative on 2026-11-22.

A value of static dimension D differentiated in time to order α has
exponent vector D − α [0,0,1,0,0,0,0]. Store seven reduced rational
numbers once per homogeneous buffer. Exact addition, subtraction and
multiplication preserve equality through composition. A binary order
denotes its exact represented value. An explicitly specified rational
can represent one third exactly. There is no reference-time parameter.

The independently versioned Aequitas runtime crate uses the same registry
quantity identity as Kwavers. It owns the allocating exponent arithmetic.
Static quantities retain their existing layout. Reflect SI axes on entry.
Semantic labels such as stress and pressure share those axes. Reflection
is one-way because dimensional equality cannot recover that distinction.

Reject a zero denominator and nonfinite scalar exponents. Verify the time
exponent of a pressure half derivative, native floating-point endpoints,
and quotient/product cancellation including orders below one ulp of the
original exponent.
