//! @ The sixteen possible \.{\\write} streams are represented by the |write_file|
//! array. The |j|th file is open if and only if |write_open[j]=true|. The last
//! two streams are special; |write_open[16]| represents a stream number
//! greater than 15, while |write_open[17]| represents a negative stream number,
//! and both of these variables are always |false|.

//
// @<Glob...@>=
// @!write_file:array[0..15] of alpha_file;
// @!write_open:array[0..17] of boolean;

#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView)]
pub(crate) static write_file: write_file_array<alpha_file> = write_file_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_1342::write_file_array;

#[globals_struct_field(TeXGlobals)]
pub(crate) static write_open: write_open_array<boolean> = write_open_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_1342::write_open_array;

type write_file_array_LENGTH_TYPENUM = typenum::op!(U15 + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) write_file_array[u8_from_0_to_n<U15>] => u8; U8; write_file_array_LENGTH_TYPENUM
);

type write_open_array_LENGTH_TYPENUM = typenum::op!(U17 + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) write_open_array[u8_from_0_to_n<U17>] => u8; U8; write_open_array_LENGTH_TYPENUM
);

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::{U1, U15, U17};
use typenum::Unsigned;