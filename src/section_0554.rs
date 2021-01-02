//! @ Of course we want to define macros that suppress the detail of how font
//! information is actually packed, so that we don't have to write things like
//! $$\hbox{|font_info[width_base[f]+font_info[char_base[f]+c].qqqq.b0].sc|}$$
//! too often. The \.{WEB} definitions here make |char_info(f)(c)| the
//! |four_quarters| word of font information corresponding to character
//! |c| of font |f|. If |q| is such a word, |char_width(f)(q)| will be
//! the character's width; hence the long formula above is at least
//! abbreviated to
//! $$\hbox{|char_width(f)(char_info(f)(c))|.}$$
//! Usually, of course, we will fetch |q| first and look at several of its
//! fields at the same time.
//!
//! The italic correction of a character will be denoted by
//! |char_italic(f)(q)|, so it is analogous to |char_width|.  But we will get
//! at the height and depth in a slightly different way, since we usually want
//! to compute both height and depth if we want either one.  The value of
//! |height_depth(q)| will be the 8-bit quantity
//! $$b=|height_index|\times16+|depth_index|,$$ and if |b| is such a byte we
//! will write |char_height(f)(b)| and |char_depth(f)(b)| for the height and
//! depth of the character |c| for which |q=char_info(f)(c)|. Got that?
//!
//! The tag field will be called |char_tag(q)|; the remainder byte will be
//! called |rem_byte(q)|, using a macro that we have already defined above.
//!
//! Access to a character's |width|, |height|, |depth|, and |tag| fields is
//! part of \TeX's inner loop, so we want these macros to produce code that is
//! as fast as possible under the circumstances.
//! @^inner loop@>

#[derive(Copy, Clone, RefCast)]
#[repr(transparent)]
pub(crate) struct char_info(four_quarters);

// @d char_info_end(#)==#].qqqq
// @d char_info(#)==font_info[char_base[#]+char_info_end
macro_rules! char_info {
    ($globals:expr, $f:expr, $c:expr) => {
        $globals.font_info[$globals.char_base[$f] as pointer + $c as pointer][crate::section_0554::MEMORY_WORD_CHAR_INFO]
    };
}
// @d char_width_end(#)==#.b0].sc
// @d char_width(#)==font_info[width_base[#]+char_width_end
macro_rules! char_width {
    ($globals:expr, $f:expr, $c:expr) => {
        $globals.font_info[$globals.width_base[$f] as pointer + $c.width() as pointer][crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d char_exists(#)==(#.b0>min_quarterword)
impl char_info {
    pub(crate) fn char_exists(self) -> bool {
        self.0[FOUR_QUARTERS_B0] > min_quarterword
    }
}
// @d char_italic_end(#)==(qo(#.b2)) div 4].sc
// @d char_italic(#)==font_info[italic_base[#]+char_italic_end
// @d height_depth(#)==qo(#.b1)
impl char_info {
    pub(crate) fn width(self) -> eight_bits {
        qo!(self.0[FOUR_QUARTERS_B0])
    }
    pub(crate) fn height_depth(self) -> eight_bits {
        qo!(self.0[FOUR_QUARTERS_B1])
    }
}
// @d char_height_end(#)==(#) div 16].sc
// @d char_height(#)==font_info[height_base[#]+char_height_end
macro_rules! char_height {
    ($globals:expr, $f:expr, $c:expr) => {
        $globals.font_info[$globals.height_base[$f] as pointer + ($c / 16) as pointer]
            [crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d char_depth_end(#)==(#) mod 16].sc
// @d char_depth(#)==font_info[depth_base[#]+char_depth_end
macro_rules! char_depth {
    ($globals:expr, $f:expr, $c:expr) => {
        $globals.font_info[$globals.depth_base[$f] as pointer + ($c % 16) as pointer]
            [crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d char_tag(#)==((qo(#.b2)) mod 4)

impl char_info {
    pub(crate) fn char_tag(self) -> char_tag {
        let b2 = qo!((self.0)[FOUR_QUARTERS_B2]);
        (b2 % 4).into()
    }

    pub(crate) fn rem_byte(self) -> quarterword {
        self.0[FOUR_QUARTERS_B3]
    }
}

impl Default for char_info {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub(crate) struct MEMORY_WORD_CHAR_INFO;

impl Index<MEMORY_WORD_CHAR_INFO> for memory_word {
    type Output = char_info;
    fn index(&self, _: MEMORY_WORD_CHAR_INFO) -> &char_info {
        char_info::ref_cast(&self[MEMORY_WORD_QQQQ])
    }
}

impl IndexMut<MEMORY_WORD_CHAR_INFO> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_CHAR_INFO) -> &mut char_info {
        char_info::ref_cast_mut(&mut self[MEMORY_WORD_QQQQ])
    }
}

use crate::section_0025::eight_bits;
use crate::section_0110::min_quarterword;
use crate::section_0113::quarterword;
use crate::section_0113::four_quarters;
use crate::section_0113::memory_word;
use crate::section_0113::FOUR_QUARTERS_B0;
use crate::section_0113::FOUR_QUARTERS_B1;
use crate::section_0113::FOUR_QUARTERS_B2;
use crate::section_0113::FOUR_QUARTERS_B3;
use crate::section_0113::MEMORY_WORD_QQQQ;
use crate::section_0544::char_tag;
use core::ops::{Index, IndexMut};
use ref_cast::RefCast;
