//! ` `

// @<Declare the procedure called |handle_right_brace|@>=
// procedure handle_right_brace;
#[allow(unused_variables)]
pub(crate) fn handle_right_brace(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p,@!q:pointer; {for short-term use}
    // @!d:scaled; {holds |split_max_depth| in |insert_group|}
    // @!f:integer; {holds |floating_penalty| in |insert_group|}
    // begin case cur_group of
    // simple_group: unsave;
    if globals.cur_group == simple_group {
        todo!("unsave");
    }
    // bottom_level: begin print_err("Too many }'s");
    // @.Too many \}'s@>
    else if globals.cur_group == bottom_level {
        // help2("You've closed more groups than you opened.")@/
        // ("Such booboos are generally harmless, so keep going."); error;
        help2!(
            globals,
            strpool_str!("You've closed more groups than you opened."),
            strpool_str!("Such booboos are generally harmless, so keep going.")
        );
        error(globals)?;
    // end;
    }
    // semi_simple_group,math_shift_group,math_left_group: extra_right_brace;
    // @t\4@>@<Cases of |handle_right_brace| where a |right_brace| triggers
    //   a delayed action@>@;
    // othercases confusion("rightbrace")
    else {
        confusion(globals, strpool_str!("rightbrace"));
        // @:this can't happen rightbrace}{\quad rightbrace@>
        // endcases;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_0095::confusion;
use crate::section_0269::*;
