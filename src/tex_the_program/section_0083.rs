//! ` `
// @<Get user's advice...@>=
pub(crate) macro Get_user_s_advice_and_return {
    ($globals:expr) => {
        crate::trace_span!("Get user's advice...");
        // loop@+begin continue: if interaction<>error_stop_mode then return;
        //   clear_for_error_prompt; prompt_input("? ");
        loop {
            crate::region_backward_label!(
                'continue_ <-
                {
                    if $globals.interaction != error_stop_mode {
                        crate::return_nojump!();
                    }
                    clear_for_error_prompt($globals);
                    prompt_input!($globals, crate::strpool_str!("? "));
                    // @.?\relax@>
                    // if last=first then return;
                    if $globals.last == $globals.first {
                        crate::return_nojump!();
                    }
                    /// what the user types
                    let mut c: ASCII_code = $globals.buffer[$globals.first];
                    // if c>="a" then c:=c+"A"-"a"; {convert to uppercase}
                    /// convert to uppercase
                    if c >= ASCII_code_literal!(b'a') && c <= ASCII_code_literal!(b'z') {
                        c = ASCII_code_literal!(c.numeric_value() as u8 + b'A' - b'a');
                    }
                    // @<Interpret code |c| and |return| if done@>;
                    crate::section_0084::Interpret_code_c_and_return_if_done!($globals, c, 'continue_);
                    // end
                }
                |'continue_|
            );
        }
        use crate::section_0018::ASCII_code;
        use crate::section_0018::ASCII_code_literal;
        use crate::section_0071::prompt_input;
        use crate::section_0073::error_stop_mode;
        use crate::section_0330::clear_for_error_prompt;
    }
}
