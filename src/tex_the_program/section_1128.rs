//! ` `
// @<Express consternation...@>=
pub(crate) macro Express_consternation_over_the_fact_that_no_alignment_is_in_progress($globals:expr) {{
    // begin print_err("Misplaced "); print_cmd_chr(cur_cmd,cur_chr);
    print_err!($globals, crate::strpool_str!("Misplaced "));
    print_cmd_chr($globals, $globals.cur_cmd, $globals.cur_chr);
    // @.Misplaced \&@>
    // @.Misplaced \\span@>
    // @.Misplaced \\cr@>
    // if cur_tok=tab_token+"&" then
    if $globals.cur_tok.get() == tab_token + b'&' as cur_tok_repr {
        // begin help6("I can't figure out why you would want to use a tab mark")@/
        // ("here. If you just want an ampersand, the remedy is")@/
        // ("simple: Just type `I\&' now. But if some right brace")@/
        // ("up above has ended a previous alignment prematurely,")@/
        // ("you're probably due for more error messages, and you")@/
        // ("might try typing `S' now just to see what is salvageable.");
        help6!(
            $globals,
            crate::strpool_str!("I can't figure out why you would want to use a tab mark"),
            crate::strpool_str!("here. If you just want an ampersand, the remedy is"),
            crate::strpool_str!("simple: Just type `I\\&' now. But if some right brace"),
            crate::strpool_str!("up above has ended a previous alignment prematurely,"),
            crate::strpool_str!("you're probably due for more error messages, and you"),
            crate::strpool_str!("might try typing `S' now just to see what is salvageable.")
        );
        // end
    }
    // else  begin help5("I can't figure out why you would want to use a tab mark")@/
    else {
        // ("or \cr or \span just now. If something like a right brace")@/
        // ("up above has ended a previous alignment prematurely,")@/
        // ("you're probably due for more error messages, and you")@/
        // ("might try typing `S' now just to see what is salvageable.");
        help5!(
            $globals,
            crate::strpool_str!("I can't figure out why you would want to use a tab mark"),
            crate::strpool_str!("or \\cr or \\span just now. If something like a right brace"),
            crate::strpool_str!("up above has ended a previous alignment prematurely,"),
            crate::strpool_str!("you're probably due for more error messages, and you"),
            crate::strpool_str!("might try typing `S' now just to see what is salvageable.")
        );
        // end;
    }
    // error;
    error($globals)?;
    // end
    use crate::section_0073::print_err;
    use crate::section_0079::help5;
    use crate::section_0079::help6;
    use crate::section_0082::error;
    use crate::section_0289::tab_token;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0298::print_cmd_chr;
}}
