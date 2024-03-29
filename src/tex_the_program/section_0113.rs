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
pub(crate) struct two_halves {
    rh: halfword,
    lh_or_b01: halfword_or_b01,
}

impl two_halves {
    pub(crate) const fn new_with_halves(lh: halfword, rh: halfword) -> Self {
        two_halves {
            lh_or_b01: halfword_or_b01 { lh },
            rh,
        }
    }
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

#[derive(Copy, Clone, Debug)]
pub(crate) struct four_quarters {
    b: (quarterword, quarterword, quarterword, quarterword),
}

impl four_quarters {
    pub(crate) const fn new_with_quarters(
        b0: quarterword,
        b1: quarterword,
        b2: quarterword,
        b3: quarterword,
    ) -> Self {
        four_quarters {
            b: (b0, b1, b2, b3),
        }
    }
}

impl Default for four_quarters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
    gr: glue_ratio,
    w: word,
    hh: two_halves,
    qqqq: four_quarters,
}

// @!word_file = file of memory_word;
pub(crate) type word_file = file_of<memory_word>;

use crate::io_support::file_of;
use crate::pascal::{integer, word};
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
pub(crate) struct MEMORY_WORD_GR;

impl Index<MEMORY_WORD_GR> for memory_word {
    type Output = glue_ratio;
    fn index(&self, _: MEMORY_WORD_GR) -> &glue_ratio {
        unsafe { &self.gr }
    }
}

impl IndexMut<MEMORY_WORD_GR> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_GR) -> &mut glue_ratio {
        unsafe { &mut self.gr }
    }
}

pub(crate) struct MEMORY_WORD_HH;

impl Index<MEMORY_WORD_HH> for memory_word {
    type Output = two_halves;
    fn index(&self, _: MEMORY_WORD_HH) -> &two_halves {
        unsafe { &self.hh }
    }
}

impl IndexMut<MEMORY_WORD_HH> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_HH) -> &mut two_halves {
        unsafe { &mut self.hh }
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

pub(crate) struct MEMORY_WORD_HH_LH;

impl Index<MEMORY_WORD_HH_LH> for memory_word {
    type Output = halfword;
    fn index(&self, _: MEMORY_WORD_HH_LH) -> &halfword {
        unsafe { &self.hh.lh_or_b01.lh }
    }
}

impl IndexMut<MEMORY_WORD_HH_LH> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_HH_LH) -> &mut halfword {
        unsafe { &mut self.hh.lh_or_b01.lh }
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

pub(crate) struct MEMORY_WORD_QQQQ_B2;

impl Index<MEMORY_WORD_QQQQ_B2> for memory_word {
    type Output = quarterword;
    fn index(&self, _: MEMORY_WORD_QQQQ_B2) -> &quarterword {
        unsafe { &self.qqqq.b.2 }
    }
}

impl IndexMut<MEMORY_WORD_QQQQ_B2> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_QQQQ_B2) -> &mut quarterword {
        unsafe { &mut self.qqqq.b.2 }
    }
}

pub(crate) struct MEMORY_WORD_QQQQ_B3;

impl Index<MEMORY_WORD_QQQQ_B3> for memory_word {
    type Output = quarterword;
    fn index(&self, _: MEMORY_WORD_QQQQ_B3) -> &quarterword {
        unsafe { &self.qqqq.b.3 }
    }
}

impl IndexMut<MEMORY_WORD_QQQQ_B3> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_QQQQ_B3) -> &mut quarterword {
        unsafe { &mut self.qqqq.b.3 }
    }
}

pub(crate) struct MEMORY_WORD_QQQQ;

impl Index<MEMORY_WORD_QQQQ> for memory_word {
    type Output = four_quarters;
    fn index(&self, _: MEMORY_WORD_QQQQ) -> &four_quarters {
        unsafe { &self.qqqq }
    }
}

impl IndexMut<MEMORY_WORD_QQQQ> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_QQQQ) -> &mut four_quarters {
        unsafe { &mut self.qqqq }
    }
}

impl Default for two_halves {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub(crate) struct TWO_HALVES_RH;

impl Index<TWO_HALVES_RH> for two_halves {
    type Output = halfword;
    fn index(&self, _: TWO_HALVES_RH) -> &halfword {
        &self.rh
    }
}

impl IndexMut<TWO_HALVES_RH> for two_halves {
    fn index_mut(&mut self, _: TWO_HALVES_RH) -> &mut halfword {
        &mut self.rh
    }
}

pub(crate) struct TWO_HALVES_LH;

impl Index<TWO_HALVES_LH> for two_halves {
    type Output = halfword;
    fn index(&self, _: TWO_HALVES_LH) -> &halfword {
        unsafe { &self.lh_or_b01.lh }
    }
}

impl IndexMut<TWO_HALVES_LH> for two_halves {
    fn index_mut(&mut self, _: TWO_HALVES_LH) -> &mut halfword {
        unsafe { &mut self.lh_or_b01.lh }
    }
}
pub(crate) struct FOUR_QUARTERS_B0;

impl Index<FOUR_QUARTERS_B0> for four_quarters {
    type Output = quarterword;
    fn index(&self, _: FOUR_QUARTERS_B0) -> &quarterword {
        &self.b.0
    }
}

impl IndexMut<FOUR_QUARTERS_B0> for four_quarters {
    fn index_mut(&mut self, _: FOUR_QUARTERS_B0) -> &mut quarterword {
        &mut self.b.0
    }
}

pub(crate) struct FOUR_QUARTERS_B1;

impl Index<FOUR_QUARTERS_B1> for four_quarters {
    type Output = quarterword;
    fn index(&self, _: FOUR_QUARTERS_B1) -> &quarterword {
        &self.b.1
    }
}

impl IndexMut<FOUR_QUARTERS_B1> for four_quarters {
    fn index_mut(&mut self, _: FOUR_QUARTERS_B1) -> &mut quarterword {
        &mut self.b.1
    }
}

pub(crate) struct FOUR_QUARTERS_B2;

impl Index<FOUR_QUARTERS_B2> for four_quarters {
    type Output = quarterword;
    fn index(&self, _: FOUR_QUARTERS_B2) -> &quarterword {
        &self.b.2
    }
}

impl IndexMut<FOUR_QUARTERS_B2> for four_quarters {
    fn index_mut(&mut self, _: FOUR_QUARTERS_B2) -> &mut quarterword {
        &mut self.b.2
    }
}

pub(crate) struct FOUR_QUARTERS_B3;

impl Index<FOUR_QUARTERS_B3> for four_quarters {
    type Output = quarterword;
    fn index(&self, _: FOUR_QUARTERS_B3) -> &quarterword {
        &self.b.3
    }
}

impl IndexMut<FOUR_QUARTERS_B3> for four_quarters {
    fn index_mut(&mut self, _: FOUR_QUARTERS_B3) -> &mut quarterword {
        &mut self.b.3
    }
}

impl FromBlob for memory_word {
    fn from_blob(data: &[u8]) -> Self {
        use core::mem::transmute;
        assert!(data.len() == 4);
        memory_word {
            int: i32::from_le_bytes([data[0], data[1], data[2], data[3]]),
        }
    }
}

impl ToBlob for memory_word {
    type BlobType = [u8; 4];

    fn to_blob(&self) -> Self::BlobType {
        self[MEMORY_WORD_INT].to_le_bytes()
    }
}

use crate::io_support::FromBlob;
use crate::io_support::ToBlob;
use core::ops::{Index, IndexMut};
