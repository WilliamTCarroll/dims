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
use dims::imperial::*;
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
let raw = mm.get_as(&INCH);
// The compiler will not allow you to add between systems:
let nope = mass + area; // <= Compiler throws an error
```
## System and Unit Creation
These particular systems are already set up already in `dims`, but you can set up your own systems:
```rust
use dims_core::unit_creation::*;
#[macro_use]
use dims_macro;

// You can derive the required attributes on this struct
#[derive(MeasureSystem)]
pub struct Length;
// Allow for conversion between the systems
// Multiple `MultiplyBy` and `DivideBy` traits can be applied for each `MeasureSystem`
impl MultiplyBy<Length> for Length {
    type Output = Area;
}

#[derive(MeasureSystem)]
pub struct Area;

impl DivideBy<Length> for Area {
    type Output = Length;
}

#[derive(MeasureSystem)]
pub struct Temperature;
// Set up some units, now

pub const FAHRENHEIT: UnitSimple<Temperature> = UnitSimple {
    /// The system is used for compiler warnings, but has no memory impact in production code
    system: PhantomData,
    // Offset is used to change the zero point.  Most of the time, this is zero
    offset: 459.67,
    // Ratio is multiplied to get to the base unit
    ratio: 5.0 / 9.0,
};
pub const CELCIUS: UnitSimple<Temperature> = UnitSimple {
    system: PhantomData,
    offset: 273.15,
    ratio: 1.0,
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

### Other Important Items
- `MultiplyBy` & `DivideBy`: Traits to allow for conversion between different unit systems.  EX: 
  - `Length` * `Length` = `Area` 
  - `Length` * `Area` = `Volume`
  - `Volume` / `Area` = `Length`

## dims
This contains a set of pre-made systems and units.  These will be added to as time goes on.

The currently set-up systems are

| System      | Base Unit   |
|-------------|-------------|
| Length      | Metre       |
| Area        | Sq Metre    |
| Volume      | Cubic Metre |
| Mass        | Gram        |
| Temperature | Kelvin      |

# Other Notes
## Performance
There is no measurable impact on **release** performance (from what my very basic tests can show).  The `Measure` struct is `[repr(transparent)]`, so everything but the value itself is optimized away.  **Debug mode code does have a hit to performance, however.**
## This crate is no_std
## Crate options for `dims` and `dims_core`
- `f32` is the default unit stored
- `f64` is available as an option
## Crate options for `dims`
- `si` provides SI/Metric units (on by default)
- `imperial` provides Imperial (or 'Merican) units (on by default)

## PLANNED FEATURES
- More units
- More systems
- More documentation
- macro to generate SI units from just the base unit