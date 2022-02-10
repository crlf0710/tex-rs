//! @ When erroneous situations arise, \TeX\ usually issues an error message
//! specific to the particular error. For example, `\.{\\noalign}' should
//! not appear in any mode, since it is recognized by the |align_peek| routine
//! in all of its legitimate appearances; a special error message is given
//! when `\.{\\noalign}' occurs elsewhere. But sometimes the most appropriate
//! error message is simply that the user is not allowed to do what he or she
//! has attempted. For example, `\.{\\moveleft}' is allowed only in vertical mode,
//! and `\.{\\lower}' only in non-vertical modes.  Such cases are enumerated
//! here and in the other sections referred to under `See also \dots.'
//
// @<Forbidden cases...@>=
pub(crate) macro Forbidden_cases_detected_in_main_control_1048($abs_mode_plus_cur_cmd:expr) {{
    // vmode+vmove,hmode+hmove,mmode+hmove,any_mode(last_item),
    let result = $abs_mode_plus_cur_cmd == vmode as u16 + vmove as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + hmove as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + hmove as u16
        || crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode!(
            $abs_mode_plus_cur_cmd,
            last_item as u16
        );
    use crate::section_0208::*;
    use crate::section_0211::*;
    result
}}
