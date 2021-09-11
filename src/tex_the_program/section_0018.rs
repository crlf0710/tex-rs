//! @ Characters of text that have been converted to \TeX's internal form
//! are said to be of type |ASCII_code|, which is a subrange of the integers.
//!
// @<Types...@>=
// @!ASCII_code=0..255; {eight-bit numbers}
//

#[cfg(not(feature = "unicode_support"))]
pub(crate) type ASCII_code_repr = u8;

#[cfg(feature = "unicode_support")]
pub(crate) type ASCII_code_repr = u32;

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
/// When `unicode_support` feature is not enabled, these are eight-bit numbers
/// otherwise, it is a 32-bit internal form character code, compatible with ascii
pub struct ASCII_code(pub(crate) ASCII_code_repr);

impl ASCII_code {
    pub(crate) fn numeric_value(self) -> ASCII_code_repr {
        self.0
    }
    pub(crate) fn from_integer(val: integer) -> Self {
        #[cfg(not(feature = "unicode_support"))]
        {
            assert!(val >= 0 && val <= 255);
            ASCII_code(val as _)
        }
        #[cfg(feature = "unicode_support")]
        {
            if val >= 0 && val <= 255 {
                ASCII_code(val as _)
            } else {
                todo!("{} is > 255", val);
            }
        }
    }
}

crate::migration_complete!();

pub(crate) macro ASCII_code_literal($val:expr) {{
    use crate::section_0018::ASCII_code;

    let val: u8 = $val;
    ASCII_code(val as _)
}}

impl From<integer> for ASCII_code {
    fn from(val: integer) -> Self {
        Self::from_integer(val)
    }
}

impl ASCII_code {
    #[cfg(not(feature = "unicode_support"))]
    pub(crate) fn len_bytes(self) -> usize {
        1
    }

    #[cfg(feature = "unicode_support")]
    pub(crate) fn len_bytes(self) -> usize {
        crate::unicode_support::len_fss_utf(self.0 as integer)
    }
}

use crate::pascal::integer;
