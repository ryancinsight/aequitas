//! The macro the generated class inventory expands through.

/// Declare one `Quantity` subclass per named dimension and the table
/// `Classed` consults when a quantity crosses into Python.
///
/// Input is `Class: dimensions::Dimension` per class, optionally followed by
/// `{ Alias: dimensions::AliasDimension, ... }` for the dimension's other
/// Aequitas aliases. Two compile-time assertions keep the generator honest:
/// every alias must have its class's dimension, and no two classes may share
/// one. Tags are read from the Rust types, never written here.
#[macro_export]
macro_rules! quantity_classes {
    ($($class:ident: $dimension:ty $({ $($alias:ident: $alias_dimension:ty,)+ })?,)+) => {
        $(
            #[doc = concat!("A quantity of the `", stringify!($class), "` dimension.")]
            #[::pyo3::pyclass(
                extends = $crate::quantity::PyQuantity,
                frozen,
                module = "pyaequitas._pyaequitas"
            )]
            pub(crate) struct $class;

            $($(
                const _: () = assert!(
                    <$alias_dimension as $crate::tag::TaggedDimension>::TAG
                        .same_as(<$dimension as $crate::tag::TaggedDimension>::TAG),
                    concat!(
                        "`", stringify!($alias), "` is grouped under `", stringify!($class),
                        "` but names a different dimension"
                    ),
                );
            )+)?
        )+

        /// Every generated class, in inventory order.
        pub(crate) const CLASSES: &[$crate::quantity::classes::Class] = &[
            $(
                $crate::quantity::classes::Class {
                    name: stringify!($class),
                    tag: <$dimension as $crate::tag::TaggedDimension>::TAG,
                    aliases: &[$($(stringify!($alias),)+)?],
                    instantiate: |py, quantity| {
                        ::pyo3::Bound::new(
                            py,
                            ::pyo3::PyClassInitializer::from(quantity).add_subclass($class),
                        )
                        .map(::pyo3::Bound::into_any)
                    },
                    register: |module| {
                        ::pyo3::types::PyModuleMethods::add_class::<$class>(module)
                    },
                },
            )+
        ];

        const _: () = assert!(
            $crate::quantity::classes::tags_distinct(CLASSES),
            "two generated classes share a dimension; the generator grouped wrongly",
        );
    };
}
