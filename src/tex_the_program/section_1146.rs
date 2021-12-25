//! ` `

// @<Calculate the natural width, |w|, by which...@>=
pub(crate) macro Calculate_the_natural_width__w__by_which_the_characters_of_the_final_line_extend_to_the_right_of_the_reference_point__plus_two_ems__or_set_w_to_max_dimen_if_the_non_blank_information_on_that_line_is_affected_by_stretching_or_shrinking($globals:expr, $w:expr) {#[allow(unused_assignments)]{
    /// `w` plus possible glue amount
    let mut v;
    /// current node when calculating `pre_display_size`
    let mut p;
    // v:=shift_amount(just_box)+2*quad(cur_font); w:=-max_dimen;
    v = shift_amount!($globals, $globals.just_box) + scaled::new_from_inner(2 * quad!($globals,
        internal_font_number::new(cur_font!($globals))).inner());
    $w = scaled::new_from_inner(-max_dimen);
    // p:=list_ptr(just_box);
    p = list_ptr!($globals, $globals.just_box);
    crate::region_forward_label!{
        |'done|
        {
            // while p<>null do
            while p != null {
                /// increment to `v`
                let d;
                crate::region_forward_label! {
                    |'not_found|
                    {
                        crate::region_forward_label! {
                            |'found|
                            {
                                // begin @<Let |d| be the natural width of node |p|;
                                //   if the node is ``visible,'' |goto found|;
                                //   if the node is glue that stretches or shrinks, set |v:=max_dimen|@>;
                                crate::section_1147::Let_d_be_the_natural_width_of_node_p__if_the_node_is_visible__goto_found__if_the_node_is_glue_that_stretches_or_shrinks__set_v_to_max_dimen!($globals, p, d, v, 'found);
                                // if v<max_dimen then v:=v+d;
                                if v < scaled::new_from_inner(max_dimen) {
                                    v += d;
                                }
                                // goto not_found;
                                crate::goto_forward_label!('not_found);
                            }
                            // found: if v<max_dimen then
                            'found <-
                        }
                        if v < scaled::new_from_inner(max_dimen) {
                            // begin v:=v+d; w:=v;
                            v += d;
                            $w = v;
                            // end
                        }
                        // else  begin w:=max_dimen; goto done;
                        else {
                            $w = scaled::new_from_inner(max_dimen);
                            crate::goto_forward_label!('done);
                            // end;
                        }
                    }
                    // not_found: p:=link(p);
                    'not_found <-
                }
                p = link!($globals, p);
                // end;
            }
        }
        // done:
        'done <-
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::list_ptr;
    use crate::section_0135::shift_amount;
    use crate::section_0230::cur_font;
    use crate::section_0421::max_dimen;
    use crate::section_0548::internal_font_number;
    use crate::section_0558::quad;
}}
