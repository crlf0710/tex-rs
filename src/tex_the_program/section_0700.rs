//! ` `
//!
//! Before an mlist is converted to an hlist, \TeX\ makes sure that
//! the fonts in family~2 have enough parameters to be math-symbol
//! fonts, and that the fonts in family~3 have enough parameters to be
//! math-extension fonts. The math-symbol parameters are referred to by using the
//! following macros, which take a size code as their parameter; for example,
//! |num1(cur_size)| gives the value of the |num1| parameter for the current size.
//! @^parameters for symbols@>
//! @^font parameters@>

// @d mathsy_end(#)==fam_fnt(2+#)]].sc
// @d mathsy(#)==font_info[#+param_base[mathsy_end
pub(crate) macro mathsy($globals:expr, $u:expr, $v:expr) {
    $globals.font_info
        [($u + $globals.param_base[crate::section_0230::fam_fnt!($globals, 2 + $v)]) as u16]
        [crate::section_0101::MEMORY_WORD_SC]
}
// @d math_x_height==mathsy(5) {height of `\.x'}
/// height of `x'
pub(crate) macro math_x_height($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 5, $v)
}
// @d math_quad==mathsy(6) {\.{18mu}}
/// `18mu`
pub(crate) macro math_quad($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 6, $v)
}
// @d num1==mathsy(8) {numerator shift-up in display styles}
/// numerator shift-up in display styles
pub(crate) macro num1($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 8, $v)
}
// @d num2==mathsy(9) {numerator shift-up in non-display, non-\.{\\atop}}
/// numerator shift-up in non-display, non-`\atop`
pub(crate) macro num2($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 9, $v)
}
// @d num3==mathsy(10) {numerator shift-up in non-display \.{\\atop}}
/// numerator shift-up in non-display `\atop`
pub(crate) macro num3($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 10, $v)
}
// @d denom1==mathsy(11) {denominator shift-down in display styles}
/// denominator shift-down in display styles
pub(crate) macro denom1($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 11, $v)
}
// @d denom2==mathsy(12) {denominator shift-down in non-display styles}
/// denominator shift-down in non-display styles
pub(crate) macro denom2($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 12, $v)
}
// @d sup1==mathsy(13) {superscript shift-up in uncramped display style}
/// superscript shift-up in uncramped display style
pub(crate) macro sup1($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 13, $v)
}
// @d sup2==mathsy(14) {superscript shift-up in uncramped non-display}
/// superscript shift-up in uncramped non-display
pub(crate) macro sup2($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 14, $v)
}
// @d sup3==mathsy(15) {superscript shift-up in cramped styles}
/// superscript shift-up in cramped styles
pub(crate) macro sup3($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 15, $v)
}
// @d sub1==mathsy(16) {subscript shift-down if superscript is absent}
/// subscript shift-down if superscript is absent
pub(crate) macro sub1($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 16, $v)
}
// @d sub2==mathsy(17) {subscript shift-down if superscript is present}
// @d sup_drop==mathsy(18) {superscript baseline below top of large box}
/// superscript baseline below top of large box
pub(crate) macro sup_drop($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 18, $v)
}
// @d sub_drop==mathsy(19) {subscript baseline below bottom of large box}
/// subscript baseline below bottom of large box
pub(crate) macro sub_drop($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 19, $v)
}
// @d delim1==mathsy(20) {size of \.{\\atopwithdelims} delimiters
//   in display styles}
/// size of `\atopwithdelims` delimiters in display styles
pub(crate) macro delim1($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 20, $v)
}
// @d delim2==mathsy(21) {size of \.{\\atopwithdelims} delimiters in non-displays}
/// size of `\atopwithdelims` delimiters in non-displays
pub(crate) macro delim2($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 21, $v)
}
// @d axis_height==mathsy(22) {height of fraction lines above the baseline}
/// height of fraction lines above the baseline
pub(crate) macro axis_height($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 22, $v)
}
// @d total_mathsy_params=22
pub(crate) const total_mathsy_params: halfword = 22;

use crate::section_0113::halfword;
