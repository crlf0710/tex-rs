//! ` `

// @<Append an insertion to the current page and |goto contribute|@>=
pub(crate) macro Append_an_insertion_to_the_current_page_and_goto_contribute($globals:expr, $p:expr, $lbl_contribute:lifetime) {{
    /// nodes being examined
    let mut r;
    /// insertion box number
    let mut n;
    // begin if page_contents=empty then freeze_page_specs(inserts_only);
    if $globals.page_contents == page_contents_kind::empty {
        freeze_page_specs($globals, page_contents_kind::inserts_only);
    }
    // n:=subtype(p); r:=page_ins_head;
    n = subtype!($globals, $p);
    r = page_ins_head;
    // while n>=subtype(link(r)) do r:=link(r);
    while n >= subtype!($globals, link!($globals, r)) {
        r = link!($globals, r);
    }
    // n:=qo(n);
    n = qo!(n);
    // if subtype(r)<>qi(n) then
    if subtype!($globals, r) != qi!(n) {
        // @<Create a page insertion node with |subtype(r)=qi(n)|, and
        //   include the glue correction for box |n| in the
        //   current page state@>;
        crate::section_1009::Create_a_page_insertion_node_with_subtype_r_eq_qi_n__and_include_the_glue_correction_for_box_n_in_the_current_page_state!($globals, r, n);
    }
    // if type(r)=split_up then insert_penalties:=insert_penalties+float_cost(p)
    if r#type!($globals, r) == page_ins_node_type::split_up as _ {
        $globals.insert_penalties += float_cost!($globals, $p);
    }
    // else  begin last_ins_ptr(r):=p;
    else {
        /// sizes used for insertion calculations
        let (delta, h);
        last_ins_ptr!($globals, r) = $p;
        // delta:=page_goal-page_total-page_depth+page_shrink;
        //   {this much room is left if we shrink the maximum}
        delta = page_goal!($globals) - page_total!($globals) - page_depth!($globals) + page_shrink!($globals);
        /// this much room is left if we shrink the maximum
        const _ : () = ();
        // if count(n)=1000 then h:=height(p)
        if count!($globals, n) == 1000 {
            h = height!($globals, $p);
        }
        // else h:=x_over_n(height(p),1000)*count(n); {this much room is needed}
        else {
            h = scaled::new_from_inner(x_over_n($globals, height!($globals, $p), 1000).inner() * count!($globals, n));
            /// this much room is needed
            const _: () = ();
        }
        // if ((h<=0)or(h<=delta))and(height(p)+height(r)<=dimen(n)) then
        if (h <= scaled::zero() || h <= delta) && (height!($globals, $p) + height!($globals, r) <= dimen!($globals, n)) {
            // begin page_goal:=page_goal-h; height(r):=height(r)+height(p);
            page_goal!($globals) -= h;
            let height_p = height!($globals, $p);
            height!($globals, r) += height_p;
            // end
        }
        // else @<Find the best way to split the insertion, and change
        //   |type(r)| to |split_up|@>;
        else {
            crate::section_1010::Find_the_best_way_to_split_the_insertion__and_change_type_r_to_split_up!($globals, r, n, $p);
        }
        // end;
    }
    // goto contribute;
    crate::goto_forward_label!($lbl_contribute);
    // end
    use crate::section_0101::scaled;
    use crate::section_0106::x_over_n;
    use crate::section_0112::qi;
    use crate::section_0112::qo;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::height;
    use crate::section_0140::float_cost;
    use crate::section_0162::page_ins_head;
    use crate::section_0236::count;
    use crate::section_0247::dimen;
    use crate::section_0980::page_contents_kind;
    use crate::section_0981::page_ins_node_type;
    use crate::section_0981::last_ins_ptr;
    use crate::section_0982::page_goal;
    use crate::section_0982::page_total;
    use crate::section_0982::page_depth;
    use crate::section_0982::page_shrink;
    use crate::section_0987::freeze_page_specs;
}}
