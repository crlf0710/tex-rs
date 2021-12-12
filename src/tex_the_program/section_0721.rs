//! @ Here we save memory space in a common case.
//
// @<Simplify a trivial box@>=
pub(crate) macro Simplify_a_trivial_box($globals:expr, $x:expr) {{
    /// beginning of a list to be boxed
    let q;
    /// temporary pointer
    let r;
    // q:=list_ptr(x);
    q = list_ptr!($globals, $x);
    // if is_char_node(q) then
    if is_char_node!($globals, q) {
        // begin r:=link(q);
        r = link!($globals, q);
        // if r<>null then if link(r)=null then if not is_char_node(r) then
        //  if type(r)=kern_node then {unneeded italic correction}
        if r != null
            && link!($globals, r) == null
            && !is_char_node!($globals, r)
            && r#type!($globals, r) == kern_node
        {
            /// unneeded italic correction
            const _: () = ();
            // begin free_node(r,small_node_size); link(q):=null;
            free_node($globals, r, small_node_size as _);
            link!($globals, q) = null;
            // end;
        }
        // end
    }
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::list_ptr;
    use crate::section_0141::small_node_size;
    use crate::section_0155::kern_node;
}}
