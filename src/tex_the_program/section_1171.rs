//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1171($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+math_style: tail_append(new_style(cur_chr));
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + math_style as u16 {
        tail_append!(
            $globals,
            new_style($globals, small_number::new($globals.cur_chr.get() as _))?
        );
        use crate::section_0214::tail_append;
        use crate::section_0688::new_style;
        true
    }
    // mmode+non_script: begin tail_append(new_glue(zero_glue));
    else if $abs_mode_plus_cur_cmd == mmode as u16 + non_script as u16 {
        // subtype(tail):=cond_math_glue;
        // end;
        todo!("mmode+non_script");
        true
    }
    // mmode+math_choice: append_choices;
    else if $abs_mode_plus_cur_cmd == mmode as u16 + math_choice as u16 {
        todo!("append_choices");
        true
    } else {
        false
    };
    use crate::section_0101::small_number;
    use crate::section_0208::math_choice;
    use crate::section_0208::math_style;
    use crate::section_0208::non_script;
    use crate::section_0211::mmode;
    processed
}}
