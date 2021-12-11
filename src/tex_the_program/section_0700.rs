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
// @d math_quad==mathsy(6) {\.{18mu}}
/// `18mu`
pub(crate) macro math_quad($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 6, $v)
}
// @d num1==mathsy(8) {numerator shift-up in display styles}
// @d num2==mathsy(9) {numerator shift-up in non-display, non-\.{\\atop}}
// @d num3==mathsy(10) {numerator shift-up in non-display \.{\\atop}}
// @d denom1==mathsy(11) {denominator shift-down in display styles}
// @d denom2==mathsy(12) {denominator shift-down in non-display styles}
// @d sup1==mathsy(13) {superscript shift-up in uncramped display style}
// @d sup2==mathsy(14) {superscript shift-up in uncramped non-display}
// @d sup3==mathsy(15) {superscript shift-up in cramped styles}
// @d sub1==mathsy(16) {subscript shift-down if superscript is absent}
// @d sub2==mathsy(17) {subscript shift-down if superscript is present}
// @d sup_drop==mathsy(18) {superscript baseline below top of large box}
// @d sub_drop==mathsy(19) {subscript baseline below bottom of large box}
// @d delim1==mathsy(20) {size of \.{\\atopwithdelims} delimiters
//   in display styles}
// @d delim2==mathsy(21) {size of \.{\\atopwithdelims} delimiters in non-displays}
// @d axis_height==mathsy(22) {height of fraction lines above the baseline}
/// height of fraction lines above the baseline
pub(crate) macro axis_height($globals:expr, $v:expr) {
    crate::section_0700::mathsy!($globals, 22, $v)
}
// @d total_mathsy_params=22
pub(crate) const total_mathsy_params: halfword = 22;

use crate::section_0113::halfword;
