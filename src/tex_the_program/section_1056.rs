//! @ As an introduction to these routines, let's consider one of the simplest
//! cases: What happens when `\.{\\hrule}' occurs in vertical mode, or
//! `\.{\\vrule}' in horizontal mode or math mode? The code in |main_control|
//! is short, since the |scan_rule_spec| routine already does most of what is
//! required; thus, there is no need for a special action procedure.
//!
//! Note that baselineskip calculations are disabled after a rule in vertical
//! mode, by setting |prev_depth:=ignore_depth|.
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1056 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // vmode+hrule,hmode+vrule,mmode+vrule: begin tail_append(scan_rule_spec);
        if $abs_mode_plus_cur_cmd == vmode as u16 + hrule as u16
            || $abs_mode_plus_cur_cmd == hmode as u16 + vrule as u16
            || $abs_mode_plus_cur_cmd == mmode as u16 + vrule as u16
        {
            tail_append!($globals, scan_rule_spec($globals)?);
            // if abs(mode)=vmode then prev_depth:=ignore_depth
            if mode!($globals).get().abs() == vmode {
                prev_depth!($globals) = ignore_depth;
            }
            // else if abs(mode)=hmode then space_factor:=1000;
            else if mode!($globals).get().abs() == hmode {
                space_factor!($globals) = 1000;
            }
            // end;
            use crate::section_0212::ignore_depth;
            use crate::section_0463::scan_rule_spec;
            true
        } else {
            false
        }
    }};
}
