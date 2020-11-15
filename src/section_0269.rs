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
// @d simple_group=1 {group code for local structure only}
// @d hbox_group=2 {code for `\.{\\hbox}\grp'}
// @d adjusted_hbox_group=3 {code for `\.{\\hbox}\grp' in vertical mode}
// @d vbox_group=4 {code for `\.{\\vbox}\grp'}
// @d vtop_group=5 {code for `\.{\\vtop}\grp'}
// @d align_group=6 {code for `\.{\\halign}\grp', `\.{\\valign}\grp'}
// @d no_align_group=7 {code for `\.{\\noalign}\grp'}
// @d output_group=8 {code for output routine}
// @d math_group=9 {code for, e.g., `\.{\char'136}\grp'}
// @d disc_group=10 {code for `\.{\\discretionary}\grp\grp\grp'}
// @d insert_group=11 {code for `\.{\\insert}\grp', `\.{\\vadjust}\grp'}
// @d vcenter_group=12 {code for `\.{\\vcenter}\grp'}
// @d math_choice_group=13 {code for `\.{\\mathchoice}\grp\grp\grp\grp'}
// @d semi_simple_group=14 {code for `\.{\\begingroup...\\endgroup}'}
// @d math_shift_group=15 {code for `\.{\$...\$}'}
// @d math_left_group=16 {code for `\.{\\left...\\right}'}
// @d max_group_code=16
pub(crate) const max_group_code: u8 = max_group_code_TYPENUM::U8;
pub(crate) type max_group_code_TYPENUM = U16;
//
// @<Types...@>=
// @!group_code=0..max_group_code; {|save_level| for a level boundary}
/// `save_level` for a level boundary
pub(crate) type group_code = u8_from_0_to_n<max_group_code_TYPENUM>;

use crate::pascal::u8_from_0_to_n;
use typenum::Unsigned;
use typenum::U16;