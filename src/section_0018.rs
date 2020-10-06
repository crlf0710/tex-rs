//! @ Characters of text that have been converted to \TeX's internal form
//! are said to be of type |ASCII_code|, which is a subrange of the integers.
//!
// @<Types...@>=
// @!ASCII_code=0..255; {eight-bit numbers}
//

/// eight-bit numbers
#[cfg(not(feature = "unicode_support"))]
pub struct ASCII_code(u8);

/// 32-bit internal form character code, compatible with ascii
#[cfg(feature = "unicode_support")]
pub struct ASCII_code(u32);

migration_complete!();
