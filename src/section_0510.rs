//! @ The processing of conditionals is complete except for the following
//! code, which is actually part of |expand|. It comes into play when
//! \.{\\or}, \.{\\else}, or \.{\\fi} is scanned.
//
// @<Terminate the current conditional and skip to \.{\\fi}@>=
macro_rules! Terminate_the_current_conditional_and_skip_to_fi {
    ($globals:expr) => {{
        // if cur_chr>if_limit then
        if $globals.cur_chr.get() > $globals.if_limit.get() as _ {
            //   if if_limit=if_code then insert_relax {condition not yet evaluated}
            if $globals.if_limit == if_code {
                /// condition not yet evaluated
                insert_relax($globals);
            }
            //   else  begin print_err("Extra "); print_cmd_chr(fi_or_else,cur_chr);
            else {
                print_err!($globals, strpool_str!("Extra "));
                print_cmd_chr($globals, fi_or_else, $globals.cur_chr);
                // @.Extra \\or@>
                // @.Extra \\else@>
                // @.Extra \\fi@>
                // help1("I'm ignoring this; it doesn't match any \if.");
                help1!($globals, strpool_str!("I'm ignoring this; it doesn't match any \\if."));
                // error;
                error($globals)?;
                // end
            }
        }
        // else  begin while cur_chr<>fi_code do pass_text; {skip to \.{\\fi}}
        else {
            /// skip to `\fi`
            while $globals.cur_chr.get() != fi_code as _ {
                pass_text($globals)?;
            }
            // @<Pop the condition stack@>;
            Pop_the_condition_stack!($globals);
            // end
        }
        use crate::section_0082::error;
        use crate::section_0298::print_cmd_chr;
        use crate::section_0379::insert_relax;
        use crate::section_0489::if_code;
        use crate::section_0489::fi_code;
        use crate::section_0494::pass_text;
    }}
}