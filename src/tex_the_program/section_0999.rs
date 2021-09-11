//! ` `
// @<Recycle node |p|@>=
pub(crate) macro Recycle_node_p($globals:expr, $p:expr) {{
    // link(contrib_head):=link(p); link(p):=null; flush_node_list(p)
    link!($globals, contrib_head) = link!($globals, $p);
    link!($globals, $p) = null;
    flush_node_list($globals, $p)?;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::contrib_head;
    use crate::section_0202::flush_node_list;
}}
