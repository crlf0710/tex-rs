//! @ The function |get_avail| returns a pointer to a new one-word node whose
//! |link| field is null. However, \TeX\ will halt if there is no more room left.
//! @^inner loop@>
//!
//! If the available-space list is empty, i.e., if |avail=null|,
//! we try first to increase |mem_end|. If that cannot be done, i.e., if
//! |mem_end=mem_max|, we try to decrease |hi_mem_min|. If that cannot be
//! done, i.e., if |hi_mem_min=lo_mem_max+1|, we have to quit.
//
// @p function get_avail : pointer; {single-word node allocation}
/// single-word node allocation
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn get_avail(globals: &mut TeXGlobals) -> pointer {
    // var p:pointer; {the new node being got}
    /// the new node being got
    let mut p: pointer;
    // begin p:=avail; {get top location in the |avail| stack}
    /// get top location in the `avail` stack
    {
        p = globals.avail;
    }
    // if p<>null then avail:=link(avail) {and pop it off}
    if p != null {
        /// and pop it off
        {
            globals.avail = link!(globals, globals.avail);
        }
    }
    // else if mem_end<mem_max then {or go into virgin territory}
    else if (globals.mem_end as u32) < mem_max {
        /// or go into virgin territory
        crate::trace_expr!("mem_end = {}", globals.mem_end);
        todo!();
    //   begin incr(mem_end); p:=mem_end;
    //   end
    }
    // else   begin decr(hi_mem_min); p:=hi_mem_min;
    else {
        decr!(globals.hi_mem_min);
        p = globals.hi_mem_min;

        //   if hi_mem_min<=lo_mem_max then
        if globals.hi_mem_min <= globals.lo_mem_max {
            //     begin runaway; {if memory is exhausted, display possible runaway text}
            //     overflow("main memory size",mem_max+1-mem_min);
            //       {quit; all one-word nodes are busy}
            // @:TeX capacity exceeded main memory size}{\quad main memory size@>
            //     end;
            //   end;
            todo!();
        }
    }
    // link(p):=null; {provide an oft-desired initialization of the new node}
    /// provide an oft-desired initialization of the new node
    const _: () = ();
    link!(globals, p) = null;
    // @!stat incr(dyn_used);@+tats@;{maintain statistics}
    crate::region_stat! {
        /// maintain statistics
        incr!(globals.dyn_used);
    }
    // get_avail:=p;
    return p;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0011::mem_max;
use crate::section_0016::decr;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
