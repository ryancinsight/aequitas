//! Correctly rounded powers of ten for unit scales.
//!
//! A decimal unit such as the decimetre has a scale that is the binary64
//! rounding of `10^n`. Raising that rounded scale to a power by repeated
//! multiplication rounds again at every step: `0.1 * 0.1` is one binary64 step
//! above the nearest double to `0.01`. A scale that is itself `10^n` is
//! instead raised by exponent addition, `(10^n)^p = 10^(n·p)`, and rounded
//! once.

/// Largest `n` for which `10^n` is exactly representable in binary64.
///
/// `10^n = 2^n · 5^n` and `5^22 < 2^53`, so the odd factor fits the 53-bit
/// significand through `n = 22`; `5^23` does not.
const MAX_EXACT_DECADE: i32 = 22;

/// The binary64 nearest to `10^exponent`, or `None` past [`MAX_EXACT_DECADE`].
///
/// For `n >= 0` each partial product `10^k` is exact, so the result is exact.
/// For `n < 0` the result is one division `1 / 10^|n|` of two exact operands,
/// which IEEE 754 rounds to nearest, the correctly rounded `10^n`. Beyond the
/// exact range a product or quotient would round twice, so no value is offered.
pub(super) const fn power_of_ten(exponent: i32) -> Option<f64> {
    if exponent.unsigned_abs() > MAX_EXACT_DECADE.unsigned_abs() {
        return None;
    }
    let mut exact = 1.0;
    let mut remaining = exponent.unsigned_abs();
    while remaining != 0 {
        exact *= 10.0;
        remaining -= 1;
    }
    Some(if exponent < 0 { 1.0 / exact } else { exact })
}

/// The exponent `n` when `scale` is the binary64 nearest to `10^n`, for
/// `n != 0` within the exact range, and `None` otherwise.
pub(super) const fn decade_of(scale: f64) -> Option<i32> {
    let mut exponent = -MAX_EXACT_DECADE;
    while exponent <= MAX_EXACT_DECADE {
        if exponent != 0
            && let Some(decade) = power_of_ten(exponent)
            && decade.to_bits() == scale.to_bits()
        {
            return Some(exponent);
        }
        exponent += 1;
    }
    None
}

/// `scale^exponent` as one correctly rounded power of ten, when `scale` is a
/// decade and the product of exponents stays in the exact range.
pub(super) const fn decade_power(scale: f64, exponent: i32) -> Option<f64> {
    match decade_of(scale) {
        Some(decade) => match decade.checked_mul(exponent) {
            Some(product) => power_of_ten(product),
            None => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use super::{MAX_EXACT_DECADE, decade_of, decade_power, power_of_ten};

    /// Rust's float parser rounds to nearest, independent of the multiply and
    /// divide used by [`power_of_ten`].
    fn nearest(exponent: i32) -> f64 {
        format!("1e{exponent}")
            .parse()
            .expect("invariant: `1e<integer>` is a valid float literal")
    }

    #[test]
    fn power_of_ten_is_the_nearest_double_across_the_exact_range() {
        for exponent in -MAX_EXACT_DECADE..=MAX_EXACT_DECADE {
            let computed = power_of_ten(exponent).expect("in range");
            assert_eq!(
                computed.to_bits(),
                nearest(exponent).to_bits(),
                "10^{exponent}"
            );
        }
    }

    #[test]
    fn power_of_ten_declines_the_range_it_cannot_round_once() {
        assert!(power_of_ten(MAX_EXACT_DECADE + 1).is_none());
        assert!(power_of_ten(-MAX_EXACT_DECADE - 1).is_none());
        assert!(power_of_ten(i32::MIN).is_none());
    }

    #[test]
    fn a_decade_power_is_one_rounding_of_the_summed_exponent() {
        for decade in (-MAX_EXACT_DECADE..=MAX_EXACT_DECADE).filter(|n| *n != 0) {
            for power in -8..=8 {
                let expected = decade * power;
                let observed = decade_power(nearest(decade), power);
                if expected.abs() <= MAX_EXACT_DECADE {
                    assert_eq!(
                        observed.expect("in range").to_bits(),
                        nearest(expected).to_bits(),
                        "(1e{decade})^{power}"
                    );
                } else {
                    assert!(observed.is_none(), "(1e{decade})^{power}");
                }
            }
        }
    }

    #[test]
    fn repeated_multiplication_is_the_defect_the_decade_path_removes() {
        assert_ne!((0.1_f64 * 0.1).to_bits(), nearest(-2).to_bits());
        assert_eq!(decade_power(0.1, 2), Some(nearest(-2)));
    }

    #[test]
    fn non_decade_scales_are_not_claimed() {
        for scale in [
            1.0,
            0.3048,
            1852.0,
            core::f64::consts::PI / 180.0,
            1.0e-3 * 1.0e-3 * 1.5,
        ] {
            assert_eq!(decade_of(scale), None, "{scale}");
        }
    }
}
