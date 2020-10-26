//! @ We don't want to leave |main_control| immediately when a |stop| command
//! is sensed, because it may be necessary to invoke an \.{\\output} routine
//! several times before things really grind to a halt. (The output routine
//! might even say `\.{\\gdef\\end\{...\}}', to prolong the life of the job.)
//! Therefore |its_all_over| is |true| only when the current page
//! and contribution list are empty, and when the last output was not a
//! ``dead cycle.''
//
// @<Declare act...@>=
// function its_all_over:boolean; {do this when \.{\\end} or \.{\\dump} occurs}
/// do this when `\end` or `\dump` occurs
#[allow(unused_variables)]
pub(crate) fn its_all_over(globals: &mut TeXGlobals) -> boolean {
    // label exit;
    // begin if privileged then
    //   begin if (page_head=page_tail)and(head=tail)and(dead_cycles=0) then
    //     begin its_all_over:=true; return;
    return true;
    //     end;
    //   back_input; {we will try to end again after ejecting residual material}
    //   tail_append(new_null_box);
    //   width(tail):=hsize;
    //   tail_append(new_glue(fill_glue));
    //   tail_append(new_penalty(-@'10000000000));@/
    //   build_page; {append \.{\\hbox to \\hsize\{\}\\vfill\\penalty-'10000000000}}
    //   end;
    // its_all_over:=false;
    // exit:end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;