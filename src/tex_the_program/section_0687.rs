//! @ A few more kinds of noads will complete the set: An |under_noad| has its
//! nucleus underlined; an |over_noad| has it overlined. An |accent_noad| places
//! an accent over its nucleus; the accent character appears as
//! |fam(accent_chr(p))| and |character(accent_chr(p))|. A |vcenter_noad|
//! centers its nucleus vertically with respect to the axis of the formula;
//! in such noads we always have |math_type(nucleus(p))=sub_box|.
//!
//! And finally, we have |left_noad| and |right_noad| types, to implement
//! \TeX's \.{\\left} and \.{\\right}. The |nucleus| of such noads is
//! replaced by a |delimiter| field; thus, for example, `\.{\\left(}' produces
//! a |left_noad| such that |delimiter(p)| holds the family and character
//! codes for all left parentheses. A |left_noad| never appears in an mlist
//! except as the first element, and a |right_noad| never appears in an mlist
//! except as the last element; furthermore, we either have both a |left_noad|
//! and a |right_noad|, or neither one is present. The |subscr| and |supscr|
//! fields are always |empty| in a |left_noad| and a |right_noad|.
//
// @d under_noad=fraction_noad+1 {|type| of a noad for underlining}
/// `type` of a noad for underlining
pub(crate) const under_noad: quarterword = fraction_noad + 1;
// @d over_noad=under_noad+1 {|type| of a noad for overlining}
/// `type` of a noad for overlining
pub(crate) const over_noad: quarterword = under_noad + 1;
// @d accent_noad=over_noad+1 {|type| of a noad for accented subformulas}
/// `type` of a noad for accented subformulas
pub(crate) const accent_noad: quarterword = over_noad + 1;
// @d accent_noad_size=5 {number of |mem| words in an accent noad}
// @d accent_chr(#)==#+4 {the |accent_chr| field of an accent noad}
// @d vcenter_noad=accent_noad+1 {|type| of a noad for \.{\\vcenter}}
/// `type` of a noad for `\vcenter`
pub(crate) const vcenter_noad: quarterword = accent_noad + 1;
// @d left_noad=vcenter_noad+1 {|type| of a noad for \.{\\left}}
/// `type` of a noad for `\left`
pub(crate) const left_noad: quarterword = vcenter_noad + 1;
// @d right_noad=left_noad+1 {|type| of a noad for \.{\\right}}
/// `type` of a noad for `\right`
pub(crate) const right_noad: quarterword = left_noad + 1;
// @d delimiter==nucleus {|delimiter| field in left and right noads}
/// `delimiter` field in left and right noads
pub(crate) macro delimiter($v:expr) {
    crate::section_0681::nucleus!($v)
}
// @d scripts_allowed(#)==(type(#)>=ord_noad)and(type(#)<left_noad)
pub(crate) macro scripts_allowed($globals:expr, $v:expr) {
    crate::section_0133::r#type!($globals, $v) >= crate::section_0682::ord_noad
        && crate::section_0133::r#type!($globals, $v) < left_noad
}

use crate::section_0113::quarterword;
use crate::section_0683::fraction_noad;
