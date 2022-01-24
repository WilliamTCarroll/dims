# dims
This collection of crates are used to set up type-safe measurements.

This will allow for storing values like distance or mass, as well as preventing adding grams to feet.

## IMPORTANT
This is still a Work-In-Progress.  Expect rough-edges, but I am working to smooth them out.

# Usage
If you just want to use the pre-generated units (Work in progress: creating more):
```rust
// Be sure you include prelude; this exposes required traits
use dims::prelude::*;
use dims::us::*;
use dims::si::*;

let feet = FOOT.from(12.5);
let mm = MM.from(317.5);
assert_eq!(feet, mm);
// You can add between different systems
// They are all stored as `metre` here, anyway
let another = feet + mm;
// You can also multiply to get area
let area = feet * mm;
assert_eq!(area, SQFT.from(156.25));
let mass = GRAM.from(18.25);
// You can also create them the other way around
let this_works = Measure::new(&INCH,0.125);
// You can grab the stored value as a float via
let raw = mm.val_as(&INCH);
// The compiler will not allow you to add between systems:
// let nope = mass + area; // <== Compiler throws an error
```
## Usage in functions or structs
The `dims` crate contains type aliases for each generic measure.  EX:
```rust
pub type Length = Measure<'static, LengthSystem>;
```
These are included in the mod `prelude`, or you could import `unit_type`
```rust
use dims::prelude::{Length, Area};
MultiplyBy
fn do_something(len1: Length, len2: Length) -> Area {
    // ...
}
```
## System and Unit Creation
These particular systems are already set up already in `dims`, but you can set up your own systems:
```rust
use dims_core::unit_creation::*;
#[macro_use]
use dims_macro;

// You can use this macro to generate a full system.
// It requires a length for use as debug.
measure_system!(name: LengthSystem, debug_unit: FOOT);
// Allow for conversion between the systems
// Multiple `MultiplyBy` and `DivideBy` traits can be applied for each `MeasureSystem`
impl MultiplyBy<LengthSystem> for LengthSystem {
    type Output = AreaSystem;
}

measure_system!(name: LengthSystem, debug_unit: SQUARE_FOOT);

impl DivideBy<LengthSystem> for AreaSystem {
    type Output = LengthSystem;
}

measure_system!(name: TemperatureSystem, debug_unit: FAHRENHEIT);

// Set up some units, now

pub const FAHRENHEIT: UnitSimple<TemperatureSystem> = UnitSimple {
    /// The system is used for compiler warnings, but has no memory impact in production code
    system: PhantomData,
    // Offset is used to change the zero point.  Most of the time, this is zero
    offset: 459.67,
    // Ratio is multiplied to get to the base unit
    ratio: 5.0 / 9.0,
};
pub const CELCIUS: UnitSimple<TemperatureSystem> = UnitSimple {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
};
pub const FOOT: UnitSimple<LengthSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.3048,
};
pub const SQUARE_FOOT: UnitSimple<AreaSystem> = UnitSimple {
    system: PhantomData,
    offset: 0.0,
    ratio: 0.09290304,
};
```

The conversion formulae used for `UnitSimple` are as follows:
```rust
    /// Convert the given value from this unit into the base
    fn to_base(&self, val: Flt) -> Flt {
        (val + self.offset) * self.ratio
    }
    /// Convert the given value (as the base unit) into this unit
    fn to_self(&self, val: Flt) -> Flt {
        (val / self.ratio) - self.offset
    }
```
This allows for basic units with the same zero (most of them), as well as those with different zero points (temperature).

# Crates Stored

## dims_core
___
The core components used to build a set of units.\
No actual units (or even measurement systems) are stored, here.\
This crate should only be used directly if you want to make your own `Unit` or `MeasureSystem`
### Base Structs and Traits
- `MeasureSystem`: A system of measurement (EX: Length, Mass, Volume) used to prevent cross-over between a `Measure`.  These are implemented in `dims`
- `Measure`: The actual value stored stored.  Any value created (like inches or pounds) will be converted to the base unit (EX: Metre for Length, Gram for Mass).
- `UnitTrait`: A trait used to create a type of unit.  This allows for creation of a custom conversion function between units.  For nearly all situations, `UnitSimple` can be used.
- `UnitSimple`: A basic implementation of `UnitTrait`.  The ratio between the specified unit and the base unit are specified, as well as the offset.
- `UnitFormat`: A bit more complex implementation of `UnitTrait`.  The ratio between the specified unit and the base unit are specified, as well as the offset.  In addition, the textual name for the unit is stored.

### Other Important Items
- `MultiplyBy` & `DivideBy`: Traits to allow for conversion between different unit systems.  EX: 
  - `Length` * `Length` = `Area` 
  - `Length` * `Area` = `Volume`
  - `Volume` / `Area` = `Length`
## dims_macro
This contains the `si_unit!` macro, which will generate a whole set (or individual) SI units with the given info.
TODO: EXAMPLE

### MeasureSystem macro
This is used to easily generate a new measuring system with 
used as:
```rs
  measure_system! {name: Mass, debug_unit: GRAM}
```
Expands to (with absolute paths):
```rs
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct Mass;
    impl<'t> MeasureSystem<'t> for Mass {
        #[cfg(feature = "str")]
        const DEBUG_UNIT: UnitFormat<'t, Self> = GRAM;
    }
```
The `DEBUG_UNIT` is how the value will be displayed when debugging.  This *could* be how you want to display it, but that should be specified explicitly by consuming code.
## dims
This contains a set of pre-made systems and units.  These will be added to as time goes on.

The current systems are:

| System      | Base Unit   | debug_us    |
|-------------|-------------|-------------|
| Length      | Metre       | Inch        |
| Area        | Sq Metre    | Square Inch |
| Volume      | Cubic Metre | Cubic Inch  |
| Mass        | Gram        | Pound       |
| Temperature | Kelvin*     | Fahrenheit  |

*Notes on Temperature:
-  No checking for negative values is performed, as endothermic reactions would have a negative value when applied to the environment. 
- The debug unit for this is Celcius

# Other Notes
## Performance
There is no measurable impact on **release** performance compared to the stored value (from what my very basic tests can show).  The `Measure` struct is `[repr(transparent)]`, so everything but the value itself is optimized away.  **Debug mode code does have a hit to performance, however.**
## Crate options for `dims` and `dims_core`
- `f64` is available as an option; `f32` is the default
- `no_std` will set the crates to `no_std` (go figure).\
This disables the `UnitFormatTrait` (as the functions return `String`), but can still be used with `str` (see below) to store the unit name info.
## Crate options for `dims`
- `str` (default) will utilize `UnitFormat` and store:
  - `abbr`: Abbreviated unit name (`m` or `ft`)
  - `singular`: Singular name of a unit (`metre` or `foot`)
  - `plural`: Plural name of the unit (`metres` or `feet`)
- `si` provides SI/Metric units (on by default)
- `us` provides us (or 'Merican) units (on by default)
- `debug_us` will use units as specified in the table above for debugging.

### IMPORTANT
Selecting `us` in the above options will NOT change the base unit.  It will still be stored in SI.

## PLANNED FEATURES
- More units
- More systems
- More documentation
- ~Tests for US units~
- ~~macro to generate SI units from just the base unit~~
- Generic storage type (allowing any numeric type)
- Compound units more generic (EX: Mass/Volume for Density)