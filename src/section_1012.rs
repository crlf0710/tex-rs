//! @ When the page builder has looked at as much material as could appear before
//! the next page break, it makes its decision. The break that gave minimum
//! badness will be used to put a completed ``page'' into box 255, with insertions
//! appended to their other boxes.
//!
//! We also set the values of |top_mark|, |first_mark|, and |bot_mark|. The
//! program uses the fact that |bot_mark<>null| implies |first_mark<>null|;
//! it also knows that |bot_mark=null| implies |top_mark=first_mark=null|.
//!
//! The |fire_up| subroutine prepares to output the current page at the best
//! place; then it fires up the user's output routine, if there is one,
//! or it simply ships out the page. There is one parameter, |c|, which represents
//! the node that was being contributed to the page when the decision to
//! force an output was made.
//
// @<Declare the procedure called |fire_up|@>=
// procedure fire_up(@!c:pointer);
#[allow(unused_variables)]
pub(crate) fn fire_up(globals: &mut TeXGlobals, c: pointer) -> TeXResult<()> {
    // label exit;
    // var p,@!q,@!r,@!s:pointer; {nodes being examined and/or changed}
    // @!prev_p:pointer; {predecessor of |p|}
    // @!n:min_quarterword..255; {insertion box number}
    // @!wait:boolean; {should the present insertion be held over?}
    // @!save_vbadness:integer; {saved value of |vbadness|}
    // @!save_vfuzz: scaled; {saved value of |vfuzz|}
    // @!save_split_top_skip: pointer; {saved value of |split_top_skip|}
    // begin @<Set the value of |output_penalty|@>;
    Set_the_value_of_output_penalty!(globals);
    // if bot_mark<>null then
    if bot_mark!(globals) != null {
        // begin if top_mark<>null then delete_token_ref(top_mark);
        // top_mark:=bot_mark; add_token_ref(top_mark);
        // delete_token_ref(first_mark); first_mark:=null;
        // end;
        todo!("bot_mark");
    }
    // @<Put the \(o)optimal current page into box 255, update |first_mark| and
    //   |bot_mark|, append insertions to their boxes, and put the
    //   remaining nodes back on the contribution list@>;
    Put_the_optimal_current_page_into_box_255__update_first_mark_and_bot_mark__append_insertions_to_their_boxes__and_put_the_remaining_nodes_back_on_the_contribution_list!
        (globals, c);
    // if (top_mark<>null)and(first_mark=null) then
    if top_mark!(globals) != null && first_mark!(globals) == null {
        // begin first_mark:=top_mark; add_token_ref(top_mark);
        first_mark!(globals) = top_mark!(globals);
        add_token_ref!(globals, top_mark!(globals));
        // end;
    }
    // if output_routine<>null then
    if output_routine!(globals) != null {
        // if dead_cycles>=max_dead_cycles then
        if globals.dead_cycles >= max_dead_cycles!(globals) {
            // @<Explain that too many dead cycles have occurred in a row@>
            todo!("explain");
        }
        // else @<Fire up the user's output routine and |return|@>;
        else {
            Fire_up_the_user_s_output_routine_and_return!(globals);
        }
    }
    else {
        // @<Perform the default output routine@>;
        todo!("perform default output");
    }
    // exit:end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0115::null;
