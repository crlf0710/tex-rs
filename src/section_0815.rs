//! @ Since |line_break| is a rather lengthy procedure---sort of a small world unto
//! itself---we must build it up little by little, somewhat more cautiously
//! than we have done with the simpler procedures of \TeX. Here is the
//! general outline.
//
// @p@t\4@>@<Declare subprocedures for |line_break|@>
// procedure line_break(@!final_widow_penalty:integer);
#[allow(unused_variables)]
pub(crate) fn line_break(globals: &mut TeXGlobals, final_widow_penalty: integer) {
    // label done,done1,done2,done3,done4,done5,continue;
    // var @<Local variables for line breaking@>@;
    // begin pack_begin_line:=mode_line; {this is for over/underfull box messages}
    // @<Get ready to start line breaking@>;
    // @<Find optimal breakpoints@>;
    // @<Break the paragraph at the chosen breakpoints, justify the resulting lines
    // to the correct widths, and append them to the current vertical list@>;
    // @<Clean up the memory by removing the break nodes@>;
    // pack_begin_line:=0;
    // end;
    todo!("line_break");
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
