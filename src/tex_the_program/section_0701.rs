//! ` `
//! The math-extension parameters have similar macros, but the size code is
//! omitted (since it is always |cur_size| when we refer to such parameters).
//! @^parameters for symbols@>
//! @^font parameters@>
//
// @d mathex(#)==font_info[#+param_base[fam_fnt(3+cur_size)]].sc
pub(crate) macro mathex($globals:expr, $u:expr) {
    $globals.font_info[($u
        + $globals.param_base[crate::section_0230::fam_fnt!($globals, 3 + $globals.cur_size.get())])
        as u16][crate::section_0101::MEMORY_WORD_SC]
}
// @d default_rule_thickness==mathex(8) {thickness of \.{\\over} bars}
/// thickness of `\over` bars
pub(crate) macro default_rule_thickness($globals:expr) {
    crate::section_0701::mathex!($globals, 8)
}
// @d big_op_spacing1==mathex(9) {minimum clearance above a displayed op}
// @d big_op_spacing2==mathex(10) {minimum clearance below a displayed op}
// @d big_op_spacing3==mathex(11) {minimum baselineskip above displayed op}
// @d big_op_spacing4==mathex(12) {minimum baselineskip below displayed op}
// @d big_op_spacing5==mathex(13) {padding above and below displayed limits}
// @d total_mathex_params=13
pub(crate) const total_mathex_params: halfword = 13;

use crate::section_0113::halfword;
