//! @ Fixed-point arithmetic is done on {\sl scaled integers\/} that are multiples
//! of $2^{-16}$. In other words, a binary point is assumed to be sixteen bit
//! positions from the right end of a binary computer word.
//
// @d unity == @'200000 {$2^{16}$, represents 1.00000}
/// `2^{16}`, represents 1.00000
pub(crate) const unity: scaled = scaled(0o200000);
// @d two == @'400000 {$2^{17}$, represents 2.00000}
pub(crate) const two: scaled = scaled(0o400000);
//
// @<Types...@>=
// @!scaled = integer; {this type is used for scaled integers}
// @!nonnegative_integer=0..@'17777777777; {$0\L x<2^{31}$}
// @!small_number=0..63; {this type is self-explanatory}
//
#[derive(Copy, Clone, RefCast, PartialEq, PartialOrd, Debug)]
#[repr(transparent)]
pub struct scaled(integer);

impl scaled {
    pub(crate) const fn zero() -> Self {
        scaled(0)
    }

    pub(crate) const fn new_from_inner(v: integer) -> Self {
        scaled(v)
    }

    pub(crate) const fn inner(&self) -> integer {
        self.0
    }
}

impl Default for scaled {
    fn default() -> Self {
        scaled::zero()
    }
}

impl core::ops::Add for scaled {
    type Output = scaled;
    fn add(self, rhs: scaled) -> scaled {
        scaled(self.0 + rhs.0)
    }
}

impl core::ops::AddAssign for scaled {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl core::ops::Sub for scaled {
    type Output = scaled;
    fn sub(self, rhs: scaled) -> scaled {
        scaled(self.0 - rhs.0)
    }
}

impl core::ops::SubAssign for scaled {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl core::ops::Neg for scaled {
    type Output = scaled;
    fn neg(self) -> Self::Output {
        scaled(-self.inner())
    }
}


pub type nonnegative_integer = i32_from_m_to_n<Z0, ::typenum::op!(P2147483648 - P1)>;
pub type small_number = u8_from_m_to_n<U0, U63>;

use crate::pascal::{i32_from_m_to_n, integer, u32_from_m_to_n, u8_from_m_to_n};
use crate::section_0113::memory_word;
use crate::section_0113::MEMORY_WORD_INT;
use core::ops::{Index, IndexMut};
use ref_cast::RefCast;
use typenum::{P1, P2147483648, Z0, U0, U1, U63};

pub(crate) struct MEMORY_WORD_SC;

impl Index<MEMORY_WORD_SC> for memory_word {
    type Output = scaled;
    fn index(&self, _: MEMORY_WORD_SC) -> &scaled {
        scaled::ref_cast(&self[MEMORY_WORD_INT])
    }
}

impl IndexMut<MEMORY_WORD_SC> for memory_word {
    fn index_mut(&mut self, _: MEMORY_WORD_SC) -> &mut scaled {
        scaled::ref_cast_mut(&mut self[MEMORY_WORD_INT])
    }
}
