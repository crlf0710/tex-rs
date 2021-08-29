// @ @<Glob...@>=

// @!input_stack : array[0..stack_size] of in_state_record;
#[globals_struct_field(TeXGlobals)]
pub(crate) static input_stack: input_stack_array<in_state_record> = input_stack_array::default();

type input_stack_array_LENGTH_TYPENUM = typenum::op!(stack_size_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) input_stack_array[u16_from_0_to_n<stack_size_TYPENUM>] => u16; U16; input_stack_array_LENGTH_TYPENUM
);

// @!input_ptr : 0..stack_size; {first unused location of |input_stack|}
/// first unused location of `input_stack`
#[globals_struct_field(TeXGlobals)]
pub(crate) static input_ptr: u16_from_0_to_n<stack_size_TYPENUM> = u16_from_0_to_n::new(0);

// @!max_in_stack: 0..stack_size; {largest value of |input_ptr| when pushing}
#[globals_struct_field(TeXGlobals)]
pub(crate) static max_in_stack: u16_from_0_to_n<stack_size_TYPENUM> = u16_from_0_to_n::new(0);

// @!cur_input : in_state_record;
//   {the ``top'' input state, according to convention (1)}
/// the "top" input state, according to convention (1)
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_input: in_state_record = in_state_record::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::stack_size_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::section_0300::in_state_record;

#[globals_struct_use(TeXGlobals)]
use crate::section_0301::input_stack_array;

use crate::pascal::u16_from_0_to_n;
use crate::section_0011::stack_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
