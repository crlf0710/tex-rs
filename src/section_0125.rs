//! @ A call to |get_node| with argument |s| returns a pointer to a new node
//! of size~|s|, which must be 2~or more. The |link| field of the first word
//! of this new node is set to null. An overflow stop occurs if no suitable
//! space exists.
//!
//! If |get_node| is called with $s=2^{30}$, it simply merges adjacent free
//! areas and returns the value |max_halfword|.
//
// @p function get_node(@!s:integer):pointer; {variable-size node allocation}
/// variable-size node allocation
#[allow(unused_variables)]
pub(crate) fn get_node(globals: &mut TeXGlobals, s: integer) -> TeXResult<pointer> {
    // label found,exit,restart;
    // var p:pointer; {the node currently under inspection}
    /// the node currently under inspection
    let mut p: pointer;
    // @!q:pointer; {the node physically after node |p|}
    /// the node physically after node `p`
    let mut q: pointer;
    // @!r:integer; {the newly allocated node, or a candidate for this honor}
    /// the newly allocated node, or a candidate for this honor
    let mut r: integer;
    // @!t:integer; {temporary register}
    const _ : () = ();

    // begin restart: p:=rover; {start at some free node in the ring}
    region_backward_label!(
    'restart <-
    {
    /// start at some free node in the ring
    const _ : () = ();
    p = globals.rover;
    region_forward_label!(
    |'found|
    {
    // repeat @<Try to allocate within node |p| and its physical successors,
    //   and |goto found| if allocation was possible@>;
    loop {
        Try_to_allocate_within_node_p_and_its_physical_successors_and_goto_found_if_allocation_was_possible!
            (globals, p, q, r, s, 'found);
        // @^inner loop@>
        // p:=rlink(p); {move to the next node in the ring}
        /// move to the next node in the ring
        const _: () = ();
        p = rlink!(globals, p);
        // until p=rover; {repeat until the whole list has been traversed}
        /// repeat until the whole list has been traversed
        if p == globals.rover {
            break;
        }
    }
    // if s=@'10000000000 then
    if s == 0o10000000000 {
        // begin get_node:=max_halfword; return;
        return_nojump!(max_halfword);
        // end;
    }
    // if lo_mem_max+2<hi_mem_min then if lo_mem_max+2<=mem_bot+max_halfword then
    if globals.lo_mem_max + 2 < globals.hi_mem_min && globals.lo_mem_max + 2 <= mem_bot + max_halfword {
        // @<Grow more variable-size memory and |goto restart|@>;
        Grow_more_variable_size_memory_and_goto_restart!(globals, p, q, 'restart);
    }
    // overflow("main memory size",mem_max+1-mem_min);
    //   {sorry, nothing satisfactory is left}
    /// sorry, nothing satisfactory is left
    overflow(globals, strpool_str!("main memory size"), (mem_max + 1 - mem_min) as _)?;
    // @:TeX capacity exceeded main memory size}{\quad main memory size@>
    // found: link(r):=null; {this node is now nonempty}
    }
    'found <-
    );
    /// this node is now nonempty
    const _: () = ();
    link!(globals, r as pointer) = null;
    }
    |'restart|
    );
    // @!stat var_used:=var_used+s; {maintain usage statistics}
    /// maintain usage statistics
    region_stat! {
        todo!();
    }
    // tats@;@/
    // get_node:=r;
    return_nojump!(r as _);
    // exit:end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0011::mem_max;
use crate::section_0011::mem_min;
use crate::section_0012::mem_bot;
use crate::section_0081::TeXResult;
use crate::section_0094::overflow;
use crate::section_0110::max_halfword;
use crate::section_0115::pointer;
use crate::section_0115::null;
