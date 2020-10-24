//! @ Each entry in |eqtb| is a |memory_word|. Most of these words are of type
//! |two_halves|, and subdivided into three fields:
//!
//! \yskip\hangg 1) The |eq_level| (a quarterword) is the level of grouping at
//! which this equivalent was defined. If the level is |level_zero|, the
//! equivalent has never been defined; |level_one| refers to the outer level
//! (outside of all groups), and this level is also used for global
//! definitions that never go away. Higher levels are for equivalents that
//! will disappear at the end of their group.  @^global definitions@>
//!
//! \yskip\hangg 2) The |eq_type| (another quarterword) specifies what kind of
//! entry this is. There are many types, since each \TeX\ primitive like
//! \.{\\hbox}, \.{\\def}, etc., has its own special code. The list of
//! command codes above includes all possible settings of the |eq_type| field.
//!
//! \yskip\hangg 3) The |equiv| (a halfword) is the current equivalent value.
//! This may be a font number, a pointer into |mem|, or a variety of other
//! things.
//
// @d eq_level_field(#)==#.hh.b1
macro_rules! eq_level_field {
    ($val:expr) => {
        $val[crate::section_0113::MEMORY_WORD_HH_B1]
    };
}
// @d eq_type_field(#)==#.hh.b0
macro_rules! eq_type_field {
    ($val:expr) => {
        $val[crate::section_0113::MEMORY_WORD_HH_B0]
    };
}
// @d equiv_field(#)==#.hh.rh
macro_rules! equiv_field {
    ($val:expr) => {
        $val[crate::section_0113::MEMORY_WORD_HH_RH]
    };
}
// @d eq_level(#)==eq_level_field(eqtb[#]) {level of definition}
/// level of definition
#[allow(unused_macros)]
macro_rules! eq_level {
    ($globals:expr, $val:expr) => {
        eq_level_field!($globals.eqtb[$val as crate::section_0115::pointer])
    };
}
// @d eq_type(#)==eq_type_field(eqtb[#]) {command code for equivalent}
/// command code for equivalent
#[allow(unused_macros)]
macro_rules! eq_type {
    ($globals:expr, $val:expr) => {
        eq_type_field!($globals.eqtb[$val as crate::section_0115::pointer])
    };
}
// @d equiv(#)==equiv_field(eqtb[#]) {equivalent value}
/// equivalent value
macro_rules! equiv {
    ($globals:expr, $val:expr) => {
        equiv_field!($globals.eqtb[$val as crate::section_0115::pointer])
    };
}
// @d level_zero=min_quarterword {level for undefined quantities}
/// level for undefined quantities
pub(crate) const level_zero: quarterword = min_quarterword;
// @d level_one=level_zero+1 {outermost level for defined quantities}
/// outermost level for defined quantities
pub(crate) const level_one: quarterword = level_zero + 1;

use crate::section_0110::min_quarterword;
use crate::section_0113::quarterword;
