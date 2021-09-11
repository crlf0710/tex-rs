//! @ Constructions that require a box are started by calling |scan_box| with
//! a specified context code. The |scan_box| routine verifies
//! that a |make_box| command comes next and then it calls |begin_box|.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1073($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    let processed = if $abs_mode_plus_cur_cmd == vmode as u16 + hmove as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + vmove as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + vmove as u16
    {
        // vmode+hmove,hmode+vmove,mmode+vmove: begin t:=cur_chr;
        //   scan_normal_dimen;
        //   if t=0 then scan_box(cur_val)@+else scan_box(-cur_val);
        //   end;
        todo!("hmove/vmove");
        true
    } else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, leader_ship as u16) {
        // any_mode(leader_ship): scan_box(leader_flag-a_leaders+cur_chr);
        scan_box(
            $globals,
            leader_flag - glue_node_subtype::a_leaders as integer
                + $globals.cur_chr.get() as integer,
        )?;
        use crate::pascal::integer;
        use crate::section_0149::glue_node_subtype;
        use crate::section_1071::leader_flag;
        use crate::section_1084::scan_box;
        true
    } else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, make_box as u16) {
        // any_mode(make_box): begin_box(0);
        begin_box($globals, 0)?;
        use crate::section_1079::begin_box;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
