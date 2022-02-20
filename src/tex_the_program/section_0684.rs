//! @ The global variable |empty_field| is set up for initialization of empty
//! fields in new noads. Similarly, |null_delimiter| is for the initialization
//! of delimiter fields.
const empty: halfword = 0;
// @<Glob...@>=
// @!empty_field:two_halves;
pub(crate) const empty_field: two_halves = two_halves::new_with_halves(null, empty);
// @!null_delimiter:four_quarters;
#[cfg(not(feature = "unicode_support"))]
pub(crate) fn null_delimiter(globals: &mut TeXGlobals) -> memory_word {
    let mut result = memory_word::default();
    result[crate::section_0113::MEMORY_WORD_QQQQ] =
        four_quarters::new_with_quarters(0, min_quarterword, 0, min_quarterword);
    result
}

#[cfg(feature = "unicode_support")]
pub(crate) fn null_delimiter(globals: &mut TeXGlobals) -> memory_word {
    let mut result = memory_word::default();
    result[crate::section_0113::MEMORY_WORD_HH_LH] =
        crate::unicode_support::register_fontchar_value(
            globals,
            crate::section_0134::font_and_character {
                font: 0.into(),
                character: (min_quarterword as integer).into(),
            },
        );
    result[crate::section_0113::MEMORY_WORD_HH_RH] =
        crate::unicode_support::register_fontchar_value(
            globals,
            crate::section_0134::font_and_character {
                font: 0.into(),
                character: (min_quarterword as integer).into(),
            },
        );
    result
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0110::min_quarterword;
use crate::section_0113::four_quarters;
use crate::section_0113::halfword;
use crate::section_0113::memory_word;
use crate::section_0113::two_halves;
use crate::section_0115::null;
