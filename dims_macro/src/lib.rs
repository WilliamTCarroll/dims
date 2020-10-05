pub use paste;

#[cfg(feature = "str")]
mod with_str;

#[cfg(not(feature = "str"))]
mod no_str;

mod si_macro;

#[macro_export]
/// Repeat the item presented x number of times. \
/// This is for internal use for generating si units. \
/// Each set must explicitly be noted, so this only goes up to 4 (will be expanded if required/requested).
macro_rules! repeat_item {
    ($item: expr, 4) => {
        $item * repeat_item!($item, 3)
    };
    ($item: expr, 3) => {
        $item * repeat_item!($item, 2)
    };
    ($item: expr, 2) => {
        $item * $item
    };
    ($item: expr, 1) => {
        $item
    };
}
