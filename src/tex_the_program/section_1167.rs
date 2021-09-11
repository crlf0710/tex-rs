//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1167($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+vcenter: begin scan_spec(vcenter_group,false); normal_paragraph;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + vcenter as u16 {
        scan_spec($globals, vcenter_group.into(), false)?;
        normal_paragraph($globals)?;
        // push_nest; mode:=-vmode; prev_depth:=ignore_depth;
        push_nest($globals);
        mode!($globals) = (-vmode).into();
        prev_depth!($globals) = ignore_depth;
        // if every_vbox<>null then begin_token_list(every_vbox,every_vbox_text);
        if every_vbox!($globals) != null {
            begin_token_list($globals, every_vbox!($globals), every_vbox_text);
        }
        // end;
        use crate::section_0115::null;
        use crate::section_0212::ignore_depth;
        use crate::section_0213::mode;
        use crate::section_0213::prev_depth;
        use crate::section_0216::push_nest;
        use crate::section_0230::every_vbox;
        use crate::section_0269::vcenter_group;
        use crate::section_0307::every_vbox_text;
        use crate::section_0323::begin_token_list;
        use crate::section_0645::scan_spec;
        use crate::section_1070::normal_paragraph;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    processed
}}
