//! @ Here are the group codes that are used to discriminate between different
//! kinds of groups. They allow \TeX\ to decide what special actions, if any,
//! should be performed when a group ends.
//! \def\grp{\.{\char'173...\char'175}}
//!
//! Some groups are not supposed to be ended by right braces. For example,
//! the `\.\$' that begins a math formula causes a |math_shift_group| to
//! be started, and this should be terminated by a matching `\.\$'. Similarly,
//! a group that starts with \.{\\left} should end with \.{\\right}, and
//! one that starts with \.{\\begingroup} should end with \.{\\endgroup}.
//
// @d bottom_level=0 {group code for the outside world}
/// group code for the outside world
pub(crate) const bottom_level: quarterword = 0;
// @d simple_group=1 {group code for local structure only}
/// group code for local structure only
pub(crate) const simple_group: quarterword = 1;
// @d hbox_group=2 {code for `\.{\\hbox}\grp'}
/// code for `\hbox{...}`
pub(crate) const hbox_group: quarterword = 2;
// @d adjusted_hbox_group=3 {code for `\.{\\hbox}\grp' in vertical mode}
/// code for `\hbox{...}` in vertical mode
pub(crate) const adjusted_hbox_group: quarterword = 3;
// @d vbox_group=4 {code for `\.{\\vbox}\grp'}
/// code for `\vbox{...}`
pub(crate) const vbox_group: quarterword = 4;
// @d vtop_group=5 {code for `\.{\\vtop}\grp'}
/// code for `\vtop{...}`
pub(crate) const vtop_group: quarterword = 5;
// @d align_group=6 {code for `\.{\\halign}\grp', `\.{\\valign}\grp'}
/// code for `\halign{...}`, `\valign{...}`
pub(crate) const align_group: quarterword = 6;
// @d no_align_group=7 {code for `\.{\\noalign}\grp'}
/// code for `\noalign{...}`
pub(crate) const no_align_group: quarterword = 7;
// @d output_group=8 {code for output routine}
/// code for output routine
pub(crate) const output_group: quarterword = 8;
// @d math_group=9 {code for, e.g., `\.{\char'136}\grp'}
/// code for, e.g., `\char'136{...}`
pub(crate) const math_group: quarterword = 9;
// @d disc_group=10 {code for `\.{\\discretionary}\grp\grp\grp'}
/// code for `\discretionary{...}{...}{...}`
pub(crate) const disc_group: quarterword = 10;
// @d insert_group=11 {code for `\.{\\insert}\grp', `\.{\\vadjust}\grp'}
/// code for `\insert{...}`, `\vadjust{...}`
pub(crate) const insert_group: quarterword = 11;
// @d vcenter_group=12 {code for `\.{\\vcenter}\grp'}
/// code for `\vcenter{...}`
pub(crate) const vcenter_group: quarterword = 12;
// @d math_choice_group=13 {code for `\.{\\mathchoice}\grp\grp\grp\grp'}
/// code for `\mathchoice{...}{...}{...}{...}`
pub(crate) const math_choice_group: quarterword = 13;
// @d semi_simple_group=14 {code for `\.{\\begingroup...\\endgroup}'}
/// code for `\begingroup...\endgroup`
pub(crate) const semi_simple_group: quarterword = 14;
// @d math_shift_group=15 {code for `\.{\$...\$}'}
/// code for `$...$`
pub(crate) const math_shift_group: quarterword = 15;
// @d math_left_group=16 {code for `\.{\\left...\\right}'}
/// code for `\left...\right`
pub(crate) const math_left_group: quarterword = 16;
// @d max_group_code=16
pub(crate) const max_group_code: u8 = max_group_code_TYPENUM::U8;
pub(crate) type max_group_code_TYPENUM = U16;
//
// @<Types...@>=
// @!group_code=0..max_group_code; {|save_level| for a level boundary}
/// `save_level` for a level boundary
pub(crate) type group_code = u8_from_0_to_n<max_group_code_TYPENUM>;

use crate::pascal::u8_from_0_to_n;
use crate::section_0113::quarterword;
use typenum::Unsigned;
use typenum::U16;
