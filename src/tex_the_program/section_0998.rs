//! ` `

// @<Link node |p| into the current page and |goto done|@>=
pub(crate) macro Link_node_p_into_the_current_page_and_goto_done($globals:expr, $p:expr, $lbl_done:lifetime) {{
    // link(page_tail):=p; page_tail:=p;
    link!($globals, $globals.page_tail) = $p;
    $globals.page_tail = $p;
    // link(contrib_head):=link(p); link(p):=null; goto done
    link!($globals, contrib_head) = link!($globals, $p);
    link!($globals, $p) = null;
    crate::goto_forward_label!($lbl_done);

    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::contrib_head;
}}
