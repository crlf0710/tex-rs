//! @ Whew---that covers the main loop. We can now proceed at a leisurely
//! pace through the other combinations of possibilities.
//
// @d any_mode(#)==vmode+#,hmode+#,mmode+# {for mode-independent commands}
macro_rules! abs_mode_plus_cur_cmd_matches_any_mode {
    ($abs_mode_plus_cur_cmd:expr, $cur_cmd:expr) => {
        $abs_mode_plus_cur_cmd == vmode as u16 + $cur_cmd
            || $abs_mode_plus_cur_cmd == hmode as u16 + $cur_cmd
            || $abs_mode_plus_cur_cmd == mmode as u16 + $cur_cmd
    }
}
//
// @<Cases of |main_control| that are not part of the inner loop@>=
macro_rules! Cases_of_main_control_that_are_not_part_of_the_inner_loop {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {
        // any_mode(relax),vmode+spacer,mmode+spacer,mmode+no_boundary:do_nothing;
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, relax as u16)
            || $abs_mode_plus_cur_cmd == vmode as u16 + spacer as u16
            || $abs_mode_plus_cur_cmd == mmode as u16 + spacer as u16
            || $abs_mode_plus_cur_cmd == mmode as u16 + no_boundary as u16
        {
            do_nothing!();
        }
        // any_mode(ignore_spaces): begin @<Get the next non-blank non-call...@>;
        else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, ignore_spaces as u16) {
            todo!();
            // goto reswitch;
            // end;
        }
        // vmode+stop: if its_all_over then return; {this is the only way out}
        else if $abs_mode_plus_cur_cmd == vmode as u16 + stop as u16 {
            if its_all_over($globals) {
                /// this is the only way out
                {
                    return_nojump!();
                }
            }
        }
        else {
            todo!();
            // @t\4@>@<Forbidden cases detected in |main_control|@>@+@,any_mode(mac_param):
            //   report_illegal_case;
            // @<Math-only cases in non-math modes, or vice versa@>: insert_dollar_sign;
            // @t\4@>@<Cases of |main_control| that build boxes and lists@>@;
            // @t\4@>@<Cases of |main_control| that don't depend on |mode|@>@;
            // @t\4@>@<Cases of |main_control| that are for extensions to \TeX@>@;
        }
        use crate::section_1054::its_all_over;
    }
}