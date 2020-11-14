//! ` `
// @d mode==cur_list.mode_field {current mode}
/// current mode
macro_rules! mode {
    ($globals:expr) => {
        $globals.cur_list.mode_field
    };
}
// @d head==cur_list.head_field {header node of current list}
/// header node of current list
macro_rules! head {
    ($globals:expr) => {
        $globals.cur_list.head_field
    };
}
// @d tail==cur_list.tail_field {final node on current list}
/// final node on current list
macro_rules! tail {
    ($globals:expr) => {
        $globals.cur_list.tail_field
    };
}
// @d prev_graf==cur_list.pg_field {number of paragraph lines accumulated}
// @d aux==cur_list.aux_field {auxiliary data about the current list}
// @d prev_depth==aux.sc {the name of |aux| in vertical mode}
// @d space_factor==aux.hh.lh {part of |aux| in horizontal mode}
// @d clang==aux.hh.rh {the other part of |aux| in horizontal mode}
// @d incompleat_noad==aux.int {the name of |aux| in math mode}
// @d mode_line==cur_list.ml_field {source file line number at beginning of list}
//
// @<Glob...@>=
// @!nest:array[0..nest_size] of list_state_record;
#[globals_struct_field(TeXGlobals)]
pub(crate) static nest: nest_array<list_state_record> = nest_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0213::nest_array;

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) nest_array[u8_from_0_to_n<nest_size_TYPENUM>] => u8; U8; nest_size_TYPENUM
);

// @!nest_ptr:0..nest_size; {first unused location of |nest|}
// @!max_nest_stack:0..nest_size; {maximum of |nest_ptr| when pushing}
// @!cur_list:list_state_record; {the ``top'' semantic state}
/// the "top" semantic state
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_list: list_state_record = list_state_record::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0212::list_state_record;
// @!shown_mode:-mmode..mmode; {most recent mode shown by \.{\\tracingcommands}}
/// most recent mode shown by `\tracingcommands`
#[globals_struct_field(TeXGlobals)]
pub(crate) static shown_mode: i16_from_m_to_n<mmode_NEG_TYPENUM, mmode_POS_TYPENUM> =
    i16_from_m_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::pascal::i16_from_m_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0211::mmode_NEG_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::section_0211::mmode_POS_TYPENUM;

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::nest_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
