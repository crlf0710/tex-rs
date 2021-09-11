//! @ The |param_stack| is an auxiliary array used to hold pointers to the token
//! lists for parameters at the current level and subsidiary levels of input.
//! This stack is maintained with convention (2), and it grows at a different
//! rate from the others.

//
// @<Glob...@>=

// @!param_stack:array [0..param_size] of pointer;
//   {token list pointers for parameters}
/// token list pointers for parameters
#[globals_struct_field(TeXGlobals)]
pub(crate) static param_stack: param_stack_array<pointer> = param_stack_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0308::param_stack_array;

type param_stack_array_LENGTH_TYPENUM = typenum::op!(param_size_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) param_stack_array[u8_from_0_to_n<param_size_TYPENUM>] => u8; U8; param_stack_array_LENGTH_TYPENUM
);

// @!param_ptr:0..param_size; {first unused entry in |param_stack|}
/// first unused entry in `param_stack`
#[globals_struct_field(TeXGlobals)]
pub(crate) static param_ptr: u8_from_0_to_n<param_size_TYPENUM> = u8_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::param_size_TYPENUM;

// @!max_param_stack:integer;
//   {largest value of |param_ptr|, will be |<=param_size+9|}
/// largest value of `param_ptr`, will be `<=param_size+9`
#[globals_struct_field(TeXGlobals)]
pub(crate) static max_param_stack: integer = integer::default();

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::param_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
