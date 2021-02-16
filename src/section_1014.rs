//! @ As the page is finally being prepared for output,
//! pointer |p| runs through the vlist, with |prev_p| trailing behind;
//! pointer |q| is the tail of a list of insertions that
//! are being held over for a subsequent page.
//
// @<Put the \(o)optimal current page into box 255...@>=
macro_rules! Put_the_optimal_current_page_into_box_255__update_first_mark_and_bot_mark__append_insertions_to_their_boxes__and_put_the_remaining_nodes_back_on_the_contribution_list {
    ($globals:expr, $c:expr) => {{
        /// nodes being examined and/or changed
        let (mut p, q): (pointer, pointer);
        /// predecessor of `p`
        let mut prev_p: pointer;
        /// saved value of `split_top_skip`
        let save_split_top_skip: pointer;
        // if c=best_page_break then best_page_break:=null; {|c| not yet linked in}
        /// `c` not yet linked in
        const _ : () = ();
        if $c == $globals.best_page_break {
            $globals.best_page_break = null;
        }
        // @<Ensure that box 255 is empty before output@>;
        Ensure_that_box_255_is_empty_before_output!($globals);
        // insert_penalties:=0; {this will count the number of insertions held over}
        /// this will count the number of insertions held over
        const _ : () = ();
        $globals.insert_penalties = 0;
        // save_split_top_skip:=split_top_skip;
        save_split_top_skip = split_top_skip!($globals);
        // if holding_inserts<=0 then
        if holding_inserts!($globals) <= 0 {
            // @<Prepare all the boxes involved in insertions to act as queues@>;
            todo!("prepare all the boxes");
        }
        // q:=hold_head; link(q):=null; prev_p:=page_head; p:=link(prev_p);
        q = hold_head;
        link!($globals, q) = null;
        prev_p = page_head;
        p = link!($globals, prev_p);
        // while p<>best_page_break do
        while p != $globals.best_page_break {
            // begin if type(p)=ins_node then
            if r#type!($globals, p) == ins_node {
                todo!("ins node prior of page break");
                // begin if holding_inserts<=0 then
                //    @<Either insert the material specified by node |p| into the
                //      appropriate box, or hold it for the next page;
                //      also delete node |p| from the current page@>;
                // end
            }
            // else if type(p)=mark_node then @<Update the values of
            //   |first_mark| and |bot_mark|@>;
            else if r#type!($globals, p) == mark_node {
                todo!("mark node prior of page break");
            }
            // prev_p:=p; p:=link(prev_p);
            prev_p = p;
            p = link!($globals, prev_p);
            // end;
        }
        // split_top_skip:=save_split_top_skip;
        split_top_skip!($globals) = save_split_top_skip;
        // @<Break the current page at node |p|, put it in box~255,
        //   and put the remaining nodes on the contribution list@>;
        todo!("break cur page");
        // @<Delete \(t)the page-insertion nodes@>
        Delete_the_page_insertion_nodes!($globals);
        use crate::section_0140::ins_node;
        use crate::section_0141::mark_node;
        use crate::section_0162::page_head;
        use crate::section_0162::hold_head;
    }}
}
