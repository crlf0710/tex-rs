//! ` `
//! Glue and penalty and kern and math nodes are deleted at the beginning of
//! a line, except in the anomalous case that the node to be deleted is actually
//! one of the chosen breakpoints. Otherwise
//! the pruning done here is designed to match
//! the lookahead computation in |try_break|, where the |break_width| values
//! are computed for non-discretionary breakpoints.
//
// @<Prune unwanted nodes at the beginning of the next line@>=
pub(crate) macro Prune_unwanted_nodes_at_the_beginning_of_the_next_line($globals:expr) {{
    /// temporary registers for list manipulation
    let (q, r);

    // begin r:=temp_head;
    r = temp_head;
    crate::region_forward_label! {
        |'done1|
        {
            // loop@+  begin q:=link(r);
            loop {
                q = link!($globals, r);
                // if q=cur_break(cur_p) then goto done1;
                //   {|cur_break(cur_p)| is the next breakpoint}
                if q == cur_break!($globals, $globals.cur_p) {
                    /// |cur_break(cur_p)| is the next breakpoint
                    const _ : () = ();
                    crate::goto_forward_label!('done1);
                }
                // {now |q| cannot be |null|}
                /// now |q| cannot be |null|
                const _ : () = ();
                // if is_char_node(q) then goto done1;
                if is_char_node!($globals, q) {
                    crate::goto_forward_label!('done1);
                }
                // if non_discardable(q) then goto done1;
                if non_discardable!($globals, q) {
                    crate::goto_forward_label!('done1);
                }
                // if type(q)=kern_node then if subtype(q)<>explicit then goto done1;
                // r:=q; {now |type(q)=glue_node|, |kern_node|, |math_node|, or |penalty_node|}
                // end;
                todo!("prune");
            }
        }
        // done1: if r<>temp_head then
        'done1 <-
    };
    if r != temp_head {
        // begin link(r):=null; flush_node_list(link(temp_head));
        // link(temp_head):=q;
        // end;
        todo!("prune2");
    }
    // end
    use crate::section_0118::link;
    use crate::section_0134::is_char_node;
    use crate::section_0148::non_discardable;
    use crate::section_0162::temp_head;
    use crate::section_0821::cur_break;
}}
