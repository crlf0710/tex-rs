//! ` `

// @<Glob...@>=
// @!save_stack : array[0..save_size] of memory_word;
#[globals_struct_field(TeXGlobals)]
pub(crate) static save_stack: save_stack_array<memory_word> = save_stack_array::default();

type save_stack_array_LENGTH_TYPENUM = typenum::op!(save_size_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) save_stack_array[u16_from_0_to_n<save_size_TYPENUM>] => u16; U16; save_stack_array_LENGTH_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0271::save_stack_array;

// @!save_ptr : 0..save_size; {first unused entry on |save_stack|}
/// first unused entry on `save_stack`
#[globals_struct_field(TeXGlobals)]
pub(crate) static save_ptr: u16_from_0_to_n<save_size_TYPENUM> = u16_from_0_to_n::default();
// @!max_save_stack:0..save_size; {maximum usage of save stack}
/// maximum usage of save stack
#[globals_struct_field(TeXGlobals)]
pub(crate) static max_save_stack: u16_from_0_to_n<save_size_TYPENUM> = u16_from_0_to_n::default();
// @!cur_level: quarterword; {current nesting level for groups}
/// current nesting level for groups
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_level: quarterword = 0;
// @!cur_group: group_code; {current group type}
/// current group type
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_group: group_code = group_code::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0269::group_code;

// @!cur_boundary: 0..save_size; {where the current level begins}
/// where the current level begins
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_boundary: u16_from_0_to_n<save_size_TYPENUM> = u16_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::save_size_TYPENUM;


use crate::section_0004::TeXGlobals;
use crate::section_0011::save_size_TYPENUM;
use crate::pascal::u16_from_0_to_n;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
