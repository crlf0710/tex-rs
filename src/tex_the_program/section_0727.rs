//! ` `
//!
//! We use the fact that no character nodes appear in an mlist, hence
//! the field |type(q)| is always present.
//
// @<Process node-or-noad...@>=
pub(crate) macro Process_node_or_noad_q_as_much_as_possible_in_preparation_for_the_second_pass_of_mlist_to_hlist__then_move_to_the_next_item_in_the_mlist($globals:expr, $q:expr, $r:expr, $r_type:expr, $max_h:expr, $max_d:expr, $delta:expr) {{
    /// temporary registers for list construction
    let z;
    crate::region_forward_label! {
        |'done_with_node|
        {
            crate::region_forward_label! {
                |'done_with_noad|
                {
                    // begin @<Do first-pass processing based on |type(q)|; |goto done_with_noad|
                    //   if a noad has been fully processed, |goto check_dimensions| if it
                    //   has been translated into |new_hlist(q)|, or |goto done_with_node|
                    //   if a node has been fully processed@>;
                    crate::region_forward_label! {
                        |'check_dimensions|
                        {
                            crate::section_0728::Do_first_pass_processing_based_on_type_q__goto_done_with_noad_if_a_noad_has_been_fully_processed__goto_check_dimensions_if_it_has_been_translated_into_new_hlist_q___or_goto_done_with_node_if_a_node_has_been_fully_processed!($globals, $q, $r, $r_type, $delta, $max_h, $max_d, 'check_dimensions, 'done_with_noad, 'done_with_node);
                        }
                        // check_dimensions: z:=hpack(new_hlist(q),natural);
                        'check_dimensions <-
                    }
                    z = hpack(
                        $globals,
                        new_hlist!($globals, $q) as _,
                        natural0!(),
                        natural1!(),
                    )?;
                    // if height(z)>max_h then max_h:=height(z);
                    if height!($globals, z) > $max_h {
                        $max_h = height!($globals, z);
                    }
                    // if depth(z)>max_d then max_d:=depth(z);
                    if depth!($globals, z) > $max_d {
                        $max_d = depth!($globals, z);
                    }
                    // free_node(z,box_node_size);
                    free_node($globals, z, box_node_size as _);
                    // done_with_noad: r:=q; r_type:=type(r);
                }
                'done_with_noad <-
            }
            $r = $q;
            $r_type = r#type!($globals, $r);
            // done_with_node: q:=link(q);
        }
        'done_with_node <-
    }
    $q = link!($globals, $q);
    // end
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0135::box_node_size;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
    use crate::section_0725::new_hlist;
}}
