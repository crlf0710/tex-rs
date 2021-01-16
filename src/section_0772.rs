//! @ Alignment stack maintenance is handled by a pair of trivial routines
//! called |push_alignment| and |pop_alignment|.
//
// @p procedure push_alignment;
pub(crate) fn push_alignment(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p:pointer; {the new alignment stack node}
    /// the new alignment stack node
    let p: pointer;
    // begin p:=get_node(align_stack_node_size);
    p = get_node(globals, align_stack_node_size.into())?;
    // link(p):=align_ptr; info(p):=cur_align;
    link!(globals, p) = globals.align_ptr;
    info_inner!(globals, p) = globals.cur_align;
    // llink(p):=preamble; rlink(p):=cur_span;
    llink!(globals, p) = preamble!(globals);
    rlink!(globals, p) = globals.cur_span;
    // mem[p+2].int:=cur_loop; mem[p+3].int:=align_state;
    globals.mem[p + 2][MEMORY_WORD_INT] = globals.cur_loop.into();
    globals.mem[p + 3][MEMORY_WORD_INT] = globals.align_state;
    // info(p+4):=cur_head; link(p+4):=cur_tail;
    // align_ptr:=p;
    globals.align_ptr = p;
    // cur_head:=get_avail;
    globals.cur_head = get_avail(globals);
    // end;
    ok_nojump!()
}
// @#
// procedure pop_alignment;
pub(crate) fn pop_alignment(_globals: &mut TeXGlobals) {
    todo!("pop_alignment");
    // var p:pointer; {the top alignment stack node}
    // begin free_avail(cur_head);
    // p:=align_ptr;
    // cur_tail:=link(p+4); cur_head:=info(p+4);
    // align_state:=mem[p+3].int; cur_loop:=mem[p+2].int;
    // cur_span:=rlink(p); preamble:=llink(p);
    // cur_align:=info(p); align_ptr:=link(p);
    // free_node(p,align_stack_node_size);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::MEMORY_WORD_INT;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0125::get_node;
use crate::section_0770::align_stack_node_size;
