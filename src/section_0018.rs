//! @ Characters of text that have been converted to \TeX's internal form
//! are said to be of type |ASCII_code|, which is a subrange of the integers.
//!
// @<Types...@>=
// @!ASCII_code=0..255; {eight-bit numbers}
//

/// eight-bit numbers
#[cfg(not(feature = "unicode_support"))]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct ASCII_code(pub(crate) u8);

#[cfg(not(feature = "unicode_support"))]
impl ASCII_code {
    pub(crate) fn numeric_value(self) -> u8 {
        self.0
    }
}

/// 32-bit internal form character code, compatible with ascii
#[cfg(feature = "unicode_support")]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct ASCII_code(pub(crate) u32);

#[cfg(feature = "unicode_support")]
impl ASCII_code {
    pub(crate) fn numeric_value(self) -> u32 {
        self.0
    }
}
migration_complete!();

macro_rules! ASCII_code_literal {
    ($val:expr) => {{
        use crate::section_0018::ASCII_code;

        let val: u8 = $val;
        ASCII_code(val as _)
    }};
}

impl From<integer> for ASCII_code {
    fn from(val: integer) -> Self {
        #[cfg(not(feature = "unicode_support"))]
        {
            assert!(val >=0 && val <= 255);
            ASCII_code(val as _)
        }
        #[cfg(feature = "unicode_support")]
        {
            if val >= 0 && val <= 255
            {
                ASCII_code(val as _)
            }
            else
            {
                todo!();
            }
        }
    }
}

use crate::pascal::integer;