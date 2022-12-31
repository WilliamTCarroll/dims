# dims_core

This collection of crates are used to set up type-safe measurements.

This will allow for creating storage types for values like distance or mass, as well as preventing adding grams to feet.

The intent is for these values to be kept in-memory as `Mass` or `Length`, without having to worry about the unit except for creation of the value, and when accessing it (for display, storage, etc).

## IMPORTANT

This is still a Work-In-Progress. Expect rough-edges, but I am working to smooth them out.

## System and Unit Creation

These particular systems are already set up already in `dims`, but you can set up your own systems

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

---

- The core components used to build a set of units.
- No actual units (or even measurement systems) are stored, here.
- This crate should only be used directly if you want to make your own `Unit` or `MeasureSystem`.
- The stored value is not specified, but left generic. The most common options would likely be `f32` and `f64` (as used in the `dims` crate, here), but this is left generic.

### Base Structs and Traits

- `MeasureSystem`: A system of measurement (EX: Length, Mass, Volume) used to prevent cross-over between a `Measure`. These are implemented in `dims`
- `Measure`: The actual value stored stored. Any value created (like inches or pounds) will be converted to the base unit (EX: Metre for Length, Gram for Mass).
- `UnitTrait`: A trait used to create a type of unit. This allows for creation of a custom conversion function between units. For nearly all situations, `UnitSimple` can be used.
- `UnitSimple`: A basic implementation of `UnitTrait`. The ratio between the specified unit and the base unit are specified, as well as the offset.
- `UnitFormat`: A bit more complex implementation of `UnitTrait`. The ratio between the specified unit and the base unit are specified, as well as the offset. In addition, the textual name for the unit is stored.

### Other Important Items

- `MultiplyBy` & `DivideBy`: Traits to allow for conversion between different unit systems. EX:
  - `Length` \* `Length` = `Area`
  - `Length` \* `Area` = `Volume`
  - `Volume` / `Area` = `Length`
- You can also multiply and divide by the stored datatype. EX:

```rust
    let len = MM.from(8.0);
    let two = len * 2.0;
    let one = two / 2.0;
```

The `DEBUG_UNIT` is how the value will be displayed when debugging. This _could_ be how you want to display it, but that should be specified explicitly by consuming code.

# Other Notes

## Performance

There is no measurable impact on **release** performance compared to the stored value (from what my very basic tests can show). The `Measure` struct is `[repr(transparent)]`, so everything but the value itself is optimized away. **Debug mode code does have a hit to performance, however.**

## Crate options

- `std` is the default option, and defaults to using the standard library.\
  This enables the `UnitFormatTrait` (as the functions return `String`), but `no_std` can still be used with `str` (see below) to store the unit name info.
- `str` (default) will utilize `UnitFormat` and store:
  - `abbr`: Abbreviated unit name (`m` or `ft`)
  - `singular`: Singular name of a unit (`metre` or `foot`)
  - `plural`: Plural name of the unit (`metres` or `feet`)
