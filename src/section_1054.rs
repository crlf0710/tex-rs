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
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn its_all_over(globals: &mut TeXGlobals) -> TeXResult<boolean> {
    // label exit;
    // begin if privileged then
    if privileged(globals)? {
        // begin if (page_head=page_tail)and(head=tail)and(dead_cycles=0) then
        if page_head == globals.page_tail && head!(globals) == tail!(globals) && globals.dead_cycles == 0 {
            // begin its_all_over:=true; return;
            return_nojump!(true);
            // end;
        }
        // back_input; {we will try to end again after ejecting residual material}
        /// we will try to end again after ejecting residual material
        back_input(globals);
        // tail_append(new_null_box);
        tail_append!(globals, new_null_box(globals)?);
        // width(tail):=hsize;
        width!(globals, tail!(globals)) = hsize!(globals);
        // tail_append(new_glue(fill_glue));
        tail_append!(globals, new_glue(globals, fill_glue)?);
        // tail_append(new_penalty(-@'10000000000));@/
        tail_append!(globals, new_penalty(globals, -0o10000000000)?);
        // build_page; {append \.{\\hbox to \\hsize\{\}\\vfill\\penalty-'10000000000}}
        /// append `\hbox` to `\hsize{}\vfill\penalty-'10000000000`
        const _ : () = ();
        build_page(globals);
        // end;
    }
    // its_all_over:=false;
    ok_nojump!(false)
    // exit:end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0136::new_null_box;
use crate::section_0153::new_glue;
use crate::section_0158::new_penalty;
use crate::section_0162::fill_glue;
use crate::section_0162::page_head;
use crate::section_0325::back_input;
use crate::section_0994::build_page;
use crate::section_1051::privileged;
