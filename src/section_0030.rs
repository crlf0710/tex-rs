//! @ Binary input and output are done with \PASCAL's ordinary |get| and |put|
//! procedures, so we don't have to make any other special arrangements for
//! binary~I/O. Text output is also easy to do with standard \PASCAL\ routines.
//! The treatment of text input is more difficult, however, because
//! of the necessary translation to |ASCII_code| values.
//! \TeX's conventions should be efficient, and they should
//! blend nicely with the user's operating environment.
//!
//! @ Input from text files is read one line at a time, using a routine called
//! |input_ln|. This function is defined in terms of global variables called
//! |buffer|, |first|, and |last| that will be described in detail later; for
//! now, it suffices for us to know that |buffer| is an array of |ASCII_code|
//! values, and that |first| and |last| are indices into this array
//! representing the beginning and ending of a line of text.
//
// @<Glob...@>=
// @!buffer:array[0..buf_size] of ASCII_code; {lines of characters being read}
// @!first:0..buf_size; {the first unused position in |buffer|}
// @!last:0..buf_size; {end of the line just input to |buffer|}
// @!max_buf_stack:0..buf_size; {largest index used in |buffer|}
//

#[globals_struct_field(TeXGlobals)]
/// lines of characters being read
pub(crate) static buffer: buf_size_array<ASCII_code> = buf_size_array::default();

#[globals_struct_field(TeXGlobals)]
/// the first unused position in `buffer`
pub(crate) static first: u16_from_0_to_n<buf_size_TYPENUM> = u16_from_0_to_n::new(0);

#[globals_struct_field(TeXGlobals)]
/// end of the line just input to `buffer`
pub(crate) static last: u16_from_0_to_n<buf_size_TYPENUM> = u16_from_0_to_n::new(0);

#[globals_struct_field(TeXGlobals)]
/// largest index used in `buffer`
pub(crate) static max_buf_stack: u16_from_0_to_n<buf_size_TYPENUM> = u16_from_0_to_n::new(0);

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u16_from_0_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0018::ASCII_code;

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::buf_size;

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::buf_size_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::section_0030::buf_size_array;

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) buf_size_array[u16_from_0_to_n<buf_size_TYPENUM>] => u16; U16; buf_size_TYPENUM
);

use crate::pascal::u16_from_0_to_n;
use crate::section_0011::buf_size_TYPENUM;

use globals_struct::{globals_struct_field, globals_struct_use};
