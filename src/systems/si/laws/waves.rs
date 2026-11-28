//! Electromagnetic wave-propagation laws over typed quantities.
//!
//! Relative-medium speed, skin depth, plasma cutoff and the Poynting
//! magnitude. Scalar transcendence runs through
//! [`eunomia::FloatElement`].

use eunomia::FloatElement;

use super::super::constants::{
    ELECTRON_MASS, ELEMENTARY_CHARGE, VACUUM_PERMEABILITY, VACUUM_PERMITTIVITY,
};
use super::super::quantities::{
    AngularVelocity, Dimensionless, ElectricFieldStrength, ElectricalConductivity, Frequency,
    Intensity, Length, MagneticPermeability, NumberDensity, Permittivity,
};

impl<T: FloatElement> AngularVelocity<T> {
    /// Skin depth `delta = sqrt(2 / (omega · mu · sigma))`.
    ///
    /// Good-conductor attenuation length for a harmonic field; takes the
    /// (possibly relative-scaled) permeability and the conductivity.
    #[inline]
    #[must_use]
    pub fn skin_depth(
        self,
        permeability: MagneticPermeability<T>,
        conductivity: ElectricalConductivity<T>,
    ) -> Length<T> {
        let denominator = self.into_base() * permeability.into_base() * conductivity.into_base();
        Length::from_base((T::from_f64(2.0) / denominator).powf(T::from_f64(0.5)))
    }
}

impl<T: FloatElement> NumberDensity<T> {
    /// Plasma angular cutoff `omega_p = sqrt(n · e² / (eps₀ · m_e))`.
    ///
    /// Uses the exact elementary charge, vacuum permittivity and electron
    /// mass; the density is the electron number density.
    #[inline]
    #[must_use]
    pub fn plasma_frequency(self) -> AngularVelocity<T> {
        let charge = ELEMENTARY_CHARGE.cast::<T>().into_base();
        let inner = self.into_base() * charge * charge
            / (VACUUM_PERMITTIVITY.cast::<T>().into_base() * ELECTRON_MASS.cast::<T>().into_base());
        AngularVelocity::from_base(inner.powf(T::from_f64(0.5)))
    }
}

impl<T: FloatElement> ElectricFieldStrength<T> {
    /// Time-averaged Poynting magnitude `S = E_rms² / Z`.
    ///
    /// Plane wave in a medium of wave impedance `Z`; `self` is the RMS
    /// electric-field amplitude.
    #[inline]
    #[must_use]
    pub fn poynting_magnitude(
        self,
        impedance: super::super::quantities::ElectricalImpedance<T>,
    ) -> Intensity<T> {
        let e = self.into_base();
        Intensity::from_base(e * e / impedance.into_base())
    }
}

impl<T: FloatElement> Permittivity<T> {
    /// Phase speed in a non-magnetic dielectric `v = c₀ / sqrt(eps_r)`.
    ///
    /// Takes the relative permittivity; the magnetic response stays vacuum.
    /// An associated function: the absolute permittivity scale is fixed by
    /// the exact vacuum constants, so no receiver value enters.
    #[inline]
    #[must_use]
    pub fn dielectric_wave_speed(
        relative_permittivity: Dimensionless<T>,
    ) -> super::super::quantities::Velocity<T> {
        let vacuum = VACUUM_PERMITTIVITY
            .cast::<T>()
            .vacuum_wave_speed(VACUUM_PERMEABILITY.cast::<T>());
        super::super::quantities::Velocity::from_base(
            vacuum.into_base() / relative_permittivity.into_base().powf(T::from_f64(0.5)),
        )
    }
}

impl<T: FloatElement> Frequency<T> {
    /// Free-space wavelength `lambda = c₀ / f`, stated for the wave
    /// frequency rather than an acoustic source.
    #[inline]
    #[must_use]
    pub fn free_space_wavelength(self) -> Length<T> {
        use super::super::constants::SPEED_OF_LIGHT;
        SPEED_OF_LIGHT.cast::<T>() / self
    }
}
