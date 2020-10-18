//! @ Procedure |check_mem| makes sure that the available space lists of
//! |mem| are well formed, and it optionally prints out all locations
//! that are reserved now but were free the last time this procedure was called.
//
// @p @!debug procedure check_mem(@!print_locs : boolean);
#[cfg(feature = "debugging")]
#[allow(unused_variables)]
pub(crate) fn check_mem(globals: &mut TeXGlobals, print_locs: boolean) {
    // label done1,done2; {loop exits}
    // var p,@!q:pointer; {current locations of interest in |mem|}
    // @!clobbered:boolean; {is something amiss?}
    // begin for p:=mem_min to lo_mem_max do free[p]:=false; {you can probably
    //   do this faster}
    // for p:=hi_mem_min to mem_end do free[p]:=false; {ditto}
    // @<Check single-word |avail| list@>;
    // @<Check variable-size |avail| list@>;
    // @<Check flags of unavailable nodes@>;
    // if print_locs then @<Print newly busy locations@>;
    // for p:=mem_min to lo_mem_max do was_free[p]:=free[p];
    // for p:=hi_mem_min to mem_end do was_free[p]:=free[p];
    //   {|was_free:=free| might be faster}
    // was_mem_end:=mem_end; was_lo_max:=lo_mem_max; was_hi_min:=hi_mem_min;
    // end;
    // gubed
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
