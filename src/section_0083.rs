//! ` `
// @<Get user's advice...@>=
macro_rules! Get_user_s_advice_and_return {
    ($globals:expr) => {
        // loop@+begin continue: clear_for_error_prompt; prompt_input("? ");
        loop {
            clear_for_error_prompt($globals);
            prompt_input!($globals, strpool_str!("? "));
            // @.?\relax@>
            // if last=first then return;
            if $globals.last == $globals.first {
                return_nojump!();
            }
            // c:=buffer[first];
            // if c>="a" then c:=c+"A"-"a"; {convert to uppercase}
            // @<Interpret code |c| and |return| if done@>;
            todo!("interpret");
            // end
        }
        use crate::section_0330::clear_for_error_prompt;
    }
}
