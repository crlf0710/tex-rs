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
// @d frozen_end_group=frozen_control_sequence+2 {permanent `\.{\\endgroup}'}
// @d frozen_right=frozen_control_sequence+3 {permanent `\.{\\right}'}
// @d frozen_fi=frozen_control_sequence+4 {permanent `\.{\\fi}'}
// @d frozen_end_template=frozen_control_sequence+5 {permanent `\.{\\endtemplate}'}
// @d frozen_endv=frozen_control_sequence+6 {second permanent `\.{\\endtemplate}'}
// @d frozen_relax=frozen_control_sequence+7 {permanent `\.{\\relax}'}
// @d end_write=frozen_control_sequence+8 {permanent `\.{\\endwrite}'}
// @d frozen_dont_expand=frozen_control_sequence+9
//   {permanent `\.{\\notexpanded:}'}
// @d frozen_null_font=frozen_control_sequence+10
//   {permanent `\.{\\nullfont}'}
/// permanent `\nullfont`
pub(crate) type frozen_null_font_TYPENUM = typenum::op!(frozen_control_sequence_TYPENUM + U10);
pub(crate) const frozen_null_font: word = frozen_null_font_TYPENUM::U32;
// @d font_id_base=frozen_null_font-font_base
//   {begins table of 257 permanent font identifiers}
// @d undefined_control_sequence=frozen_null_font+257 {dummy location}
/// dummy location
pub(crate) type undefined_control_sequence_TYPENUM = typenum::op!(frozen_null_font_TYPENUM + U257);
pub(crate) const undefined_control_sequence: word = undefined_control_sequence_TYPENUM::U32;
// @d glue_base=undefined_control_sequence+1 {beginning of region 3}
/// beginning of region 3
pub(crate) type glue_base_TYPENUM = typenum::op!(undefined_control_sequence_TYPENUM + U1);
pub(crate) const glue_base: word = glue_base_TYPENUM::U32;

// @<Initialize table entries...@>=
// eq_type(undefined_control_sequence):=undefined_cs;
// equiv(undefined_control_sequence):=null;
// eq_level(undefined_control_sequence):=level_zero;
// for k:=active_base to undefined_control_sequence-1 do
//   eqtb[k]:=eqtb[undefined_control_sequence];
//

use crate::pascal::word;
use crate::section_0012::hash_size_TYPENUM;
use crate::section_0113::halfword;
use typenum::Unsigned;
use typenum::{U1, U10, U256, U257};
