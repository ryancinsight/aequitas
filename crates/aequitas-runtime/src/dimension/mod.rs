mod exponent;
mod reflection;

pub use exponent::Exponent;
pub use reflection::StaticDimension;

/// Exact SI powers in length, mass, time, current, temperature, amount and
/// luminous intensity order. Algebra acts independently on each axis.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RuntimeDimension([Exponent; 7]);

impl RuntimeDimension {
    /// Construct from seven SI exponents in canonical axis order.
    #[must_use]
    pub const fn new(axes: [Exponent; 7]) -> Self {
        Self(axes)
    }

    /// Reflect the seven exponents of a static Aequitas dimension.
    #[must_use]
    pub fn of<D: StaticDimension>() -> Self {
        D::runtime_dimension()
    }

    /// Borrow the canonical exponents.
    #[must_use]
    pub const fn axes(&self) -> &[Exponent; 7] {
        &self.0
    }

    /// Multiplying quantities adds their exponents.
    #[must_use]
    pub fn product(&self, rhs: &Self) -> Self {
        Self(core::array::from_fn(|i| {
            Exponent(&self.0[i].0 + &rhs.0[i].0)
        }))
    }

    /// Dividing quantities subtracts their exponents.
    #[must_use]
    pub fn quotient(&self, rhs: &Self) -> Self {
        Self(core::array::from_fn(|i| {
            Exponent(&self.0[i].0 - &rhs.0[i].0)
        }))
    }

    /// Raising a quantity to a rational power multiplies each exponent by it.
    #[must_use]
    pub fn power(&self, power: &Exponent) -> Self {
        Self(core::array::from_fn(|i| Exponent(&self.0[i].0 * &power.0)))
    }
}
