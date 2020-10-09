// @ @<Glob...@>=
// @!input_stack : array[0..stack_size] of in_state_record;
// @!input_ptr : 0..stack_size; {first unused location of |input_stack|}
// @!max_in_stack: 0..stack_size; {largest value of |input_ptr| when pushing}
// @!cur_input : in_state_record;
//   {the ``top'' input state, according to convention (1)}

#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_input: in_state_record = in_state_record::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0300::in_state_record;

use globals_struct::{globals_struct_field, globals_struct_use};
