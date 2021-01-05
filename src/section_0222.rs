//! @ Many locations in |eqtb| have symbolic names. The purpose of the next
//! paragraphs is to define these names, and to set up the initial values of the
//! equivalents.
//!
//! In the first region we have 256 equivalents for ``active characters'' that
//! act as control sequences, followed by 256 equivalents for single-character
//! control sequences.
//!
//! Then comes region~2, which corresponds to the hash table that we will
//! define later.  The maximum address in this region is used for a dummy
//! control sequence that is perpetually undefined. There also are several
//! locations for control sequences that are perpetually defined
//! (since they are used in error recovery).

// @d active_base=1 {beginning of region 1, for active character equivalents}
/// beginning of region 1, for active character equivalents
pub(crate) type active_base_TYPENUM = U1;
pub(crate) const active_base: word = active_base_TYPENUM::U32;
// @d single_base=active_base+256 {equivalents of one-character control sequences}
/// equivalents of one-character control sequences
pub(crate) type single_base_TYPENUM = typenum::op!(active_base_TYPENUM + U256);
pub(crate) const single_base: word = single_base_TYPENUM::U32;
// @d null_cs=single_base+256 {equivalent of \.{\\csname\\endcsname}}
/// equivalent of `\csname\endcsname`
pub(crate) type null_cs_TYPENUM = typenum::op!(single_base_TYPENUM + U256);
pub(crate) const null_cs: halfword = null_cs_TYPENUM::U16;
// @d hash_base=null_cs+1 {beginning of region 2, for the hash table}
/// beginning of region 2, for the hash table
pub(crate) type hash_base_TYPENUM = typenum::op!(null_cs_TYPENUM + U1);
pub(crate) const hash_base: word = hash_base_TYPENUM::U32;
// @d frozen_control_sequence=hash_base+hash_size {for error recovery}
/// for error recovery
pub(crate) type frozen_control_sequence_TYPENUM =
    typenum::op!(hash_base_TYPENUM + hash_size_TYPENUM);
pub(crate) const frozen_control_sequence: word = frozen_control_sequence_TYPENUM::U32;
// @d frozen_protection=frozen_control_sequence {inaccessible but definable}
// @d frozen_cr=frozen_control_sequence+1 {permanent `\.{\\cr}'}
/// permanent `\cr`
pub(crate) const frozen_cr: word = frozen_control_sequence + 1;
// @d frozen_end_group=frozen_control_sequence+2 {permanent `\.{\\endgroup}'}
/// permanent `\endgroup`
pub(crate) const frozen_end_group: word = frozen_control_sequence + 2;
// @d frozen_right=frozen_control_sequence+3 {permanent `\.{\\right}'}
// @d frozen_fi=frozen_control_sequence+4 {permanent `\.{\\fi}'}
/// permanent `\fi`
pub(crate) const frozen_fi: word = frozen_control_sequence + 4;
// @d frozen_end_template=frozen_control_sequence+5 {permanent `\.{\\endtemplate}'}
// @d frozen_endv=frozen_control_sequence+6 {second permanent `\.{\\endtemplate}'}
/// second permanent `\endtemplate`
pub(crate) const frozen_endv: word = frozen_control_sequence + 6;
// @d frozen_relax=frozen_control_sequence+7 {permanent `\.{\\relax}'}
/// permanent `\relax`
pub(crate) const frozen_relax: word = frozen_control_sequence + 7;
// @d end_write=frozen_control_sequence+8 {permanent `\.{\\endwrite}'}
/// permanent `\endwrite'
pub(crate) const end_write: word = frozen_control_sequence + 8;
// @d frozen_dont_expand=frozen_control_sequence+9
//   {permanent `\.{\\notexpanded:}'}
/// permanent `\notexpanded:`
pub(crate) const frozen_dont_expand: word = frozen_control_sequence + 9;
// @d frozen_null_font=frozen_control_sequence+10
//   {permanent `\.{\\nullfont}'}
/// permanent `\nullfont`
pub(crate) type frozen_null_font_TYPENUM = typenum::op!(frozen_control_sequence_TYPENUM + U10);
pub(crate) const frozen_null_font: word = frozen_null_font_TYPENUM::U32;
// @d font_id_base=frozen_null_font-font_base
//   {begins table of 257 permanent font identifiers}
/// begins table of 257 permanent font identifiers
pub(crate) const font_id_base: word = frozen_null_font - font_base as word;
// @d undefined_control_sequence=frozen_null_font+257 {dummy location}
/// dummy location
pub(crate) type undefined_control_sequence_TYPENUM = typenum::op!(frozen_null_font_TYPENUM + U257);
pub(crate) const undefined_control_sequence: pointer = undefined_control_sequence_TYPENUM::U16;
// @d glue_base=undefined_control_sequence+1 {beginning of region 3}
/// beginning of region 3
pub(crate) type glue_base_TYPENUM = typenum::op!(undefined_control_sequence_TYPENUM + U1);
pub(crate) const glue_base: word = glue_base_TYPENUM::U32;

// @<Initialize table entries...@>=
#[distributed_slice(INIT_TBLENTRY)]
#[allow(unused_variables)]
pub(crate) fn initialize_table_entries_done_by_initex_only_0222(globals: &mut TeXGlobals) {
    // eq_type(undefined_control_sequence):=undefined_cs;
    eq_type!(globals, undefined_control_sequence) = undefined_cs;
    // equiv(undefined_control_sequence):=null;
    equiv!(globals, undefined_control_sequence) = null;
    // eq_level(undefined_control_sequence):=level_zero;
    eq_level!(globals, undefined_control_sequence) = level_zero;
    // for k:=active_base to undefined_control_sequence-1 do
    for k in active_base as u16..undefined_control_sequence {
        // eqtb[k]:=eqtb[undefined_control_sequence];
        globals.eqtb[k as u16] = globals.eqtb[undefined_control_sequence];
    }
}

use crate::pascal::word;
use crate::section_0012::hash_size_TYPENUM;
use crate::section_0012::font_base;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use crate::section_0115::null;
use crate::section_0210::undefined_cs;
use crate::section_0221::level_zero;
use typenum::Unsigned;
use typenum::{U1, U10, U256, U257};

use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
