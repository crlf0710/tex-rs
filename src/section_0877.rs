//! @ The total number of lines that will be set by |post_line_break|
//! is |best_line-prev_graf-1|. The last breakpoint is specified by
//! |break_node(best_bet)|, and this passive node points to the other breakpoints
//! via the |prev_break| links. The finishing-up phase starts by linking the
//! relevant passive nodes in forward order, changing |prev_break| to
//! |next_break|. (The |next_break| fields actually reside in the same memory
//! space as the |prev_break| fields did, but we give them a new name because
//! of their new significance.) Then the lines are justified, one by one.
//
// @d next_break==prev_break {new name for |prev_break| after links are reversed}
macro_rules! next_break {
    ($globals:expr, $p:expr) => {
        prev_break!($globals, $p)
    }
}

// @<Declare subprocedures for |line_break|@>=
// procedure post_line_break(@!final_widow_penalty:integer);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn post_line_break(globals: &mut TeXGlobals, final_widow_penalty: integer) -> TeXResult<()> {
    // label done,done1;
    // var q,@!r,@!s:pointer; {temporary registers for list manipulation}
    // @!disc_break:boolean; {was the current break at a discretionary node?}
    /// was the current break at a discretionary node?
    let disc_break: boolean;
    // @!post_disc_break:boolean; {and did it have a nonempty post-break part?}
    /// and did it have a nonempty post-break part?
    let post_disc_break: boolean;
    // @!cur_width:scaled; {width of line number |cur_line|}
    // @!cur_indent:scaled; {left margin of line number |cur_line|}
    // @!t:quarterword; {used for replacement counts in discretionary nodes}
    // @!pen:integer; {use when calculating penalties between lines}
    // @!cur_line: halfword; {the current line number being justified}
    /// the current line number being justified
    let cur_line: halfword;
    // begin @<Reverse the links of the relevant passive nodes, setting |cur_p| to the
    //   first breakpoint@>;
    Reverse_the_links_of_the_relevant_passive_nodes__setting_cur_p_to_the_first_breakpoint!
        (globals);
    // cur_line:=prev_graf+1;
    cur_line = (prev_graf!(globals) + 1) as _;
    // repeat @<Justify the line ending at breakpoint |cur_p|, and append it to the
    //   current vertical list, together with associated penalties and other
    //   insertions@>;
    loop {
        Justify_the_line_ending_at_breakpoint_cur_p__and_append_it_to_the_current_vertical_list__together_with_associated_penalties_and_other_insertions!
            (globals, cur_line, disc_break, post_disc_break);
        // incr(cur_line); cur_p:=next_break(cur_p);
        incr!(cur_line);
        globals.cur_p = next_break!(globals, globals.cur_p);
        // if cur_p<>null then if not post_disc_break then
        if globals.cur_p != null && !post_disc_break {
            // @<Prune unwanted nodes at the beginning of the next line@>;
            todo!("prune unwanted");
        }
        // until cur_p=null;
        if globals.cur_p == null {
            break;
        }
    }
    // if (cur_line<>best_line)or(link(temp_head)<>null) then
    if cur_line != globals.best_line || link!(globals, temp_head) != null {
        // confusion("line breaking");
        confusion(globals, strpool_str!("line breaking"))?;
    }
    // @:this can't happen line breaking}{\quad line breaking@>
    // prev_graf:=best_line-1;
    prev_graf!(globals) = globals.best_line as integer - 1;
    // end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0113::halfword;
use crate::section_0115::null;
use crate::section_0162::temp_head;
