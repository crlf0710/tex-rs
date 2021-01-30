//! @ When the |endv| command at the end of a \<v_j> template comes through the
//! scanner, things really start to happen; and it is the |fin_col| routine
//! that makes them happen. This routine returns |true| if a row as well as a
//! column has been finished.
//
// @p function fin_col:boolean;
#[allow(unused_variables)]
pub(crate) fn fin_col(globals: &mut TeXGlobals) -> boolean {
    todo!();
    // label exit;
    // var p:pointer; {the alignrecord after the current one}
    // @!q,@!r:pointer; {temporary pointers for list manipulation}
    // @!s:pointer; {a new span node}
    // @!u:pointer; {a new unset box}
    // @!w:scaled; {natural width}
    // @!o:glue_ord; {order of infinity}
    // @!n:halfword; {span counter}
    // begin if cur_align=null then confusion("endv");
    // q:=link(cur_align);@+if q=null then confusion("endv");
    // @:this can't happen endv}{\quad endv@>
    // if align_state<500000 then
    //   fatal_error("(interwoven alignment preambles are not allowed)");
    // @.interwoven alignment preambles...@>
    // p:=link(q);
    // @<If the preamble list has been traversed, check that the row has ended@>;
    // if extra_info(cur_align)<>span_code then
    //   begin unsave; new_save_level(align_group);@/
    //   @<Package an unset box for the current column and record its width@>;
    //   @<Copy the tabskip glue between columns@>;
    //   if extra_info(cur_align)>=cr_code then
    //     begin fin_col:=true; return;
    //     end;
    //   init_span(p);
    //   end;
    // align_state:=1000000; @<Get the next non-blank non-call token@>;
    // cur_align:=p;
    // init_col; fin_col:=false;
    // exit: end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
