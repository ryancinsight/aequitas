use core::marker::PhantomData;

/// Seven-axis SI dimension vector.
///
/// The type parameters are type-level integer exponents in this order:
/// length, mass, time, electric current, thermodynamic temperature, amount of
/// substance, and luminous intensity. The marker occupies no storage.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Dimension<Length, Mass, Time, Current, Temperature, Amount, Luminosity>(
    PhantomData<(Length, Mass, Time, Current, Temperature, Amount, Luminosity)>,
);
