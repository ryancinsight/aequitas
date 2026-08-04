# 5. Scaled Units

*Chapter prose deferred — DoR item.*

<!-- Key points to cover:
  - The scaled units in systems::si::units::scaled (Millimeter, MHz, kPa, …)
  - How SCALE encodes SI prefix: Millimeter::SCALE = 1e-3 converts mm → m
  - Adding a custom scaled unit: one struct, Sealed + LinearUnit<D> impls,
    SCALE and SYMBOL constants — nothing else
  - Non-SI units with an exact SI definition (ElectronVolt::SCALE = 1.602…e-19)
  - Units that are semantically distinct but numerically identical at the
    dimension level (GrayPerSecond vs WattPerKilogram — both Gy/s = W/kg but
    different vocabulary for different dosimetry domains)
-->
