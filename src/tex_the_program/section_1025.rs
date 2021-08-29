//! ` `
// @<Fire up the user's output routine and |return|@>=
macro_rules! Fire_up_the_user_s_output_routine_and_return {
    ($globals:expr) => {{
        // begin output_active:=true;
        $globals.output_active = true;
        // incr(dead_cycles);
        incr!($globals.dead_cycles);
        // push_nest; mode:=-vmode; prev_depth:=ignore_depth; mode_line:=-line;
        push_nest($globals);
        mode!($globals) = (-vmode).into();
        prev_depth!($globals) = ignore_depth;
        mode_line!($globals) = -$globals.line;
        // begin_token_list(output_routine,output_text);
        begin_token_list($globals, output_routine!($globals), output_text);
        // new_save_level(output_group); normal_paragraph;
        new_save_level($globals, output_group.into());
        normal_paragraph($globals)?;
        // scan_left_brace;
        scan_left_brace($globals)?;
        // return;
        return_nojump!();
        // end
        use crate::section_0211::vmode;
        use crate::section_0212::ignore_depth;
        use crate::section_0216::push_nest;
        use crate::section_0269::output_group;
        use crate::section_0274::new_save_level;
        use crate::section_0307::output_text;
        use crate::section_0323::begin_token_list;
        use crate::section_0403::scan_left_brace;
        use crate::section_1070::normal_paragraph;
    }}
}
