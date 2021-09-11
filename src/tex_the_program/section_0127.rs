//! @ Empirical tests show that the routine in this section performs a
//! node-merging operation about 0.75 times per allocation, on the average,
//! after which it finds that |r>p+1| about 95\pct! of the time.
//
// @<Try to allocate...@>=
pub(crate) macro Try_to_allocate_within_node_p_and_its_physical_successors_and_goto_found_if_allocation_was_possible($globals:expr, $p:expr, $q:expr, $r:expr, $s:expr, $lbl_found:lifetime) {{
    // q:=p+node_size(p); {find the physical successor}
    /// find the physical successor
    const _: () = ();
    $q = $p + node_size!($globals, $p);
    // @^inner loop@>
    // while is_empty(q) do {merge node |p| with node |q|}
    /// merge node `p` with node `q`
    while is_empty!($globals, $q) {
        /// temporary register
        let t: integer;
        // begin t:=rlink(q);
        t = rlink!($globals, $q) as _;
        // if q=rover then rover:=t;
        if $q == $globals.rover {
            $globals.rover = t as _;
        }
        // llink(t):=llink(q); rlink(llink(q)):=t;@/
        llink!($globals, t as pointer) = llink!($globals, $q);
        let llink_q = llink!($globals, $q);
        rlink!($globals, llink_q) = t as _;
        // q:=q+node_size(q);
        $q = $q + node_size!($globals, $q);
        // end;
    }
    // r:=q-s;
    $r = $q as integer - $s;
    // if r>p+1 then @<Allocate from the top of node |p| and |goto found|@>;
    if $r > ($p + 1) as integer {
        crate::section_0128::Allocate_from_the_top_of_node_p_and_goto_found!(
            $globals, $p, $r, $lbl_found
        );
    }
    // if r=p then if rlink(p)<>p then
    //   @<Allocate entire node |p| and |goto found|@>;
    if $r == $p as integer && rlink!($globals, $p) != $p {
        crate::section_0129::Allocate_entire_node_p_and_goto_found!($globals, $p, $lbl_found);
    }
    // node_size(p):=q-p {reset the size in case it grew}
    node_size!($globals, $p) = $q - $p;
    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0124::empty_flag;
    use crate::section_0124::is_empty;
    use crate::section_0124::llink;
    use crate::section_0124::node_size;
    use crate::section_0124::rlink;
}}
