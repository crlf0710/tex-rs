//! @ When the following code is executed, the current page runs from node
//! |link(page_head)| to node |prev_p|, and the nodes from |p| to |page_tail|
//! are to be placed back at the front of the contribution list. Furthermore
//! the heldover insertions appear in a list from |link(hold_head)| to |q|; we
//! will put them into the current page list for safekeeping while the user's
//! output routine is active.  We might have |q=hold_head|; and |p=null| if
//! and only if |prev_p=page_tail|. Error messages are suppressed within
//! |vpackage|, since the box might appear to be overfull or underfull simply
//! because the stretch and shrink from the \.{\\skip} registers for inserts
//! are not actually present in the box.
//
// @<Break the current page at node |p|, put it...@>=
macro_rules! Break_the_current_page_at_node_p__put_it_in_box_255__and_put_the_remaining_nodes_on_the_contribution_list {
    ($globals:expr, $p:expr, $q:expr) => {{
        // if p<>null then
        if $p != null {
            // begin if link(contrib_head)=null then
            //   if nest_ptr=0 then tail:=page_tail
            //   else contrib_tail:=page_tail;
            // link(page_tail):=link(contrib_head);
            // link(contrib_head):=p;
            // link(prev_p):=null;
            // end;
            todo!("p != null");
        }
        /// saved value of `vbadness`
        let save_vbadness: integer;
        /// saved value of `vfuzz`
        let save_vfuzz: scaled;
        // save_vbadness:=vbadness; vbadness:=inf_bad;
        save_vbadness = vbadness!($globals);
        vbadness!($globals) = inf_bad as _;
        // save_vfuzz:=vfuzz; vfuzz:=max_dimen; {inhibit error messages}
        save_vfuzz = vfuzz!($globals);
        vfuzz!($globals) = scaled::new_from_inner(max_dimen);
        /// inhibit error messages
        const _: () = ();
        // box(255):=vpackage(link(page_head),best_size,exactly,page_max_depth);
        r#box!($globals, 255) = vpackage(
            $globals,
            link!($globals, page_head),
            $globals.best_size,
            exactly.into(),
            $globals.page_max_depth,
        )?;
        // vbadness:=save_vbadness; vfuzz:=save_vfuzz;
        vbadness!($globals) = save_vbadness;
        vfuzz!($globals) = save_vfuzz;
        // if last_glue<>max_halfword then delete_glue_ref(last_glue);
        if $globals.last_glue != max_halfword {
            delete_glue_ref($globals, $globals.last_glue);
        }
        // @<Start a new current page@>; {this sets |last_glue:=max_halfword|}
        Start_a_new_current_page!($globals);
        /// this sets `last_glue:=max_halfword`
        const _: () = ();
        // if q<>hold_head then
        if $q != hold_head {
            // begin link(page_head):=link(hold_head); page_tail:=q;
            link!($globals, page_head) = link!($globals, hold_head);
            $globals.page_tail = $q;
            // end
        }
        use crate::pascal::integer;
        use crate::section_0101::scaled;
        use crate::section_0108::inf_bad;
        use crate::section_0110::max_halfword;
        use crate::section_0201::delete_glue_ref;
        use crate::section_0421::max_dimen;
        use crate::section_0644::exactly;
        use crate::section_0668::vpackage;
    }};
}
