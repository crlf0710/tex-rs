//! @ When the |endv| command at the end of a \<v_j> template comes through the
//! scanner, things really start to happen; and it is the |fin_col| routine
//! that makes them happen. This routine returns |true| if a row as well as a
//! column has been finished.
//
// @p function fin_col:boolean;
#[allow(unused_variables)]
pub(crate) fn fin_col(globals: &mut TeXGlobals) -> TeXResult<boolean> {
    // label exit;
    // var p:pointer; {the alignrecord after the current one}
    /// the alignrecord after the current one
    let p: pointer;
    // @!q,@!r:pointer; {temporary pointers for list manipulation}
    /// temporary pointers for list manipulation
    let (q, _): (pointer, pointer);
    // @!s:pointer; {a new span node}
    // @!u:pointer; {a new unset box}
    // @!w:scaled; {natural width}
    // @!o:glue_ord; {order of infinity}
    // @!n:halfword; {span counter}
    // begin if cur_align=null then confusion("endv");
    if globals.cur_align == null {
        confusion(globals, crate::strpool_str!("endv"))?;
    }
    // q:=link(cur_align);@+if q=null then confusion("endv");
    q = link!(globals, globals.cur_align);
    if q == null {
        confusion(globals, crate::strpool_str!("endv"))?;
    }
    // @:this can't happen endv}{\quad endv@>
    // if align_state<500000 then
    if globals.align_state < 500000 {
        // fatal_error("(interwoven alignment preambles are not allowed)");
        fatal_error(
            globals,
            crate::strpool_str!("(interwoven alignment preambles are not allowed)"),
        )?;
    }
    // @.interwoven alignment preambles...@>
    // p:=link(q);
    p = link!(globals, q);
    // @<If the preamble list has been traversed, check that the row has ended@>;
    crate::section_0792::If_the_preamble_list_has_been_traversed__check_that_the_row_has_ended!(
        globals, p
    );
    // if extra_info(cur_align)<>span_code then
    if extra_info!(globals, globals.cur_align) != span_code {
        // begin unsave; new_save_level(align_group);@/
        unsave(globals)?;
        new_save_level(globals, align_group.into());
        // @<Package an unset box for the current column and record its width@>;
        crate::section_0796::Package_an_unset_box_for_the_current_column_and_record_its_width!(
            globals
        );
        // @<Copy the tabskip glue between columns@>;
        crate::section_0795::Copy_the_tabskip_glue_between_columns!(globals);
        // if extra_info(cur_align)>=cr_code then
        if extra_info!(globals, globals.cur_align) >= cr_code {
            // begin fin_col:=true; return;
            crate::return_nojump!(true);
            // end;
        }
        // init_span(p);
        init_span(globals, p)?;
        // end;
    }
    // align_state:=1000000; @<Get the next non-blank non-call token@>;
    globals.align_state = 1000000;
    crate::section_0406::Get_the_next_non_blank_non_call_token!(globals);
    // cur_align:=p;
    globals.cur_align = p;
    // init_col; fin_col:=false;
    init_col(globals);
    crate::ok_nojump!(false)
    // exit: end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0093::fatal_error;
use crate::section_0095::confusion;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0269::align_group;
use crate::section_0274::new_save_level;
use crate::section_0281::unsave;
use crate::section_0769::extra_info;
use crate::section_0780::cr_code;
use crate::section_0780::span_code;
use crate::section_0787::init_span;
use crate::section_0788::init_col;
