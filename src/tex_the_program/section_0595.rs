//! @ Some systems may find it more efficient to make |dvi_buf| a |packed|
//! array, since output of four bytes at once may be facilitated.
//! @^system dependencies@>
//
// @<Glob...@>=
// @!dvi_buf:array[dvi_index] of eight_bits; {buffer for \.{DVI} output}
/// buffer for `DVI` output
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_buf: dvi_index_array<eight_bits> = dvi_index_array::default();
// @!half_buf:dvi_index; {half of |dvi_buf_size|}
/// half of `dvi_buf_size`
#[globals_struct_field(TeXGlobals)]
pub(crate) static half_buf: dvi_index = dvi_index::default();
// @!dvi_limit:dvi_index; {end of the current half buffer}
/// end of the current half buffer
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_limit: dvi_index = dvi_index::default();
// @!dvi_ptr:dvi_index; {the next available buffer address}
/// the next available buffer address
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_ptr: dvi_index = dvi_index::default();
// @!dvi_offset:integer; {|dvi_buf_size| times the number of times the
//   output buffer has been fully emptied}
/// `dvi_buf_size` times the number of times the output buffer has been fully emptied
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_offset: integer = 0;
// @!dvi_gone:integer; {the number of bytes already output to |dvi_file|}
/// the number of bytes already output to `dvi_file`
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_gone: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::section_0594::dvi_index;

#[globals_struct_use(TeXGlobals)]
use crate::section_0594::dvi_index_array;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

