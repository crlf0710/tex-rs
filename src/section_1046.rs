//! @ Here is a list of cases where the user has probably gotten into or out of math
//! mode by mistake. \TeX\ will insert a dollar sign and rescan the current token.

// @d non_math(#)==vmode+#,hmode+#
macro_rules! abs_mode_plus_cur_cmd_matches_non_math_mode {
    ($abs_mode_plus_cur_cmd:expr, $cur_cmd:expr) => {
        $abs_mode_plus_cur_cmd == vmode as u16 + $cur_cmd
            || $abs_mode_plus_cur_cmd == hmode as u16 + $cur_cmd
    }
}

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
        if abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, sup_mark as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, sub_mark as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_char_num as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_given as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_comp as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, delim_num as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, left_right as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, above as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, radical as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_style as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_choice as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, vcenter as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, non_script as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, mkern as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, limit_switch as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, mskip as u16) ||
            abs_mode_plus_cur_cmd_matches_non_math_mode!($abs_mode_plus_cur_cmd, math_accent as u16) ||
            $abs_mode_plus_cur_cmd == mmode as u16 + endv as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + par_end as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + stop as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + vskip as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + un_vbox as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + valign as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + hrule as u16
        {
            true
        } else {
            false
        }
    }
}
