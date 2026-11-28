//! Thermal-radiation laws over typed quantities.
//!
//! Wien peak positions, radiation pressure and photon energetics. Scalar
//! transcendence runs through [`eunomia::FloatElement`].
//!
//! The Wien constants are the numerical roots of the extremum conditions,
//! not SI-defining values: `2.8214393721220787` solves `x = 3(1 − e^(−x))`
//! for the frequency peak, and the wavelength peak follows from it through
//! the exact vacuum speed. No new dimensions are needed — every law
//! resolves through the existing quantity algebra.

use eunomia::FloatElement;

use super::super::constants::{BOLTZMANN_CONSTANT, PLANCK_CONSTANT, SPEED_OF_LIGHT};
use super::super::quantities::{
    Energy, Frequency, Intensity, Length, MolarFlux, Pressure, ThermodynamicTemperature,
};

impl<T: FloatElement> ThermodynamicTemperature<T> {
    /// Wien peak frequency `nu_max = x · k · T / h`.
    ///
    /// `x = 2.8214393721220787` is the root of the frequency-form extremum
    /// condition `x = 3(1 − e^(−x))`.
    #[inline]
    #[must_use]
    pub fn wien_peak_frequency(self) -> Frequency<T> {
        let x = T::from_f64(2.821_439_372_122_078_7);
        let rate = BOLTZMANN_CONSTANT.cast::<T>() * self / PLANCK_CONSTANT.cast::<T>();
        Frequency::from_base(x * rate.into_base())
    }

    /// Wien peak wavelength `lambda_max = h · c / (x · k · T)`.
    ///
    /// `x = 4.965114231744276` is the root of the wavelength-form extremum
    /// condition `x = 5(1 − e^(−x))`. This is NOT `c / nu_max`: the frequency
    /// peak (`x = 2.82144…`) and the wavelength peak (`x = 4.96511…`) are
    /// maxima of different distributions (`B_nu` vs `B_lambda`).
    #[inline]
    #[must_use]
    pub fn wien_peak_wavelength(self) -> Length<T> {
        let x = T::from_f64(4.965_114_231_744_276);
        let numerator =
            PLANCK_CONSTANT.cast::<T>().into_base() * SPEED_OF_LIGHT.cast::<T>().into_base();
        let denominator = x * BOLTZMANN_CONSTANT.cast::<T>().into_base() * self.into_base();
        Length::from_base(numerator / denominator)
    }
}

impl<T: FloatElement> Frequency<T> {
    /// Photon energy `E = h · nu`.
    #[inline]
    #[must_use]
    pub fn photon_energy(self) -> Energy<T> {
        PLANCK_CONSTANT.cast::<T>() * self
    }
}

impl<T: FloatElement> Intensity<T> {
    /// Radiation pressure on a perfect absorber `p = I / c`.
    #[inline]
    #[must_use]
    pub fn radiation_pressure(self) -> Pressure<T> {
        self / SPEED_OF_LIGHT.cast::<T>()
    }

    /// Molar photon flux `Phi = I / (h · nu · N_A)`.
    ///
    /// Power per area divided by the photon energy and the Avogadro number:
    /// moles of photons per square metre per second (einstein flux).
    #[inline]
    #[must_use]
    pub fn molar_photon_flux(self, frequency: Frequency<T>) -> MolarFlux<T> {
        use super::super::constants::AVOGADRO_CONSTANT;
        let photon_rate = self.into_base() / frequency.photon_energy().into_base();
        MolarFlux::from_base(photon_rate / AVOGADRO_CONSTANT.cast::<T>().into_base())
    }
}
