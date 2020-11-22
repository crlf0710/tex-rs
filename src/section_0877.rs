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
//
// @<Declare subprocedures for |line_break|@>=
// procedure post_line_break(@!final_widow_penalty:integer);
#[allow(unused_variables)]
pub(crate) fn post_line_break(globals: &mut TeXGlobals, final_widow_penalty: integer) {
    // label done,done1;
    // var q,@!r,@!s:pointer; {temporary registers for list manipulation}
    // @!disc_break:boolean; {was the current break at a discretionary node?}
    // @!post_disc_break:boolean; {and did it have a nonempty post-break part?}
    // @!cur_width:scaled; {width of line number |cur_line|}
    // @!cur_indent:scaled; {left margin of line number |cur_line|}
    // @!t:quarterword; {used for replacement counts in discretionary nodes}
    // @!pen:integer; {use when calculating penalties between lines}
    // @!cur_line: halfword; {the current line number being justified}
    // begin @<Reverse the links of the relevant passive nodes, setting |cur_p| to the
    //   first breakpoint@>;
    // cur_line:=prev_graf+1;
    // repeat @<Justify the line ending at breakpoint |cur_p|, and append it to the
    //   current vertical list, together with associated penalties and other
    //   insertions@>;
    // incr(cur_line); cur_p:=next_break(cur_p);
    // if cur_p<>null then if not post_disc_break then
    //   @<Prune unwanted nodes at the beginning of the next line@>;
    // until cur_p=null;
    // if (cur_line<>best_line)or(link(temp_head)<>null) then
    //   confusion("line breaking");
    // @:this can't happen line breaking}{\quad line breaking@>
    // prev_graf:=best_line-1;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::pascal::integer;
