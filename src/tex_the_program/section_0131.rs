//! @ Just before \.{INITEX} writes out the memory, it sorts the doubly linked
//! available space list. The list is probably very short at such times, so a
//! simple insertion sort is used. The smallest available location will be
//! pointed to by |rover|, the next-smallest by |rlink(rover)|, etc.
//
// @p @!init procedure sort_avail; {sorts the available variable-size nodes
//   by location}
/// sorts the available variable-size nodes by location
#[cfg(feature = "initex")]
pub(crate) fn sort_avail(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p,@!q,@!r: pointer; {indices into |mem|}
    /// indices into `mem`
    let mut p: pointer;
    // @!old_rover:pointer; {initial |rover| setting}
    /// initial `rover` setting
    let old_rover: pointer;
    // begin p:=get_node(@'10000000000); {merge adjacent free areas}
    /// merge adjacent free areas
    const _: () = ();
    let _ = get_node(globals, 0o10000000000)?;
    // p:=rlink(rover); rlink(rover):=max_halfword; old_rover:=rover;
    p = rlink!(globals, globals.rover);
    rlink!(globals, globals.rover) = max_halfword;
    old_rover = globals.rover;
    // while p<>old_rover do @<Sort \(p)|p| into the list starting at |rover|
    //   and advance |p| to |rlink(p)|@>;
    while p != old_rover {
        crate::section_0132::Sort_p_into_the_list_starting_at_rover_and_advance_p_to_rlink_p!(
            globals, p
        );
    }
    // p:=rover;
    p = globals.rover;
    // while rlink(p)<>max_halfword do
    while rlink!(globals, p) != max_halfword {
        // begin llink(rlink(p)):=p; p:=rlink(p);
        let rlink_p = rlink!(globals, p);
        llink!(globals, rlink_p) = p;
        p = rlink_p;
        // end;
    }
    // rlink(p):=rover; llink(rover):=p;
    rlink!(globals, p) = globals.rover;
    llink!(globals, globals.rover) = p;
    // end;
    // tini
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0110::max_halfword;
use crate::section_0115::pointer;
use crate::section_0124::llink;
use crate::section_0124::rlink;
use crate::section_0125::get_node;
