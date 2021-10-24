//! ` `
//! Finally, we will reach the end of the alignment, and we can breathe a
//! set so that the columns line up, taking due account of spanned columns.
//! sigh of relief that memory hasn't overflowed. All the unset boxes will now be

//
// @p procedure@?do_assignments; forward;@t\2@>@/
// procedure@?resume_after_display; forward;@t\2@>@/
// procedure@?build_page; forward;@t\2@>@/
// procedure fin_align;
#[allow(unused_variables, unused_assignments, unused_mut)]
pub(crate) fn fin_align(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var @!p,@!q,@!r,@!s,@!u,@!v: pointer; {registers for the list operations}
    /// registers for the list operations
    let (p,);
    // @!t,@!w:scaled; {width of column}
    // @!o:scaled; {shift offset for unset boxes}
    /// shift offset for unset boxes
    let o;
    // @!n:halfword; {matching span amount}
    // @!rule_save:scaled; {temporary storage for |overfull_rule|}
    // @!aux_save:memory_word; {temporary storage for |aux|}
    // begin if cur_group<>align_group then confusion("align1");
    if globals.cur_group != align_group {
        confusion(globals, crate::strpool_str!("align1"))?;
        // @:this can't happen align}{\quad align@>
    }
    // unsave; {that |align_group| was for individual entries}
    /// that `align_group` was for individual entries
    unsave(globals)?;
    // if cur_group<>align_group then confusion("align0");
    if globals.cur_group != align_group {
        confusion(globals, crate::strpool_str!("align0"))?;
    }
    // unsave; {that |align_group| was for the whole alignment}
    /// that `align_group` was for the whole alignment
    unsave(globals)?;
    // if nest[nest_ptr-1].mode_field=mmode then o:=display_indent
    if globals.nest[globals.nest_ptr - 1].mode_field == mmode {
        o = display_indent!(globals);
    } else {
        // else o:=0;
        o = scaled::zero();
    }
    // @<Go through the preamble list, determining the column widths and
    //   changing the alignrecords to dummy unset boxes@>;
    crate::section_0801::Go_through_the_preamble_list__determining_the_column_widths_and_changing_the_alignrecords_to_dummy_unset_boxes!(
        globals
    );
    // @<Package the preamble list, to determine the actual tabskip glue amounts,
    //   and let |p| point to this prototype box@>;
    crate::section_0804::Package_the_preamble_list__to_determine_the_actual_tabskip_glue_amounts__and_let_p_point_to_this_prototype_box!(
        globals, p
    );
    // @<Set the glue in all the unset boxes of the current list@>;
    crate::section_0805::Set_the_glue_in_all_the_unset_boxes_of_the_current_list!(globals, p, o);
    // flush_node_list(p); pop_alignment;
    flush_node_list(globals, p)?;
    pop_alignment(globals);
    // @<Insert the \(c)current list into its environment@>;
    crate::section_0812::Insert_the_current_list_into_its_environment!(globals);
    // end;@/
    crate::ok_nojump!()
}
// @t\4@>@<Declare the procedure called |align_peek|@>
//

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0101::scaled;
use crate::section_0211::mmode;
use crate::section_0247::display_indent;
use crate::section_0269::align_group;
use crate::section_0281::unsave;
use crate::section_0772::pop_alignment;
use crate::tex_the_program::section_0202::flush_node_list;
