//! @ The code here is an example of a many-way switch into routines that
//! merge together in different places. Some people call this unstructured
//! programming, but the author doesn't see much wrong with it, as long as
//! @^Knuth, Donald Ervin@>
//! the various labels have a well-understood meaning.
//
// @<Move node |p| to the current page; ...@>=
macro_rules! Move_node_p_to_the_current_page__if_it_is_time_for_a_page_break__put_the_nodes_following_the_break_back_onto_the_contribution_list__and_return_to_the_user_s_output_routine_if_there_is_one {
    ($globals:expr, $p:expr) => { #[allow(unused_assignments)] {
        /// penalty to be added to the badness
        let pi: integer;
        region_forward_label!(
        |'done|
        {
        region_forward_label!(
        |'done1|
        {
        region_forward_label!(
        |'contribute|
        {
        region_forward_label!(
        |'update_heights|
        {
        // @<If the current page is empty and node |p| is to be deleted, |goto done1|;
        //   otherwise use node |p| to update the state of the current page;
        //   if this node is an insertion, |goto contribute|; otherwise if this node
        //   is not a legal breakpoint, |goto contribute| or |update_heights|;
        //   otherwise set |pi| to the penalty associated with this breakpoint@>;
        If_the_current_page_is_empty_and_node_p_is_to_be_deleted__goto_done1__otherwise_use_node_p_to_update_the_state_of_the_current_page__if_this_node_is_an_insertion__goto_contribute__otherwise_if_this_node_is_not_a_legal_breakpoint__goto_contribute_or_update_heights__otherwise_set_pi_to_the_penalty_associated_with_this_breakpoint!
            ($globals, $p, pi, 'update_heights, 'done1);
        // @<Check if node |p| is a new champion breakpoint; then \(if)if it is time for
        //   a page break, prepare for output, and either fire up the user's
        //   output routine and |return| or ship out the page and |goto done|@>;
        todo!("check if champion breakpoint");
        // if (type(p)<glue_node)or(type(p)>kern_node) then goto contribute;
        if r#type!($globals, $p) < glue_node || r#type!($globals, $p) > kern_node {
            goto_forward_label!('contribute);
        }
        }
        // update_heights:@<Update the current page measurements with respect to the
        //   glue or kern specified by node~|p|@>;
        'update_heights <-
        );
        todo!("update the current page measurements");
        }
        // contribute: @<Make sure that |page_max_depth| is not exceeded@>;
        // @<Link node |p| into the current page and |goto done|@>;
        'contribute <-
        );
        todo!("make sure depth not exceeded");
        todo!("link node p");
        }
        // done1:@<Recycle node |p|@>;
        'done1 <-
        );
        Recycle_node_p!($globals, $p);
        }
        // done:
        'done <-
        );
        use crate::pascal::integer;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
    }}
}
