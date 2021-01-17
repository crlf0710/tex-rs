//! ` `

macro_rules! Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action {
    ($globals:expr) => {
        if false {
            unreachable!();
        } else if Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1085!($globals) {
            // already processed
            true
        } else {
            false
        }
    };
}

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
        unsave(globals)?;
    }
    // bottom_level: begin print_err("Too many }'s");
    // @.Too many \}'s@>
    else if globals.cur_group == bottom_level {
        print_err!(globals, strpool_str!("Too many }'s"));
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
    else if globals.cur_group == semi_simple_group || globals.cur_group == math_shift_group ||
        globals.cur_group == math_left_group {
        todo!("extra right brace");
    }
    // @t\4@>@<Cases of |handle_right_brace| where a |right_brace| triggers
    //   a delayed action@>@;
    else if Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action!(globals) {
        /// already processed
        const _: () = ();
    }
    // othercases confusion("rightbrace")
    else {
        trace_error_expr!("cur_group = {}", globals.cur_group.get());
        confusion(globals, strpool_str!("rightbrace"))?;
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
use crate::section_0281::unsave;
