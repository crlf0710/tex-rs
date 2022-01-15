//! @ Although node |q| is not necessarily the immediate predecessor of node |p|,
//! it always points to some node in the list preceding |p|. Thus, we can delete
//! nodes by moving |q| when necessary. The algorithm takes linear time, and the
//! extra computation does not intrude on the inner loop unless it is necessary
//! to make a deletion.
//! @^inner loop@>
//
// @<Transfer node |p| to the adjustment list@>=
pub(crate) macro Transfer_node_p_to_the_adjustment_list($globals:expr, $p:expr, $q:expr) {{
    // begin while link(q)<>p do q:=link(q);
    while link!($globals, $q) != $p {
        $q = link!($globals, $q);
    }
    // if type(p)=adjust_node then
    if r#type!($globals, $p) == adjust_node {
        // begin link(adjust_tail):=adjust_ptr(p);
        link!($globals, $globals.adjust_tail) = adjust_ptr!($globals, $p) as pointer;
        // while link(adjust_tail)<>null do adjust_tail:=link(adjust_tail);
        while link!($globals, $globals.adjust_tail) != null {
            $globals.adjust_tail = link!($globals, $globals.adjust_tail);
        }
        // p:=link(p); free_node(link(q),small_node_size);
        $p = link!($globals, $p);
        free_node($globals, link!($globals, $q), small_node_size as _);
        // end
    }
    // else  begin link(adjust_tail):=p; adjust_tail:=p; p:=link(p);
    else {
        link!($globals, $globals.adjust_tail) = $p;
        $globals.adjust_tail = $p;
        $p = link!($globals, $p);
        // end;
    }
    // link(q):=p; p:=q;
    link!($globals, $q) = $p;
    $p = $q;
    // end
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0141::small_node_size;
    use crate::section_0142::adjust_node;
    use crate::section_0142::adjust_ptr;
}}
