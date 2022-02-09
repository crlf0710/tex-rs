//! @ Whew---that covers the main loop. We can now proceed at a leisurely
//! pace through the other combinations of possibilities.
//
// @d any_mode(#)==vmode+#,hmode+#,mmode+# {for mode-independent commands}
pub(crate) macro abs_mode_plus_cur_cmd_matches_any_mode($abs_mode_plus_cur_cmd:expr, $cur_cmd:expr) {
    $abs_mode_plus_cur_cmd == crate::section_0211::vmode as u16 + $cur_cmd
        || $abs_mode_plus_cur_cmd == crate::section_0211::hmode as u16 + $cur_cmd
        || $abs_mode_plus_cur_cmd == crate::section_0211::mmode as u16 + $cur_cmd
}

pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    crate::trace_span_verbose!("Cases of `main_control` that build...");
    let processed = if crate::section_1056::Cases_of_main_control_that_build_boxes_and_lists_1056!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1057::Cases_of_main_control_that_build_boxes_and_lists_1057!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1063::Cases_of_main_control_that_build_boxes_and_lists_1063!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1067::Cases_of_main_control_that_build_boxes_and_lists_1067!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1073::Cases_of_main_control_that_build_boxes_and_lists_1073!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1090::Cases_of_main_control_that_build_boxes_and_lists_1090!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1092::Cases_of_main_control_that_build_boxes_and_lists_1092!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1094::Cases_of_main_control_that_build_boxes_and_lists_1094!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1097::Cases_of_main_control_that_build_boxes_and_lists_1097!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1102::Cases_of_main_control_that_build_boxes_and_lists_1102!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1104::Cases_of_main_control_that_build_boxes_and_lists_1104!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1109::Cases_of_main_control_that_build_boxes_and_lists_1109!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1112::Cases_of_main_control_that_build_boxes_and_lists_1112!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1116::Cases_of_main_control_that_build_boxes_and_lists_1116!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1122::Cases_of_main_control_that_build_boxes_and_lists_1122!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1126::Cases_of_main_control_that_build_boxes_and_lists_1126!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1130::Cases_of_main_control_that_build_boxes_and_lists_1130!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1137::Cases_of_main_control_that_build_boxes_and_lists_1137!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1150::Cases_of_main_control_that_build_boxes_and_lists_1150!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1154::Cases_of_main_control_that_build_boxes_and_lists_1154!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1158::Cases_of_main_control_that_build_boxes_and_lists_1158!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1162::Cases_of_main_control_that_build_boxes_and_lists_1162!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1164::Cases_of_main_control_that_build_boxes_and_lists_1164!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1167::Cases_of_main_control_that_build_boxes_and_lists_1167!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1171::Cases_of_main_control_that_build_boxes_and_lists_1171!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1175::Cases_of_main_control_that_build_boxes_and_lists_1175!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1180::Cases_of_main_control_that_build_boxes_and_lists_1180!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1190::Cases_of_main_control_that_build_boxes_and_lists_1190!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1193::Cases_of_main_control_that_build_boxes_and_lists_1193!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else {
        false
    };
    use crate::section_0016::do_nothing;
    processed
}}

pub(crate) macro Cases_of_main_control_that_dont_depend_on_mode($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    crate::trace_span_verbose!("Cases of `main_control` that don't...");
    let processed = if crate::section_1210::Cases_of_main_control_that_dont_depend_on_mode_1210!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1268::Cases_of_main_control_that_dont_depend_on_mode_1268!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1271::Cases_of_main_control_that_dont_depend_on_mode_1271!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1274::Cases_of_main_control_that_dont_depend_on_mode_1274!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1276::Cases_of_main_control_that_dont_depend_on_mode_1276!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1285::Cases_of_main_control_that_dont_depend_on_mode_1285!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else if crate::section_1290::Cases_of_main_control_that_dont_depend_on_mode_1290!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else {
        false
    };
    use crate::section_0016::do_nothing;
    processed
}}

pub(crate) macro Cases_of_main_control_that_are_for_extensions_to_TeX($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    crate::trace_span!("Cases of `main_control` that are for...");
    let processed = if crate::section_1347::Cases_of_main_control_that_are_for_extensions_to_TeX_1347!(
        $globals,
        $abs_mode_plus_cur_cmd
    ) {
        /// already processed
        do_nothing!();
        true
    } else {
        false
    };
    use crate::section_0016::do_nothing;
    processed
}}

// @<Cases of |main_control| that are not part of the inner loop@>=
pub(crate) macro Cases_of_main_control_that_are_not_part_of_the_inner_loop($globals:expr, $abs_mode_plus_cur_cmd:expr, $lbl_reswitch:lifetime) {
    // any_mode(relax),vmode+spacer,mmode+spacer,mmode+no_boundary:do_nothing;
    if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, relax as u16)
        || $abs_mode_plus_cur_cmd == vmode as u16 + spacer as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + spacer as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + no_boundary as u16
    {
        do_nothing!();
    }
    // any_mode(ignore_spaces): begin @<Get the next non-blank non-call...@>;
    else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, ignore_spaces as u16)
    {
        crate::section_0406::Get_the_next_non_blank_non_call_token!($globals);
        // goto reswitch;
        crate::goto_backward_label!($lbl_reswitch);
        // end;
    }
    // vmode+stop: if its_all_over then return; {this is the only way out}
    else if $abs_mode_plus_cur_cmd == vmode as u16 + stop as u16 {
        if its_all_over($globals)? {
            /// this is the only way out
            const _: () = ();
            crate::return_nojump!();
        }
    }
    // @t\4@>@<Forbidden cases detected in |main_control|@>@+@,any_mode(mac_param):
    //   report_illegal_case;
    else if crate::section_1048::Forbidden_cases_detected_in_main_control!($abs_mode_plus_cur_cmd)
        || abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, mac_param as u16)
    {
        report_illegal_case($globals)?;
    }
    // @<Math-only cases in non-math modes, or vice versa@>: insert_dollar_sign;
    else if crate::section_1046::Math_only_cases_in_non_math_modes_or_vice_versa!(
        $abs_mode_plus_cur_cmd
    ) {
        insert_dollar_sign($globals)?;
    }
    // @t\4@>@<Cases of |main_control| that build boxes and lists@>@;
    else if Cases_of_main_control_that_build_boxes_and_lists!($globals, $abs_mode_plus_cur_cmd) {
        /// already processed
        do_nothing!();
    }
    // @t\4@>@<Cases of |main_control| that don't depend on |mode|@>@;
    else if Cases_of_main_control_that_dont_depend_on_mode!($globals, $abs_mode_plus_cur_cmd) {
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
    use crate::section_0016::do_nothing;
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_0209::*;
    use crate::section_0211::*;
    use crate::section_1047::insert_dollar_sign;
    use crate::section_1050::report_illegal_case;
    use crate::section_1054::its_all_over;
}
