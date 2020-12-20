//! @ The last two regions of |eqtb| have fullword values instead of the
//! three fields |eq_level|, |eq_type|, and |equiv|. An |eq_type| is unnecessary,
//! but \TeX\ needs to store the |eq_level| information in another array
//! called |xeq_level|.
//
// @<Glob...@>=
// @!eqtb:array[active_base..eqtb_size] of memory_word;
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView)]
pub(crate) static eqtb: Box<eqtb_range_array<memory_word>> = Box::new(eqtb_range_array::default());

#[globals_struct_use(TeXGlobals)]
use crate::section_0253::eqtb_range_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::memory_word;

type eqtb_range_array_LENGTH_TYPENUM = typenum::op!(eqtb_size_TYPENUM - active_base_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) eqtb_range_array[u16_from_m_to_n<active_base_TYPENUM, eqtb_size_TYPENUM>] =>
    u16; U16; active_base_TYPENUM; eqtb_range_array_LENGTH_TYPENUM
);

// @!xeq_level:array[int_base..eqtb_size] of quarterword;
#[globals_struct_field(TeXGlobals)]
pub(crate) static xeq_level: xeq_level_range_array<quarterword> = xeq_level_range_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

#[globals_struct_use(TeXGlobals)]
use crate::section_0253::xeq_level_range_array;

type xeq_level_range_array_LENGTH_TYPENUM = typenum::op!(eqtb_size_TYPENUM - int_base_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) xeq_level_range_array[u16_from_m_to_n<int_base_TYPENUM, eqtb_size_TYPENUM>] =>
    u16; U16; int_base_TYPENUM; xeq_level_range_array_LENGTH_TYPENUM
);

use crate::pascal::u16_from_m_to_n;
use crate::section_0222::active_base_TYPENUM;
use crate::section_0230::int_base_TYPENUM;
use crate::section_0247::eqtb_size_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
