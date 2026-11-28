//! Incompressible-flow laws over typed quantities.
//!
//! Each law takes quantities in and returns a quantity out; no raw-scalar
//! escape. Scalar transcendence runs through [`eunomia::FloatElement`], never
//! a native `f64`/`f32` method, so wrapped scalars and locked
//! feature-combination builds keep working.

use eunomia::FloatElement;

use super::super::quantities::{
    Area, Dimensionless, DynamicViscosity, Force, Length, MassDensity, Pressure, PressureGradient,
    Velocity, VolumetricFlowRate,
};

impl<T: FloatElement> MassDensity<T> {
    /// Reynolds number `Re = rho * v * L / mu`.
    ///
    /// Ratio of inertial to viscous forces; the laminar–turbulent predictor
    /// for pipes (`Re ≲ 2300` laminar), channels and boundary layers.
    #[inline]
    #[must_use]
    pub fn reynolds_number(
        self,
        velocity: Velocity<T>,
        length: Length<T>,
        viscosity: DynamicViscosity<T>,
    ) -> Dimensionless<T> {
        Dimensionless::from_base(
            self.into_base() * velocity.into_base() * length.into_base() / viscosity.into_base(),
        )
    }
}

impl<T: FloatElement> DynamicViscosity<T> {
    /// Hagen–Poiseuille pressure drop for steady laminar pipe flow:
    /// `dP = 128 * mu * L * Q / (pi * D^4)`.
    #[inline]
    #[must_use]
    pub fn hagen_poiseuille_drop(
        self,
        length: Length<T>,
        flow_rate: VolumetricFlowRate<T>,
        diameter: Length<T>,
    ) -> Pressure<T> {
        let pi = T::from_f64(core::f64::consts::PI);
        let d4 = diameter.into_base().powi(4);
        Pressure::from_base(
            T::from_f64(128.0) * self.into_base() * length.into_base() * flow_rate.into_base()
                / (pi * d4),
        )
    }

    /// Stokes drag on a sphere at low Reynolds number: `F = 6 * pi * mu * R * v`.
    #[inline]
    #[must_use]
    pub fn stokes_drag(self, radius: Length<T>, velocity: Velocity<T>) -> Force<T> {
        let six_pi = T::from_f64(6.0 * core::f64::consts::PI);
        Force::from_base(six_pi * self.into_base() * radius.into_base() * velocity.into_base())
    }

    /// Viscous wall stress for a linear near-wall profile: `tau = mu * v / R`.
    ///
    /// The `4 * mu * dR/dt / R` bubble-wall form and the pipe-flow
    /// `8 * mu * v_mean / D` form are both this law with their geometric
    /// prefactor applied by the caller.
    #[inline]
    #[must_use]
    pub fn viscous_wall_stress(self, velocity: Velocity<T>, radius: Length<T>) -> Pressure<T> {
        Pressure::from_base(self.into_base() * velocity.into_base() / radius.into_base())
    }

    /// Hagen–Poiseuille pressure gradient in a circular pipe:
    /// `dP/dx = 32 * mu * v_mean / D^2`.
    #[inline]
    #[must_use]
    pub fn hagen_poiseuille_gradient(
        self,
        mean_velocity: Velocity<T>,
        diameter: Length<T>,
    ) -> PressureGradient<T> {
        let d2 = diameter.into_base().powi(2);
        PressureGradient::from_base(
            T::from_f64(32.0) * self.into_base() * mean_velocity.into_base() / d2,
        )
    }
}

impl<T: FloatElement> Pressure<T> {
    /// Darcy–Weisbach pressure drop: `dP = f * (L/D) * rho * v^2 / 2`.
    #[inline]
    #[must_use]
    pub fn darcy_weisbach_drop(
        friction_factor: Dimensionless<T>,
        length: Length<T>,
        diameter: Length<T>,
        density: MassDensity<T>,
        velocity: Velocity<T>,
    ) -> Self {
        let v = velocity.into_base();
        Self::from_base(
            friction_factor.into_base() * length.into_base() / diameter.into_base()
                * density.into_base()
                * v
                * v
                / T::from_f64(2.0),
        )
    }

    /// Bernoulli dynamic pressure: `q = rho * v^2 / 2`.
    #[inline]
    #[must_use]
    pub fn dynamic_pressure(density: MassDensity<T>, velocity: Velocity<T>) -> Self {
        let v = velocity.into_base();
        Self::from_base(density.into_base() * v * v / T::from_f64(2.0))
    }
}

impl<T: FloatElement> super::super::quantities::SurfaceTension<T> {
    /// Young–Laplace pressure jump across a spherical interface: `dP = 2 * sigma / R`.
    #[inline]
    #[must_use]
    pub fn young_laplace_pressure(self, radius: Length<T>) -> Pressure<T> {
        Pressure::from_base(T::from_f64(2.0) * self.into_base() / radius.into_base())
    }
}

impl<T: FloatElement> MassDensity<T> {
    /// Womersley number `Wo = R * sqrt(omega * rho / mu)`.
    ///
    /// Ratio of pulsatile inertial to viscous forces; the CFDrs/Kwavers
    /// cardiovascular inflow parameter.
    #[inline]
    #[must_use]
    pub fn womersley_number(
        self,
        radius: Length<T>,
        angular_frequency: super::super::quantities::AngularVelocity<T>,
        viscosity: DynamicViscosity<T>,
    ) -> Dimensionless<T> {
        let inner = angular_frequency.into_base() * self.into_base() / viscosity.into_base();
        Dimensionless::from_base(radius.into_base() * inner.powf(T::from_f64(0.5)))
    }
}

impl<T: FloatElement> Area<T> {
    /// Volumetric flow from a mean velocity: `Q = v * A`.
    #[inline]
    #[must_use]
    pub fn volumetric_flow(self, velocity: Velocity<T>) -> VolumetricFlowRate<T> {
        self * velocity
    }
}
