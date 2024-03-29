//! @ When the right brace occurs at the end of an \.{\\hbox} or \.{\\vbox} or
//! \.{\\vtop} construction, the |package| routine comes into action. We might
//! also have to finish a paragraph that hasn't ended.
//
// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1085($globals:expr) {{
    // hbox_group: package(0);
    let processed = if $globals.cur_group == hbox_group {
        package($globals, 0.into())?;
        use crate::section_1086::package;
        true
    }
    // adjusted_hbox_group: begin adjust_tail:=adjust_head; package(0);
    else if $globals.cur_group == adjusted_hbox_group {
        $globals.adjust_tail = adjust_head;
        package($globals, 0.into())?;
        // end;
        use crate::section_0162::adjust_head;
        use crate::section_1086::package;
        true
    }
    // vbox_group: begin end_graf; package(0);
    else if $globals.cur_group == vbox_group {
        end_graf($globals)?;
        package($globals, 0.into())?;
        // end;
        use crate::section_1086::package;
        use crate::section_1096::end_graf;
        true
    }
    // vtop_group: begin end_graf; package(vtop_code);
    else if $globals.cur_group == vtop_group {
        end_graf($globals)?;
        package($globals, small_number::new(vtop_code as _))?;
        use crate::section_0101::small_number;
        use crate::section_1071::vtop_code;
        use crate::section_1086::package;
        use crate::section_1096::end_graf;
        true
    }
    //   end;
    else {
        false
    };
    use crate::section_0269::*;
    processed
}}
