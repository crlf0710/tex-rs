//! ` `
//! @<Expand a nonmacro@>=
macro_rules! Expand_a_nonmacro {
    ($globals:expr) => {
        // begin if tracing_commands>1 then show_cur_cmd_chr;
        // case cur_cmd of
        // top_bot_mark:@<Insert the \(a)appropriate mark text into the scanner@>;
        // expand_after:@<Expand the token after the next token@>;
        // no_expand:@<Suppress expansion of the next token@>;
        // cs_name:@<Manufacture a control sequence name@>;
        // convert:conv_toks; {this procedure is discussed in Part 27 below}
        // the:ins_the_toks; {this procedure is discussed in Part 27 below}
        // if_test:conditional; {this procedure is discussed in Part 28 below}
        // fi_or_else:@<Terminate the current conditional and skip to \.{\\fi}@>;
        // input:@<Initiate or terminate input from a file@>;
        // othercases @<Complain about an undefined macro@>
        Complain_about_an_undefined_macro!($globals);
        // endcases;
        // end
    }
}