//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1171($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+math_style: tail_append(new_style(cur_chr));
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + math_style as u16 {
        tail_append!(
            $globals,
            new_style($globals, small_number::new($globals.cur_chr.get() as _))?
        );
        true
    }
    // mmode+non_script: begin tail_append(new_glue(zero_glue));
    else if $abs_mode_plus_cur_cmd == mmode as u16 + non_script as u16 {
        tail_append!($globals, new_glue($globals, zero_glue)?);
        // subtype(tail):=cond_math_glue;
        subtype!($globals, tail!($globals)) = glue_node_subtype::cond_math_glue as _;
        // end;
        true
    }
    // mmode+math_choice: append_choices;
    else if $abs_mode_plus_cur_cmd == mmode as u16 + math_choice as u16 {
        append_choices($globals)?;
        true
    } else {
        false
    };
    use crate::section_0101::small_number;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0153::new_glue;
    use crate::section_0162::zero_glue;
    use crate::section_0208::math_choice;
    use crate::section_0208::math_style;
    use crate::section_0208::non_script;
    use crate::section_0211::mmode;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0688::new_style;
    use crate::section_1172::append_choices;
    processed
}}
