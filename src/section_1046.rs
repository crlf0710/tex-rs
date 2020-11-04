//! @ Here is a list of cases where the user has probably gotten into or out of math
//! mode by mistake. \TeX\ will insert a dollar sign and rescan the current token.
//
// @d non_math(#)==vmode+#,hmode+#
//
// @<Math-only cases in non-math modes...@>=
macro_rules! Math_only_cases_in_non_math_modes_or_vice_versa {
    ($abs_mode_plus_cur_cmd:expr) => {
        // non_math(sup_mark), non_math(sub_mark), non_math(math_char_num),
        // non_math(math_given), non_math(math_comp), non_math(delim_num),
        // non_math(left_right), non_math(above), non_math(radical),
        // non_math(math_style), non_math(math_choice), non_math(vcenter),
        // non_math(non_script), non_math(mkern), non_math(limit_switch),
        // non_math(mskip), non_math(math_accent),
        // mmode+endv, mmode+par_end, mmode+stop, mmode+vskip, mmode+un_vbox,
        // mmode+valign, mmode+hrule
        false
    }
}
