//! ` `
//!
//! The following |while| loop is guaranteed to
//! terminate, since the list that starts at
//! |rover| ends with |max_halfword| during the sorting procedure.
//
// @<Sort \(p)|p|...@>=
pub(crate) macro Sort_p_into_the_list_starting_at_rover_and_advance_p_to_rlink_p($globals:expr, $p:expr) {{
    /// indices into `mem`
    let (mut q, r);
    // if p<rover then
    if $p < $globals.rover {
        // begin q:=p; p:=rlink(q); rlink(q):=rover; rover:=q;
        // end
        todo!("p < rover")
    }
    // else  begin q:=rover;
    else {
        q = $globals.rover;
        // while rlink(q)<p do q:=rlink(q);
        while rlink!($globals, q) < $p {
            q = rlink!($globals, q);
        }
        // r:=rlink(p); rlink(p):=rlink(q); rlink(q):=p; p:=r;
        r = rlink!($globals, $p);
        rlink!($globals, $p) = rlink!($globals, q);
        rlink!($globals, q) = $p;
        $p = r;
        // end
    }
    use crate::section_0124::rlink;
}}
