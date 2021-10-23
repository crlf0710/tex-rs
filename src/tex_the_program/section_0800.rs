//! ` `
//! Finally, we will reach the end of the alignment, and we can breathe a
//! set so that the columns line up, taking due account of spanned columns.
//! sigh of relief that memory hasn't overflowed. All the unset boxes will now be

//
// @p procedure@?do_assignments; forward;@t\2@>@/
// procedure@?resume_after_display; forward;@t\2@>@/
// procedure@?build_page; forward;@t\2@>@/
// procedure fin_align;
pub(crate) fn fin_align(_globals: &mut TeXGlobals) {
    // var @!p,@!q,@!r,@!s,@!u,@!v: pointer; {registers for the list operations}
    // @!t,@!w:scaled; {width of column}
    // @!o:scaled; {shift offset for unset boxes}
    // @!n:halfword; {matching span amount}
    // @!rule_save:scaled; {temporary storage for |overfull_rule|}
    // @!aux_save:memory_word; {temporary storage for |aux|}
    // begin if cur_group<>align_group then confusion("align1");
    // @:this can't happen align}{\quad align@>
    // unsave; {that |align_group| was for individual entries}
    // if cur_group<>align_group then confusion("align0");
    // unsave; {that |align_group| was for the whole alignment}
    // if nest[nest_ptr-1].mode_field=mmode then o:=display_indent
    //   else o:=0;
    // @<Go through the preamble list, determining the column widths and
    //   changing the alignrecords to dummy unset boxes@>;
    // @<Package the preamble list, to determine the actual tabskip glue amounts,
    //   and let |p| point to this prototype box@>;
    // @<Set the glue in all the unset boxes of the current list@>;
    // flush_node_list(p); pop_alignment;
    // @<Insert the \(c)current list into its environment@>;
    todo!("fin_align");
    // end;@/
}
// @t\4@>@<Declare the procedure called |align_peek|@>
//

use crate::section_0004::TeXGlobals;
