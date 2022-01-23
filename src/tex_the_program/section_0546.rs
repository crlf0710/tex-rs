//! @ Extensible characters are specified by an |@!extensible_recipe|, which
//! consists of four bytes called |@!top|, |@!mid|, |@!bot|, and |@!rep| (in this
//! order). These bytes are the character codes of individual pieces used to
//! build up a large symbol.  If |top|, |mid|, or |bot| are zero, they are not
//! present in the built-up result. For example, an extensible vertical line is
//! like an extensible bracket, except that the top and bottom pieces are missing.
//!
//! Let $T$, $M$, $B$, and $R$ denote the respective pieces, or an empty box
//! if the piece isn't present. Then the extensible characters have the form
//! $TR^kMR^kB$ from top to bottom, for some |k>=0|, unless $M$ is absent;
//! in the latter case we can have $TR^kB$ for both even and odd values of~|k|.
//! The width of the extensible character is the width of $R$; and the
//! height-plus-depth is the sum of the individual height-plus-depths of the
//! components used, since the pieces are butted together in a vertical list.
//
// @d ext_top(#)==#.b0 {|top| piece in a recipe}
// @d ext_mid(#)==#.b1 {|mid| piece in a recipe}
// @d ext_bot(#)==#.b2 {|bot| piece in a recipe}
// @d ext_rep(#)==#.b3 {|rep| piece in a recipe}

#[derive(Copy, Clone, RefCast)]
#[repr(transparent)]
pub(crate) struct extensible_recipe(four_quarters);

impl extensible_recipe {
    /// `top` piece in a recipe
    pub(crate) fn ext_top(self) -> quarterword {
        self.0[FOUR_QUARTERS_B0]
    }
    /// `mid` piece in a recipe
    pub(crate) fn ext_mid(self) -> quarterword {
        self.0[FOUR_QUARTERS_B1]
    }
    /// `bot` piece in a recipe
    pub(crate) fn ext_bot(self) -> quarterword {
        self.0[FOUR_QUARTERS_B2]
    }
    /// `rep` piece in a recipe
    pub(crate) fn ext_rep(self) -> quarterword {
        self.0[FOUR_QUARTERS_B3]
    }
}

use crate::section_0113::four_quarters;
use crate::section_0113::memory_word;
use crate::section_0113::quarterword;
use crate::section_0113::MEMORY_WORD_QQQQ;
use crate::section_0113::{FOUR_QUARTERS_B0, FOUR_QUARTERS_B1, FOUR_QUARTERS_B2, FOUR_QUARTERS_B3};
use core::ops::{Index, IndexMut};
use ref_cast::RefCast;

pub(crate) struct MEMORY_WORD_EXTENSIBLE_RECIPE;

impl Index<MEMORY_WORD_EXTENSIBLE_RECIPE> for memory_word {
    type Output = extensible_recipe;
    fn index(&self, _: MEMORY_WORD_EXTENSIBLE_RECIPE) -> &extensible_recipe {
        extensible_recipe::ref_cast(&self[MEMORY_WORD_QQQQ])
    }
}

impl IndexMut<MEMORY_WORD_EXTENSIBLE_RECIPE> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_EXTENSIBLE_RECIPE) -> &mut extensible_recipe {
        extensible_recipe::ref_cast_mut(&mut self[MEMORY_WORD_QQQQ])
    }
}
