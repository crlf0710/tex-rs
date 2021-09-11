//! @ The |un_hbox| and |un_vbox| commands unwrap one of the 256 current boxes.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1109($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // vmode+un_vbox,hmode+un_hbox,mmode+un_hbox: unpackage;
    let processed = if $abs_mode_plus_cur_cmd == vmode as u16 + un_vbox as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + un_hbox as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + un_hbox as u16
    {
        unpackage($globals)?;
        use crate::section_1110::unpackage;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    processed
}}
