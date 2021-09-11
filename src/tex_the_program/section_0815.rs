//! @ Since |line_break| is a rather lengthy procedure---sort of a small world unto
//! itself---we must build it up little by little, somewhat more cautiously
//! than we have done with the simpler procedures of \TeX. Here is the
//! general outline.

pub(crate) macro Get_ready_to_start_line_breaking($globals:expr) {{
    crate::section_0816::Get_ready_to_start_line_breaking_0816!($globals);
    crate::section_0827::Get_ready_to_start_line_breaking_0827!($globals);
    crate::section_0834::Get_ready_to_start_line_breaking_0834!($globals);
    crate::section_0848::Get_ready_to_start_line_breaking_0848!($globals);
}}

// @p@t\4@>@<Declare subprocedures for |line_break|@>
// procedure line_break(@!final_widow_penalty:integer);
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
#[allow(unused_variables)]
pub(crate) fn line_break(globals: &mut TeXGlobals, final_widow_penalty: integer) -> TeXResult<()> {
    // label done,done1,done2,done3,done4,done5,continue;
    // var @<Local variables for line breaking@>@;
    // begin pack_begin_line:=mode_line; {this is for over/underfull box messages}
    /// this is for over/underfull box messages
    const _: () = ();
    globals.pack_begin_line = mode_line!(globals);
    // @<Get ready to start line breaking@>;
    Get_ready_to_start_line_breaking!(globals);
    // @<Find optimal breakpoints@>;
    crate::section_0863::Find_optimal_breakpoints!(globals);
    // @<Break the paragraph at the chosen breakpoints, justify the resulting lines
    // to the correct widths, and append them to the current vertical list@>;
    crate::section_0876::Break_the_paragraph_at_the_chosen_breakpoints__justify_the_resulting_lines_to_the_correct_widths__and_append_them_to_the_current_vertical_list!(
        globals,
        final_widow_penalty
    );
    // @<Clean up the memory by removing the break nodes@>;
    crate::section_0865::Clean_up_the_memory_by_removing_the_break_nodes!(globals);
    // pack_begin_line:=0;
    globals.pack_begin_line = 0;
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0213::mode_line;
