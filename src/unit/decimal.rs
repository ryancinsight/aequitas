//! Correctly rounded powers of ten for unit scales.
//!
//! A decimal unit such as the decimetre has a scale that is the binary64
//! rounding of `10^n`. Raising that rounded scale to a power by repeated
//! multiplication rounds again at every step: `0.1 * 0.1` is one binary64 step
//! above the nearest double to `0.01`. A scale that is itself `10^n` is
//! instead raised by exponent addition, `(10^n)^p = 10^(n·p)`, and rounded
//! once. A prefix applied to a decade root composes the same way, as
//! `10^a · 10^b = 10^(a+b)`. Operands with `0 < |n| <= 30` are recognised,
//! which spans every decimal prefix and decade root; the composed exponent
//! must stay within `|a+b| <= 22`, where one rounding is provable.

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

/// Largest `|n|` for which an operand `10^n` is recognised.
///
/// The decimal prefixes reach `10^±30` and the decade roots reach `10^-28`
/// (barn), so this bound recognises every operand a prefix composition meets.
const MAX_OPERAND_DECADE: i32 = 30;

/// The binary64 nearest to `10^n`, at index `n + MAX_OPERAND_DECADE`.
///
/// The compiler rounds float literals to nearest, so the entries with
/// `|n| > MAX_EXACT_DECADE` need no exact-arithmetic argument. They recognise
/// operands by bit equality and are never an arithmetic result.
const OPERAND_DECADES: [f64; 61] = [
    1e-30, 1e-29, 1e-28, 1e-27, 1e-26, 1e-25, 1e-24, 1e-23, 1e-22, 1e-21, 1e-20, 1e-19, 1e-18,
    1e-17, 1e-16, 1e-15, 1e-14, 1e-13, 1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4,
    1e-3, 1e-2, 1e-1, 1.0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13,
    1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29,
    1e30,
];

/// The exponent `n` when `scale` is the binary64 nearest to `10^n`, for
/// `0 < |n| <= 30`, and `None` otherwise.
pub(super) const fn decade_of(scale: f64) -> Option<i32> {
    let mut exponent = -MAX_OPERAND_DECADE;
    let mut index = 0;
    while index < OPERAND_DECADES.len() {
        if exponent != 0 && OPERAND_DECADES[index].to_bits() == scale.to_bits() {
            return Some(exponent);
        }
        exponent += 1;
        index += 1;
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

/// `left · right`, the nearest binary64 to `10^(a+b)` when `left` is `10^a`
/// and `right` is `10^b` with `0 < |a|, |b| <= 30` and `|a+b| <= 22`, and the
/// floating-point product otherwise.
pub(super) const fn product_scale(left: f64, right: f64) -> f64 {
    if let Some(left_decade) = decade_of(left)
        && let Some(right_decade) = decade_of(right)
        && let Some(sum) = left_decade.checked_add(right_decade)
        && let Some(scale) = power_of_ten(sum)
    {
        return scale;
    }
    left * right
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use super::{
        MAX_EXACT_DECADE, MAX_OPERAND_DECADE, decade_of, decade_power, power_of_ten, product_scale,
    };

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

    #[test]
    fn every_operand_decade_is_recognised_by_its_exponent() {
        for exponent in -MAX_OPERAND_DECADE..=MAX_OPERAND_DECADE {
            let expected = (exponent != 0).then_some(exponent);
            assert_eq!(decade_of(nearest(exponent)), expected, "1e{exponent}");
        }
        assert_eq!(decade_of(nearest(MAX_OPERAND_DECADE + 1)), None);
        assert_eq!(decade_of(nearest(-MAX_OPERAND_DECADE - 1)), None);
    }

    #[test]
    fn a_decade_product_is_one_rounding_of_the_summed_exponent() {
        for left in (-MAX_OPERAND_DECADE..=MAX_OPERAND_DECADE).filter(|n| *n != 0) {
            for right in (-MAX_OPERAND_DECADE..=MAX_OPERAND_DECADE).filter(|n| *n != 0) {
                if (left + right).abs() <= MAX_EXACT_DECADE {
                    assert_eq!(
                        product_scale(nearest(left), nearest(right)).to_bits(),
                        nearest(left + right).to_bits(),
                        "1e{left} * 1e{right}"
                    );
                }
            }
        }
    }
}
