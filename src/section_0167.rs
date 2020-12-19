//! @ Procedure |check_mem| makes sure that the available space lists of
//! |mem| are well formed, and it optionally prints out all locations
//! that are reserved now but were free the last time this procedure was called.
//
// @p @!debug procedure check_mem(@!print_locs : boolean);
#[cfg(feature = "debugging")]
pub(crate) fn check_mem(globals: &mut TeXGlobals, print_locs: boolean) {
    // label done1,done2; {loop exits}
    // var p,@!q:pointer; {current locations of interest in |mem|}
    // @!clobbered:boolean; {is something amiss?}
    // begin for p:=mem_min to lo_mem_max do free[p]:=false; {you can probably
    //   do this faster}
    /// you can probably do this faster
    for p in (mem_min as pointer)..=globals.lo_mem_max {
        globals.free[p] = false;
    }
    // for p:=hi_mem_min to mem_end do free[p]:=false; {ditto}
    /// ditto
    for p in globals.hi_mem_min..=globals.mem_end {
        globals.free[p] = false;
    }
    // @<Check single-word |avail| list@>;
    Check_single_word_avail_list!(globals);
    // @<Check variable-size |avail| list@>;
    Check_variable_size_avail_list!(globals);
    // @<Check flags of unavailable nodes@>;
    Check_flags_of_unavailable_nodes!(globals);
    // if print_locs then @<Print newly busy locations@>;
    if print_locs {
        todo!("Print newly busy locations");
    }
    // for p:=mem_min to lo_mem_max do was_free[p]:=free[p];
    // for p:=hi_mem_min to mem_end do was_free[p]:=free[p];
    //   {|was_free:=free| might be faster}
    /// `was_free:=free` might be faster
    for p in (mem_min as pointer)..=globals.lo_mem_max {
        globals.was_free[p] = globals.free[p];
    }
    for p in globals.hi_mem_min..=globals.mem_end {
        globals.was_free[p] = globals.free[p];
    }
    // was_mem_end:=mem_end; was_lo_max:=lo_mem_max; was_hi_min:=hi_mem_min;
    globals.was_mem_end = globals.mem_end;
    globals.was_lo_max = globals.lo_mem_max;
    globals.was_hi_min = globals.hi_mem_min;
    // end;
    // gubed
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0011::mem_min;
use crate::section_0115::pointer;
