//! ` `
// @<Get user's advice...@>=
macro_rules! Get_user_s_advice_and_return {
    ($globals:expr) => {
        trace_span!("Get user's advice...");
        // loop@+begin continue: if interaction<>error_stop_mode then return;
        //   clear_for_error_prompt; prompt_input("? ");
        loop {
            region_backward_label!(
                'continue_ <-
                {
                    if $globals.interaction != error_stop_mode {
                        return_nojump!();
                    }
                    clear_for_error_prompt($globals);
                    prompt_input!($globals, strpool_str!("? "));
                    // @.?\relax@>
                    // if last=first then return;
                    if $globals.last == $globals.first {
                        return_nojump!();
                    }
                    /// what the user types
                    let mut c: ASCII_code = $globals.buffer[$globals.first];
                    // if c>="a" then c:=c+"A"-"a"; {convert to uppercase}
                    /// convert to uppercase
                    if c >= ASCII_code_literal!(b'a') && c <= ASCII_code_literal!(b'z') {
                        c = ASCII_code_literal!(c.numeric_value() as u8 + b'A' - b'a');
                    }
                    // @<Interpret code |c| and |return| if done@>;
                    Interpret_code_c_and_return_if_done!($globals, c, 'continue_);
                    // end
                }
                |'continue_|
            );
        }
        use crate::section_0018::ASCII_code;
        use crate::section_0330::clear_for_error_prompt;
    }
}
