//! @ We have now tied up all the loose ends of the first pass of |mlist_to_hlist|.
//! The second pass simply goes through and hooks everything together with the
//! proper glue and penalties. It also handles the |left_noad| and |right_noad| that
//! might be present, since |max_h| and |max_d| are now known. Variable |p| points
//! to a node at the current end of the final hlist.
//
// @<Make a second pass over the mlist, ...@>=
pub(crate) macro Make_a_second_pass_over_the_mlist__removing_all_noads_and_inserting_the_proper_spacing_and_penalties($globals:expr, $mlist:expr, $style:expr, $penalties:expr) {{
    /// temporary registers for list construction
    let (mut p,): (pointer,);
    /// runs through the mlist
    let mut q;
    /// the most recent noad preceding `q`
    let mut r;
    /// the `type` of noad `r`, or `op_noad` if `r=null`
    let mut r_type;
    // p:=temp_head; link(p):=null; q:=mlist; r_type:=0; cur_style:=style;
    p = temp_head;
    link!($globals, p) = null;
    q = $mlist;
    r_type = 0;
    $globals.cur_style = $style;
    // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
    crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!($globals);
    // while q<>null do
    while q != null {
        /// the effective |type| of noad |q| during the second pass
        let mut t;
        /// the size of a noad to be deleted
        let s;
        /// a penalty to be inserted
        let pen;
        // begin @<If node |q| is a style node, change the style and |goto delete_q|;
        //   otherwise if it is not a noad, put it into the hlist,
        //   advance |q|, and |goto done|; otherwise set |s| to the size
        //   of noad |q|, set |t| to the associated type (|ord_noad..
        //   inner_noad|), and set |pen| to the associated penalty@>;
        crate::region_forward_label!{
            |'done|
            {
                crate::region_forward_label!{
                    |'delete_q|
                    {
                        crate::section_0761::If_node_q_is_a_style_node__change_the_style_and_goto_delete_q__otherwise_if_it_is_not_a_noad__put_it_into_the_hlist__advance_q__and_goto_done__otherwise_set_s_to_the_size_of_noad_q__set_t_to_the_associated_type_ord_noad_to_inner_noad__and_set_pen_to_the_associated_penalty!($globals, q, t, s, pen);
                        // @<Append inter-element spacing based on |r_type| and |t|@>;
                        crate::section_0766::Append_inter_element_spacing_based_on_r_type_and_t!($globals, r_type, t);
                        // @<Append any |new_hlist| entries for |q|, and any appropriate penalties@>;
                        crate::section_0767::Append_any_new_hlist_entries_for_q__and_any_appropriate_penalties!($globals, q, p, pen, $penalties);
                        // r_type:=t;
                        r_type = t;
                        // delete_q: r:=q; q:=link(q); free_node(r,s);
                    }
                    'delete_q <-
                }
                r = q;
                q = link!($globals, q);
                free_node($globals, r, s as _);
            }
            // done: end
            'done <-
        }
    }
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0162::temp_head;
}}
