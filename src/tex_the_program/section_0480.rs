//! @ Another way to create a token list is via the \.{\\read} command. The
//! sixteen files potentially usable for reading appear in the following
//! global variables. The value of |read_open[n]| will be |closed| if
//! stream number |n| has not been opened or if it has been fully read;
//! |just_open| if an \.{\\openin} but not a \.{\\read} has been done;
//! and |normal| if it is open and ready to read the next line.
//
// @d closed=2 {not open, or at end of file}
// @d just_open=1 {newly opened, first line not yet read}
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum read_open_kind {
    normal = 0,
    /// newly opened, first line not yet read
    just_open = 1,
    /// not open, or at end of file
    closed = 2,
}

impl Default for read_open_kind {
    fn default() -> Self {
        read_open_kind::normal
    }
}

// @<Glob...@>=
// @!read_file:array[0..15] of alpha_file; {used for \.{\\read}}
/// used for `\read`
#[globals_struct_field(TeXGlobals)]
pub(crate) static read_file: read_file_array<alpha_file> = read_file_array::default();
// @!read_open:array[0..16] of normal..closed; {state of |read_file[n]|}
/// state of `read_file[n]`
#[globals_struct_field(TeXGlobals)]
pub(crate) static read_open: read_open_array<read_open_kind> = read_open_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0480::read_file_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0480::read_open_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0480::read_open_kind;

type read_file_array_LENGTH_TYPENUM = typenum::op!(U15 + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) read_file_array[u8_from_0_to_n<U15>] => u8; U8; read_file_array_LENGTH_TYPENUM
);

type read_open_array_LENGTH_TYPENUM = typenum::op!(U16 + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) read_open_array[u8_from_0_to_n<U16>] => u8; U8; read_open_array_LENGTH_TYPENUM
);

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::Unsigned;
use typenum::{U1, U15, U16};
