//! @ Here we delete node |p| from the ring, and let |rover| rove around.
//
// @<Allocate entire...@>=
macro_rules! Allocate_entire_node_p_and_goto_found {
    ($globals:expr, $p:expr, $lbl_found:lifetime) => {{
        /// temporary register
        let t: integer;
        // begin rover:=rlink(p); t:=llink(p);
        $globals.rover = rlink!($globals, $p);
        t = llink!($globals, $p) as _;
        // llink(rover):=t; rlink(t):=rover;
        llink!($globals, $globals.rover) = t as _;
        rlink!($globals, t as pointer) = $globals.rover;
        // goto found;
        goto_forward_label!($lbl_found);
        // end
    }}
}
