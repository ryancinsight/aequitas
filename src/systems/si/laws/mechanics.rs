//! Classical-mechanics laws over typed quantities.
//!
//! Newton II, weight, Hooke, elastic energy and Bernoulli. Scalar arithmetic
//! stays in the quantity values; no raw-scalar escape.

use eunomia::FloatElement;

use super::super::constants::STANDARD_GRAVITY;
use super::super::quantities::{
    Acceleration, Energy, Force, Length, Mass, MassDensity, Pressure, SpringStiffness, Velocity,
};

impl<T: FloatElement> Mass<T> {
    /// Newton II `F = m · a`.
    #[inline]
    #[must_use]
    pub fn newton2(self, acceleration: Acceleration<T>) -> Force<T> {
        self * acceleration
    }

    /// Weight `W = m · g` with the conventional standard gravity.
    #[inline]
    #[must_use]
    pub fn weight(self) -> Force<T> {
        self * STANDARD_GRAVITY.cast::<T>()
    }

    /// Kinetic energy `E = m · v² / 2`.
    #[inline]
    #[must_use]
    pub fn kinetic_energy(self, velocity: Velocity<T>) -> Energy<T> {
        let v = velocity.into_base();
        Energy::from_base(self.into_base() * v * v / T::from_f64(2.0))
    }
}

impl<T: FloatElement> SpringStiffness<T> {
    /// Hooke force `F = −k · x` (magnitude; direction is the caller's mesh
    /// concern, so no negation bound is needed — pass the displacement from
    /// equilibrium toward the load).
    #[inline]
    #[must_use]
    pub fn hooke_force(self, displacement: Length<T>) -> Force<T> {
        self * displacement
    }

    /// Elastic strain energy `U = k · x² / 2`.
    #[inline]
    #[must_use]
    pub fn elastic_energy(self, displacement: Length<T>) -> Energy<T> {
        let x = displacement.into_base();
        Energy::from_base(self.into_base() * x * x / T::from_f64(2.0))
    }
}

impl<T: FloatElement> Pressure<T> {
    /// Hydrostatic pressure `p = rho · g · h`.
    #[inline]
    #[must_use]
    pub fn hydrostatic(density: MassDensity<T>, height: Length<T>) -> Self {
        density * STANDARD_GRAVITY.cast::<T>() * height
    }

    /// Bernoulli stagnation pressure `p₀ = p + rho · v² / 2`.
    #[inline]
    #[must_use]
    pub fn stagnation(self, density: MassDensity<T>, velocity: Velocity<T>) -> Self {
        Self::from_base(self.into_base() + Self::dynamic_pressure(density, velocity).into_base())
    }
}
