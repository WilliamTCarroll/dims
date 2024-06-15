# dims_macro

This will allow for easy creation of storage values like distance or mass, as well as preventing adding grams to feet.

The intent is for these values to be kept in-memory as `Mass` or `Length`, without having to worry about the unit except for creation of the value, and when accessing it (for display, storage, etc).

## IMPORTANT

This is still a Work-In-Progress. Expect rough-edges, but I am working to smooth them out.

## System and Unit Creation

These particular systems are already set up already in `dims`, but you can set up your own systems:

```rust
#[macro_use]
use dims_macro;

// You can use this macro to generate a full system.
// It requires a length for use as debug.
measure_system!(name: LengthSystem, debug_unit: FOOT,data_type: f32);
// Allow for conversion between the systems
// Multiple `MultiplyBy` and `DivideBy` traits can be applied for each `MeasureSystem`
impl MultiplyBy<LengthSystem> for LengthSystem {
    type Output = AreaSystem;
}

measure_system!(name: AreaSystem, debug_unit: SQUARE_FOOT,data_type: f32);

impl DivideBy<LengthSystem> for AreaSystem {
    type Output = LengthSystem;
}

measure_system!(name: TemperatureSystem, debug_unit: FAHRENHEIT,data_type: f32);

// Set up some units, now

pub const FAHRENHEIT: UnitSimple<TemperatureSystem> = UnitSimple {
    /// The system is used for compiler warnings, but has no memory impact in production code
    // Offset is used to change the zero point.  Most of the time, this is zero
    offset: 459.67,
    // Ratio is multiplied to get to the base unit
    ratio: 5.0 / 9.0,
};
pub const CELCIUS: UnitSimple<TemperatureSystem> = UnitSimple {
    offset: 273.15,
    ratio: 1.0,
};
pub const FOOT: UnitSimple<LengthSystem> = UnitSimple {
    offset: 0.0,
    ratio: 0.3048,
};
pub const SQUARE_FOOT: UnitSimple<AreaSystem> = UnitSimple {
    offset: 0.0,
    ratio: 0.09290304,
};
```

This also contains the `si_unit!` macro, which will generate a whole set (or individual) SI units with the given info.

TODO: EXAMPLE

### MeasureSystem macro

This is used to easily generate a new measuring system with
used as:

```rs
  measure_system! {name: Mass, debug_unit: GRAM, data_type: f32}
```

Expands to (with absolute paths):

```rs
#[derive(PartialEq, PartialOrd, Eq, Clone, Copy)]
pub struct Mass;
impl MeasureSystem for Mass {
    type N = f32;
    #[cfg(feature = "str")]
    const DEBUG_UNIT: UnitFormat<Self> = GRAM;
}
```

The `DEBUG_UNIT` is how the value will be displayed when debugging. This _could_ be how you want to display it, but that should be specified explicitly by consuming code.

# Other Notes

## Crate options for `dims` and `dims_core`

## Crate options for `dims` and `dims_macro`

- `str` (default) will utilize `UnitFormat` and store:

  - `abbr`: Abbreviated unit name (`m` or `ft`)
  - `singular`: Singular name of a unit (`metre` or `foot`)
  - `plural`: Plural name of the unit (`metres` or `feet`)

- `std` is the default option, and defaults to using the standard library.\
  This enables the `UnitFormatTrait` (as the functions return `String`), but `no_std` can still be used with `str` (see below) to store the unit name info.
