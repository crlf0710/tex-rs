//! @ Whew---that covers the main loop. We can now proceed at a leisurely
//! pace through the other combinations of possibilities.
//
// @d any_mode(#)==vmode+#,hmode+#,mmode+# {for mode-independent commands}
macro_rules! abs_mode_plus_cur_cmd_matches_any_mode {
    ($abs_mode_plus_cur_cmd:expr, $cur_cmd:expr) => {
        $abs_mode_plus_cur_cmd == vmode as u16 + $cur_cmd
            || $abs_mode_plus_cur_cmd == hmode as u16 + $cur_cmd
            || $abs_mode_plus_cur_cmd == mmode as u16 + $cur_cmd
    };
}

macro_rules! Cases_of_main_control_that_build_boxes_and_lists {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        trace_span!("Cases of `main_control` that build...");
        if Cases_of_main_control_that_build_boxes_and_lists_1056!($globals, $abs_mode_plus_cur_cmd)
        {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1057!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1063!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1067!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1073!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1090!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1094!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1102!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1109!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1126!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1130!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1137!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1154!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1167!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_build_boxes_and_lists_1193!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else {
            false
        }
    }};
}

macro_rules! Cases_of_main_control_that_dont_depend_on_mode {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        trace_span!("Cases of `main_control` that don't...");
        if Cases_of_main_control_that_dont_depend_on_mode_1210!($globals, $abs_mode_plus_cur_cmd) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_dont_depend_on_mode_1274!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_dont_depend_on_mode_1276!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_dont_depend_on_mode_1285!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else if Cases_of_main_control_that_dont_depend_on_mode_1290!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else {
            false
        }
    }};
}

macro_rules! Cases_of_main_control_that_are_for_extensions_to_TeX {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        trace_span!("Cases of `main_control` that are for...");
        if Cases_of_main_control_that_are_for_extensions_to_TeX_1347!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
            true
        } else {
            false
        }
    }};
}

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
        else if abs_mode_plus_cur_cmd_matches_any_mode!(
            $abs_mode_plus_cur_cmd,
            ignore_spaces as u16
        ) {
            todo!("ignore_spaces");
            // goto reswitch;
            // end;
        }
        // vmode+stop: if its_all_over then return; {this is the only way out}
        else if $abs_mode_plus_cur_cmd == vmode as u16 + stop as u16 {
            if its_all_over($globals)? {
                /// this is the only way out
                {
                    return_nojump!();
                }
            }
        }
        // @t\4@>@<Forbidden cases detected in |main_control|@>@+@,any_mode(mac_param):
        //   report_illegal_case;
        else if Forbidden_cases_detected_in_main_control!($abs_mode_plus_cur_cmd)
            || abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, mac_param as u16)
        {
            report_illegal_case($globals)?;
        }
        // @<Math-only cases in non-math modes, or vice versa@>: insert_dollar_sign;
        else if Math_only_cases_in_non_math_modes_or_vice_versa!($abs_mode_plus_cur_cmd) {
            insert_dollar_sign($globals);
        }
        // @t\4@>@<Cases of |main_control| that build boxes and lists@>@;
        else if Cases_of_main_control_that_build_boxes_and_lists!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
        }
        // @t\4@>@<Cases of |main_control| that don't depend on |mode|@>@;
        else if Cases_of_main_control_that_dont_depend_on_mode!($globals, $abs_mode_plus_cur_cmd)
        {
            /// already processed
            do_nothing!();
        }
        // @t\4@>@<Cases of |main_control| that are for extensions to \TeX@>@;
        else if Cases_of_main_control_that_are_for_extensions_to_TeX!(
            $globals,
            $abs_mode_plus_cur_cmd
        ) {
            /// already processed
            do_nothing!();
        } else {
            unreachable!(
                "abs(mode) = {}, cur_cmd = {}",
                $abs_mode_plus_cur_cmd - $globals.cur_cmd as u16,
                $globals.cur_cmd
            );
        }
        use crate::section_0209::*;
        use crate::section_1047::insert_dollar_sign;
        use crate::section_1050::report_illegal_case;
        use crate::section_1054::its_all_over;
    };
}
