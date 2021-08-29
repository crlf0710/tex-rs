//! ` `
// @<Delete \(t)the page-insertion nodes@>=
macro_rules! Delete_the_page_insertion_nodes {
    ($globals:expr) => {{
        /// nodes being examined and/or changed
        let (mut q, mut r): (pointer, pointer);
        // r:=link(page_ins_head);
        r = link!($globals, page_ins_head);
        // while r<>page_ins_head do
        while r != page_ins_head {
            // begin q:=link(r); free_node(r,page_ins_node_size); r:=q;
            q = link!($globals, r);
            free_node($globals, r, page_ins_node_size as _);
            r = q;
            // end;
        }
        // link(page_ins_head):=page_ins_head
        link!($globals, page_ins_head) = page_ins_head;
        use crate::section_0130::free_node;
        use crate::section_0162::page_ins_head;
        use crate::section_0981::page_ins_node_size;
    }}
}
