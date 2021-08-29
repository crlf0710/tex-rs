//! @ If many insertions are supposed to go into the same box, we want to know
//! the position of the last node in that box, so that we don't need to waste time
//! when linking further information into it. The |last_ins_ptr| fields of the
//! page insertion nodes are therefore used for this purpose during the
//! packaging phase.
//
// @<Prepare all the boxes involved in insertions to act as queues@>=
macro_rules! Prepare_all_the_boxes_involved_in_insertions_to_act_as_queues {
    ($globals:expr) => {{
        /// nodes being examined and/or changed
        let (mut p, mut r): (pointer, pointer);
        /// insertion box number
        let mut n: quarterword;
        // begin r:=link(page_ins_head);
        r = link!($globals, page_ins_head);
        // while r<>page_ins_head do
        while r != page_ins_head {
            // begin if best_ins_ptr(r)<>null then
            if best_ins_ptr!($globals, r) != null {
                // begin n:=qo(subtype(r)); ensure_vbox(n);
                n = qo!(subtype!($globals, r));
                ensure_vbox($globals, n)?;
                // if box(n)=null then box(n):=new_null_box;
                if r#box!($globals, n) == null {
                    r#box!($globals, n) = new_null_box($globals)?;
                }
                // p:=box(n)+list_offset;
                p = r#box!($globals, n) + list_offset as pointer;
                // while link(p)<>null do p:=link(p);
                while link!($globals, p) != null {
                    p = link!($globals, p);
                }
                // last_ins_ptr(r):=p;
                last_ins_ptr!($globals, r) = p;
                // end;
            }
            // r:=link(r);
            r = link!($globals, r);
            // end;
        }
        // end
        use crate::section_0113::quarterword;
        use crate::section_0135::list_offset;
        use crate::section_0136::new_null_box;
        use crate::section_0162::page_ins_head;
        use crate::section_0993::ensure_vbox;
    }}
}