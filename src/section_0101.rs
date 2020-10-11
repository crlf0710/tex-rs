//! @ Fixed-point arithmetic is done on {\sl scaled integers\/} that are multiples
//! of $2^{-16}$. In other words, a binary point is assumed to be sixteen bit
//! positions from the right end of a binary computer word.
//
// @d unity == @'200000 {$2^{16}$, represents 1.00000}
/// `2^{16}`, represents 1.00000
const unity: scaled = scaled(0o200000);
// @d two == @'400000 {$2^{17}$, represents 2.00000}
const two: scaled = scaled(0o400000);
//
// @<Types...@>=
// @!scaled = integer; {this type is used for scaled integers}
// @!nonnegative_integer=0..@'17777777777; {$0\L x<2^{31}$}
// @!small_number=0..63; {this type is self-explanatory}
//
#[derive(Copy, Clone)]
pub struct scaled(integer);

pub type nonnegative_integer = u32_from_m_to_n<U0, ::typenum::op!(U2147483648 - U1)>;
pub type small_number = u8_from_m_to_n<U0, U63>;

use crate::pascal::{integer, u32_from_m_to_n, u8_from_m_to_n};
use typenum::{U0, U1, U2147483648, U63};
