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
    info_inner!(globals, p + 4) = globals.cur_head;
    link!(globals, p + 4) = globals.cur_tail;
    // align_ptr:=p;
    globals.align_ptr = p;
    // cur_head:=get_avail;
    globals.cur_head = get_avail(globals);
    // end;
    crate::ok_nojump!()
}
// @#
// procedure pop_alignment;
pub(crate) fn pop_alignment(globals: &mut TeXGlobals) {
    // var p:pointer; {the top alignment stack node}
    /// the top alignment stack node
    let p;
    // begin free_avail(cur_head);
    free_avail!(globals, globals.cur_head);
    // p:=align_ptr;
    p = globals.align_ptr;
    // cur_tail:=link(p+4); cur_head:=info(p+4);
    globals.cur_tail = link!(globals, p + 4);
    globals.cur_head = info_inner!(globals, p + 4);
    // align_state:=mem[p+3].int; cur_loop:=mem[p+2].int;
    globals.align_state = globals.mem[p + 3][MEMORY_WORD_INT];
    globals.cur_loop = globals.mem[p + 2][MEMORY_WORD_INT] as _;
    // cur_span:=rlink(p); preamble:=llink(p);
    globals.cur_span = rlink!(globals, p);
    preamble!(globals) = llink!(globals, p);
    // cur_align:=info(p); align_ptr:=link(p);
    globals.cur_align = info_inner!(globals, p);
    globals.align_ptr = link!(globals, p);
    // free_node(p,align_stack_node_size);
    free_node(globals, p, align_stack_node_size as _);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::MEMORY_WORD_INT;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0120::get_avail;
use crate::section_0121::free_avail;
use crate::section_0124::llink;
use crate::section_0124::rlink;
use crate::section_0125::get_node;
use crate::section_0130::free_node;
use crate::section_0770::align_stack_node_size;
use crate::section_0770::preamble;
