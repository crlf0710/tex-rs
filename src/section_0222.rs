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
pub(crate) const active_base: word = 1;
// @d single_base=active_base+256 {equivalents of one-character control sequences}
/// equivalents of one-character control sequences
pub(crate) const single_base: word = active_base + 256;
// @d null_cs=single_base+256 {equivalent of \.{\\csname\\endcsname}}
/// equivalent of `\csname\endcsname`
pub(crate) const null_cs: word = single_base + 256;
// @d hash_base=null_cs+1 {beginning of region 2, for the hash table}
/// beginning of region 2, for the hash table
pub(crate) const hash_base: word = null_cs + 1;
// @d frozen_control_sequence=hash_base+hash_size {for error recovery}
/// for error recovery
pub(crate) const frozen_control_sequence: word = hash_base + hash_size as word;
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
pub(crate) const frozen_null_font: word = frozen_control_sequence + 10;
// @d font_id_base=frozen_null_font-font_base
//   {begins table of 257 permanent font identifiers}
// @d undefined_control_sequence=frozen_null_font+257 {dummy location}
/// dummy location
pub(crate) const undefined_control_sequence: word = frozen_null_font + 257;
// @d glue_base=undefined_control_sequence+1 {beginning of region 3}
/// beginning of region 3
pub(crate) const glue_base: word = undefined_control_sequence + 1;

// @<Initialize table entries...@>=
// eq_type(undefined_control_sequence):=undefined_cs;
// equiv(undefined_control_sequence):=null;
// eq_level(undefined_control_sequence):=level_zero;
// for k:=active_base to undefined_control_sequence-1 do
//   eqtb[k]:=eqtb[undefined_control_sequence];
//

use crate::pascal::word;
use crate::section_0012::hash_size;
