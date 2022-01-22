//! ` `
// @<Copy the templates from node |cur_loop| into node |p|@>=
pub(crate) macro Copy_the_templates_from_node_cur_loop_into_node_p($globals:expr, $p:expr) {{
    /// temporary pointers for list manipulation
    let (mut q, mut r): (pointer, pointer);
    // q:=hold_head; r:=u_part(cur_loop);
    q = hold_head;
    r = u_part!($globals, $globals.cur_loop) as _;
    // while r<>null do
    while r != null {
        // begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
        link!($globals, q) = get_avail($globals);
        q = link!($globals, q);
        info_inner!($globals, q) = info_inner!($globals, r);
        r = link!($globals, r);
        // end;
    }
    // link(q):=null; u_part(p):=link(hold_head);
    link!($globals, q) = null;
    u_part!($globals, $p) = link!($globals, hold_head) as _;
    // q:=hold_head; r:=v_part(cur_loop);
    q = hold_head;
    r = v_part!($globals, $globals.cur_loop) as _;
    // while r<>null do
    while r != null {
        // begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
        link!($globals, q) = get_avail($globals);
        q = link!($globals, q);
        info_inner!($globals, q) = info_inner!($globals, r);
        r = link!($globals, r);
        // end;
    }
    // link(q):=null; v_part(p):=link(hold_head)
    link!($globals, q) = null;
    v_part!($globals, $p) = link!($globals, hold_head) as _;

    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0120::get_avail;
    use crate::section_0162::hold_head;
    use crate::section_0769::u_part;
    use crate::section_0769::v_part;
}}
