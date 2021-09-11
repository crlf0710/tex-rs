//! @ So that is what \.{TFM} files hold. Since \TeX\ has to absorb such information
//! about lots of fonts, it stores most of the data in a large array called
//! |font_info|. Each item of |font_info| is a |memory_word|; the |fix_word|
//! data gets converted into |scaled| entries, while everything else goes into
//! words of type |four_quarters|.
//!
//! When the user defines \.{\\font\\f}, say, \TeX\ assigns an internal number
//! to the user's font~\.{\\f}. Adding this number to |font_id_base| gives the
//! |eqtb| location of a ``frozen'' control sequence that will always select
//! the font.
//
// @<Types...@>=
// @!internal_font_number=font_base..font_max; {|font| in a |char_node|}
/// `font` in a `char_node`
pub(crate) type internal_font_number = u16_from_m_to_n<font_base_TYPENUM, font_max_TYPENUM>;
// @!font_index=0..font_mem_size; {index into |font_info|}
/// index into `font_info`
pub(crate) type font_index = u16_from_0_to_n<font_mem_size_TYPENUM>;
pub(crate) type font_index_repr = u16;

impl font_index {
    pub(crate) const fn zero() -> Self {
        unsafe { u16_from_0_to_n::new_unchecked(0) }
    }
}

use crate::pascal::{u16_from_0_to_n, u16_from_m_to_n};
use crate::section_0011::font_max_TYPENUM;
use crate::section_0011::font_mem_size_TYPENUM;
use crate::section_0012::font_base_TYPENUM;
