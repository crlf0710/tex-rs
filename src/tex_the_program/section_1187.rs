//! ` `
// @<Replace the tail...@>=
pub(crate) macro Replace_the_tail_of_the_list_by_p($globals:expr, $p:expr) {{
    /// for short-term use
    let mut q;
    // begin q:=head; while link(q)<>tail do q:=link(q);
    q = head!($globals);
    while link!($globals, q) != tail!($globals) {
        q = link!($globals, q);
    }
    // link(q):=p; free_node(tail,noad_size); tail:=p;
    link!($globals, q) = $p;
    free_node($globals, tail!($globals), noad_size as _);
    tail!($globals) = $p;
    // end
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0213::head;
    use crate::section_0213::tail;
    use crate::section_0681::noad_size;
}}
