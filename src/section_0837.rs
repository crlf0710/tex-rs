//! @ When we insert a new active node for a break at |cur_p|, suppose this
//! new node is to be placed just before active node |a|; then we essentially
//! want to insert `$\delta\,|cur_p|\,\delta^\prime$' before |a|, where
//! $\delta=\alpha(a)-\alpha(|cur_p|)$ and $\delta^\prime=\alpha(|cur_p|)-\alpha(a)$
//! in the notation explained above.  The |cur_active_width| array now holds
//! $\gamma+\beta(|cur_p|)-\alpha(a)$; so $\delta$ can be obtained by
//! subtracting |cur_active_width| from the quantity $\gamma+\beta(|cur_p|)-
//! \alpha(|cur_p|)$. The latter quantity can be regarded as the length of a
//! line ``from |cur_p| to |cur_p|''; we call it the |break_width| at |cur_p|.
//!
//! The |break_width| is usually negative, since it consists of the background
//! (which is normally zero) minus the width of nodes following~|cur_p| that are
//! eliminated after a break. If, for example, node |cur_p| is a glue node, the
//! width of this glue is subtracted from the background; and we also look
//! ahead to eliminate all subsequent glue and penalty and kern and math
//! nodes, subtracting their widths as well.
//!
//! Kern nodes do not disappear at a line break unless they are |explicit|.
//
// @d set_break_width_to_background(#)==break_width[#]:=background[#]
macro_rules! set_break_width_to_background {
    ($globals:expr, $idx:expr) => {{
        $globals.break_width[$idx] = $globals.background[$idx]
    }}
}

// @<Compute the values of |break...@>=
macro_rules! Compute_the_values_of_break_width {
    ($globals:expr, $break_type:expr, $no_break_yet:expr) => {{
        /// runs through nodes ahead of `cur_p`
        let mut s: pointer;

        // begin no_break_yet:=false; do_all_six(set_break_width_to_background);
        $no_break_yet = false;
        do_all_six!(set_break_width_to_background !; @globals = $globals);
        // s:=cur_p;
        s = $globals.cur_p;
        // if break_type>unhyphenated then if cur_p<>null then
        if $break_type > unhyphenated && $globals.cur_p != null {
            // @<Compute the discretionary |break_width| values@>;
            todo!("compute the discretionary break_width");
        }
        region_forward_label!(
        |'done|
        {
        // while s<>null do
        while s != null {
            // begin if is_char_node(s) then goto done;
            if is_char_node!($globals, s) {
                goto_forward_label!('done);
            }
            // case type(s) of
            let type_s = r#type!($globals, s);
            // glue_node:@<Subtract glue from |break_width|@>;
            if type_s == glue_node {
                Subtract_glue_from_break_width!($globals, s);
            }
            // penalty_node: do_nothing;
            else if type_s == penalty_node {
                do_nothing!();
            }
            // math_node: break_width[1]:=break_width[1]-width(s);
            else if type_s == math_node {
                todo!();
            }
            // kern_node: if subtype(s)<>explicit then goto done
            else if type_s == kern_node {
                if subtype!($globals, s) as integer != kern_node_subtype::explicit as integer {
                    goto_forward_label!('done)
                }
                // else break_width[1]:=break_width[1]-width(s);
                else {
                    $globals.break_width[1] = $globals.break_width[1] - width!($globals, s);
                }
            }
            // othercases goto done
            else {
                goto_forward_label!('done);
            }
            // endcases;@/
            // s:=link(s);
            s = link!($globals, s);
            // end;
        }
        }
        // done: end
        'done <-
        );
        use crate::section_0115::null;
        use crate::section_0147::math_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0155::kern_node_subtype;
        use crate::section_0157::penalty_node;
        use crate::section_0819::unhyphenated;
    }}
}
