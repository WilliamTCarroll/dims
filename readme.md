# dims

This collection of crates are used to set up type-safe measurements.

This will allow for storing values like distance or mass, as well as preventing adding grams to feet.

The intent is for these values to be kept in-memory as `Mass` or `Length`, without having to worry about the unit except for creation of the value, and when accessing it (for display, storage, etc).

## IMPORTANT

This is still a Work-In-Progress. Expect rough-edges, but I am working to smooth them out.

## Stored Crates

1. `dims_core`: The unit and system creation traits. You only need this if you wish to generate your own units, systems, etc
2. `dims`: Pre-generated units, covering the gambit of the most commonly used `us` and `si` units (and some less common, like [Hogshead](https://en.wikipedia.org/wiki/Hogshead)).
3. `dims_macro`: Macros for ease of use generating units. A simple macro (`si_unit`) to create a full SI system ranging from `yotta` to `yocto` is also included

# Other Notes

## Performance

There is no measurable impact on **release** performance compared to the stored value (from what my very basic tests can show). The `Measure` struct is `[repr(transparent)]`, so everything but the value itself is optimized away. **Debug mode code does have a hit to performance, however.**

## PLANNED FEATURES

- More units
- More systems
- More documentation
- ~~Tests for US units~~
- ~~macro to generate SI units from just the base unit~~
- ~~Generic storage type (allowing any numeric type)~~
- Compound units more generic (EX: Mass/Volume for Density)
