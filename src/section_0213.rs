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
/// number of paragraph lines accumulated
macro_rules! prev_graf {
    ($globals:expr) => {
        $globals.cur_list.pg_field
    };
}

// @d aux==cur_list.aux_field {auxiliary data about the current list}
/// auxiliary data about the current list
macro_rules! aux {
    ($globals:expr) => {
        $globals.cur_list.aux_field
    }
}
// @d prev_depth==aux.sc {the name of |aux| in vertical mode}
/// the name of `aux` in vertical mode
macro_rules! prev_depth {
    ($globals:expr) => {
        aux!($globals)[crate::section_0101::MEMORY_WORD_SC]
    }
}
// @d space_factor==aux.hh.lh {part of |aux| in horizontal mode}
/// part of `aux` in horizontal mode
macro_rules! space_factor {
    ($globals:expr) => {
        aux!($globals)[crate::section_0113::MEMORY_WORD_HH_LH]
    }
}
// @d clang==aux.hh.rh {the other part of |aux| in horizontal mode}
// @d incompleat_noad==aux.int {the name of |aux| in math mode}
// @d mode_line==cur_list.ml_field {source file line number at beginning of list}
/// source file line number at beginning of list
macro_rules! mode_line {
    ($globals:expr) => {
        $globals.cur_list.ml_field
    }
}

// @<Glob...@>=
// @!nest:array[0..nest_size] of list_state_record;
#[globals_struct_field(TeXGlobals)]
pub(crate) static nest: nest_array<list_state_record> = nest_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0213::nest_array;

type nest_array_LENGTH_TYPENUM = typenum::op!(nest_size_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) nest_array[u8_from_0_to_n<nest_size_TYPENUM>] => u8; U8; nest_array_LENGTH_TYPENUM
);

// @!nest_ptr:0..nest_size; {first unused location of |nest|}
/// first unused location of `nest`
#[globals_struct_field(TeXGlobals)]
pub(crate) static nest_ptr: u8_from_0_to_n<nest_size_TYPENUM> = u8_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::nest_size_TYPENUM;

// @!max_nest_stack:0..nest_size; {maximum of |nest_ptr| when pushing}
/// maximum of `nest_ptr` when pushing
#[globals_struct_field(TeXGlobals)]
pub(crate) static max_nest_stack: u8_from_0_to_n<nest_size_TYPENUM> = u8_from_0_to_n::default();

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
use typenum::U1;
use globals_struct::{globals_struct_field, globals_struct_use};
