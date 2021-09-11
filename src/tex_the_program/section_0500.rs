//! ` `

// @<Skip to \.{\\else} or \.{\\fi}...@>=
pub(crate) macro Skip_to_else_or_fi__then_goto_common_ending($globals:expr, $save_cond_ptr:expr, $lbl_common_ending:lifetime) {{
    // loop@+  begin pass_text;
    loop {
        pass_text($globals)?;
        // if cond_ptr=save_cond_ptr then
        if $globals.cond_ptr == $save_cond_ptr {
            // begin if cur_chr<>or_code then goto common_ending;
            if $globals.cur_chr.get() != or_code as chr_code_repr {
                crate::goto_forward_label!($lbl_common_ending);
            }
            todo!("extra \\or");
            // print_err("Extra "); print_esc("or");
            // @.Extra \\or@>
            // help1("I'm ignoring this; it doesn't match any \if.");
            // error;
            // end
        }
        // else if cur_chr=fi_code then @<Pop the condition stack@>;
        else if $globals.cur_chr.get() == fi_code as chr_code_repr {
            todo!("pop stack");
        }
        // end
    }
    use crate::section_0297::chr_code_repr;
    use crate::section_0489::fi_code;
    use crate::section_0489::or_code;
    use crate::section_0494::pass_text;
}}
