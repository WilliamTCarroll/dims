# dims

This will allow for storing values like distance or mass, as well as preventing adding grams to feet.

The intent is for these values to be kept in-memory as `Mass` or `Length`, without having to worry about the unit except for creation of the value, and when accessing it (for display, storage, etc).

## IMPORTANT

This is still a Work-In-Progress. Expect rough-edges, but I am working to smooth them out.

# Usage

If you just want to use the pre-generated units (Work in progress: creating more):

```rust
// Be sure you include prelude; this exposes required traits
use dims::prelude::*;
use dims::us::*;
use dims::si::*;

let feet = FOOT.from(12.5);
let mm = MILLIMETRE.from(317.5);
assert_eq!(feet, mm);
// You can add between different systems
// They are all stored as `metre` here, anyway
let another = feet + mm;
// You can also multiply to get area (and on further to get volume)
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

### Base Structs and Traits

- `MeasureSystem`: A system of measurement (EX: Length, Mass, Volume) used to prevent cross-over between a `Measure`. These are implemented in `dims`
- `Measure`: The actual value stored stored. Any value created (like inches or pounds) will be converted to the base unit (EX: Metre for Length, Gram for Mass).
- `UnitTrait`: A trait used to create a type of unit. This allows for creation of a custom conversion function between units. For nearly all situations, `UnitSimple` can be used.
- `UnitSimple`: A basic implementation of `UnitTrait`. The ratio between the specified unit and the base unit are specified, as well as the offset.
- `UnitFormat`: A bit more complex implementation of `UnitTrait`. The ratio between the specified unit and the base unit are specified, as well as the offset. In addition, the textual name for the unit is stored.

# dims

---

This contains a set of pre-made systems and units. These will be added to as time goes on.

The current systems are:

| System      | Base Unit   | debug_us    |
| ----------- | ----------- | ----------- |
| Length      | Metre       | Inch        |
| Area        | Sq Metre    | Square Inch |
| Volume      | Cubic Metre | Cubic Inch  |
| Mass        | Gram        | Pound       |
| Temperature | Kelvin\*    | Fahrenheit  |

\*Notes on Temperature:

- No checking for negative values is performed, as endothermic reactions would have a negative value when applied to the environment.
- The debug unit for this is Celcius

# Other Notes

## Performance

There is no measurable impact on **release** performance compared to the stored value (from what my very basic tests can show). The `Measure` struct is `[repr(transparent)]`, so everything but the value itself is optimized away. **Debug mode code does have a hit to performance, however.**

## Crate options for `dims`

- `f64` is available as an option; `f32` is the default. Other datatypes can be specified when using custom units (see `dims_core`)
- `std` is the default option, and defaults to using the standard library.\
  This enables the `UnitFormatTrait` (as the functions return `String`), but `no_std` can still be used with `str` (see below) to store the unit name info.
- `str` (default) will utilize `UnitFormat` and store:
  - `abbr`: Abbreviated unit name (`m` or `ft`)
  - `singular`: Singular name of a unit (`metre` or `foot`)
  - `plural`: Plural name of the unit (`metres` or `feet`)
- `si` provides SI/Metric units (on by default)
- `us` provides US (or 'Merican) units (on by default)
- `debug_us` will use units as specified in the table above for debugging.

### IMPORTANT

Selecting `us` or `debug_us` in the above options will NOT change the base unit. It will still be stored in SI.
