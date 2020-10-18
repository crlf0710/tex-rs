//! @ The reader should study the following definitions closely:
//! @^system dependencies@>

// @d sc==int {|scaled| data is equivalent to |integer|}

/// We add `sc` directly as another variant in memory_word here.
const _: () = ();

//
// @<Types...@>=
// @!quarterword = min_quarterword..max_quarterword; {1/4 of a word}

/// 1/4 of a word
pub type quarterword = u8;
const_assert!(min_quarterword == quarterword::MIN);
const_assert!(max_quarterword == quarterword::MAX);

// @!halfword=min_halfword..max_halfword; {1/2 of a word}
pub type halfword = u16;
const_assert!(min_halfword == halfword::MIN);
const_assert!(max_halfword == halfword::MAX);

// @!two_choices = 1..2; {used when there are two variants in a record}
// @!four_choices = 1..4; {used when there are four variants in a record}

/// These are not needed in Rust.
#[doc(hidden)]
const _: () = ();

// @!two_halves = packed record@;@/
//   @!rh:halfword;
//   case two_choices of
//   1: (@!lh:halfword);
//   2: (@!b0:quarterword; @!b1:quarterword);
//   end;

#[derive(Copy, Clone)]
struct two_halves {
    rh: halfword,
    lh_or_b01: halfword_or_b01,
}

#[derive(Copy, Clone)]
union halfword_or_b01 {
    lh: halfword,
    b: (quarterword, quarterword),
}

// @!four_quarters = packed record@;@/
//   @!b0:quarterword;
//   @!b1:quarterword;
//   @!b2:quarterword;
//   @!b3:quarterword;
//   end;

#[derive(Copy, Clone)]
struct four_quarters {
    b: (quarterword, quarterword, quarterword, quarterword),
}

// @!memory_word = record@;@/
//   case four_choices of
//   1: (@!int:integer);
//   2: (@!gr:glue_ratio);
//   3: (@!hh:two_halves);
//   4: (@!qqqq:four_quarters);
//   end;

#[derive(Copy, Clone)]
pub(crate) union memory_word {
    int: integer,
    sc: scaled,
    gr: glue_ratio,
    w: word,
    hh: two_halves,
    qqqq: four_quarters,
}

// @!word_file = file of memory_word;
type word_file = file_of<memory_word>;

use crate::pascal::{file_of, integer, word};
use crate::section_0101::scaled;
use crate::section_0109::glue_ratio;
use crate::section_0110::{max_halfword, max_quarterword, min_halfword, min_quarterword};
use static_assertions::const_assert;

impl Default for memory_word {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub(crate) struct MEMORY_WORD_INT;

impl Index<MEMORY_WORD_INT> for memory_word {
    type Output = integer;
    fn index(&self, _: MEMORY_WORD_INT) -> &integer {
        unsafe { &self.int }
    }
}

impl IndexMut<MEMORY_WORD_INT> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_INT) -> &mut integer {
        unsafe { &mut self.int }
    }
}

pub(crate) struct MEMORY_WORD_HH_RH;

impl Index<MEMORY_WORD_HH_RH> for memory_word {
    type Output = halfword;
    fn index(&self, _: MEMORY_WORD_HH_RH) -> &halfword {
        unsafe { &self.hh.rh }
    }
}

impl IndexMut<MEMORY_WORD_HH_RH> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_HH_RH) -> &mut halfword {
        unsafe { &mut self.hh.rh }
    }
}

pub(crate) struct MEMORY_WORD_HH_B1;

impl Index<MEMORY_WORD_HH_B1> for memory_word {
    type Output = quarterword;
    fn index(&self, _: MEMORY_WORD_HH_B1) -> &quarterword {
        unsafe { &self.hh.lh_or_b01.b.1 }
    }
}

impl IndexMut<MEMORY_WORD_HH_B1> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_HH_B1) -> &mut quarterword {
        unsafe { &mut self.hh.lh_or_b01.b.1 }
    }
}

pub(crate) struct MEMORY_WORD_HH_B0;

impl Index<MEMORY_WORD_HH_B0> for memory_word {
    type Output = quarterword;
    fn index(&self, _: MEMORY_WORD_HH_B0) -> &quarterword {
        unsafe { &self.hh.lh_or_b01.b.0 }
    }
}

impl IndexMut<MEMORY_WORD_HH_B0> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_HH_B0) -> &mut quarterword {
        unsafe { &mut self.hh.lh_or_b01.b.0 }
    }
}

use core::ops::{Index, IndexMut};
