//! @ We will set |best_ins_ptr:=null| and package the box corresponding to
//! insertion node~|r|, just after making the final insertion into that box.
//! If this final insertion is `|split_up|', the remainder after splitting
//! and pruning (if any) will be carried over to the next page.
//
// @<Either insert the material specified by node |p| into...@>=
pub(crate) macro Either_insert_the_material_specified_by_node_p_into_the_appropriate_box__or_hold_it_for_the_next_page__also_delete_node_p_from_the_current_page($globals:expr, $p:expr, $prev_p:expr, $q:expr) {{
    /// nodes being examined and/or changed
    let mut r;
    /// should the present insertion be held over?
    let wait;

    // begin r:=link(page_ins_head);
    r = link!($globals, page_ins_head);
    // while subtype(r)<>subtype(p) do r:=link(r);
    while subtype!($globals, r) != subtype!($globals, $p) {
        r = link!($globals, r);
    }
    // if best_ins_ptr(r)=null then wait:=true
    if best_ins_ptr!($globals, r) == null {
        wait = true;
    }
    // else  begin wait:=false; s:=last_ins_ptr(r); link(s):=ins_ptr(p);
    else {
        /// nodes being examined and/or changed
        let mut s;
        wait = false;
        s = last_ins_ptr!($globals, r);
        link!($globals, s) = ins_ptr!($globals, $p);
        // if best_ins_ptr(r)=p then
        if best_ins_ptr!($globals, r) == $p {
            // @<Wrap up the box specified by node |r|, splitting node |p| if
            // called for; set |wait:=true| if node |p| holds a remainder after
            // splitting@>
            crate::section_1021::Wrap_up_the_box_specified_by_node_r__splitting_node_p_if_called_for__set_wait_to_true_if_node_p_holds_a_remainder_after_splitting!($globals, $p, r);
        }
        // else  begin while link(s)<>null do s:=link(s);
        else {
            while link!($globals, s) != null {
                s = link!($globals, s);
            }
            // last_ins_ptr(r):=s;
            last_ins_ptr!($globals, r) = s;
            // end;
        }
        // end;
    }
    // @<Either append the insertion node |p| after node |q|, and remove it
    //   from the current page, or delete |node(p)|@>;
    crate::section_1022::Either_append_the_insertion_node_p_after_node_q__and_remove_it_from_the_current_page__or_delete_node_p!($globals, $p, $prev_p, $q, wait);
    // end

    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0140::ins_ptr;
    use crate::section_0162::page_ins_head;
    use crate::section_0981::best_ins_ptr;
    use crate::section_0981::last_ins_ptr;
}}
