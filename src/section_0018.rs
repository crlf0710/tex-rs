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

#[cfg(not(feature = "unicode_support"))]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
/// eight-bit numbers
pub struct ASCII_code(pub(crate) ASCII_code_repr);

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
/// 32-bit internal form character code, compatible with ascii
pub struct ASCII_code(pub(crate) runestr::rune);

impl PartialOrd for ASCII_code {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.into_inner().partial_cmp(&other.0.into_inner())
    }
}

impl ASCII_code {
    pub(crate) fn numeric_value(self) -> ASCII_code_repr {
        #[cfg(not(feature = "unicode_support"))]
        {
            self.0
        }
        #[cfg(feature = "unicode_support")]
        {
            self.0.into_inner()
        }
    }
    pub(crate) fn from_integer(val: integer) -> Self {
        #[cfg(not(feature = "unicode_support"))]
        {
            assert!(val >=0 && val <= 255);
            ASCII_code(val as _)
        }
        #[cfg(feature = "unicode_support")]
        {
            assert!(val >= 0);
            if val == '\r' as u32 as integer {
                ASCII_code(runestr::rune::from_char_lossy('\r'))
            } else {
                ASCII_code(runestr::rune::from_inner(val as _).unwrap())
            }
        }
    }
}

migration_complete!();

macro_rules! ASCII_code_literal {
    ($val:expr) => {{
        use crate::section_0018::ASCII_code;

        let val: u8 = $val;
        ASCII_code(runestr::rune::from_char_lossy(val as char))
    }};
}

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
        self.0.len_runestr()
    }
}

use crate::pascal::integer;