//! @ Here is the main switch in the |line_break| routine, where legal breaks
//! are determined. As we move through the hlist, we need to keep the |active_width|
//! array up to date, so that the badness of individual lines is readily calculated
//! by |try_break|. It is convenient to use the short name |act_width| for
//! the component of active width that represents real width as opposed to glue.
//
// @d act_width==active_width[1] {length from first active node to current node}
/// length from first active node to current node
pub(crate) macro act_width($globals:expr) {
    $globals.active_width[1]
}
// @d kern_break==begin if not is_char_node(link(cur_p)) and auto_breaking then
//   if type(link(cur_p))=glue_node then try_break(0,unhyphenated);
pub(crate) macro kern_break($globals:expr, $auto_breaking:expr) {{
    if !is_char_node!($globals, link!($globals, $globals.cur_p))
        && $auto_breaking
        && r#type!($globals, link!($globals, $globals.cur_p)) == glue_node
    {
        try_break($globals, 0, unhyphenated.into())?;
    }
    // act_width:=act_width+width(cur_p);
    act_width!($globals) += width!($globals, $globals.cur_p);
    // end
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::width;
    use crate::section_0149::glue_node;
    use crate::section_0819::unhyphenated;
    use crate::section_0829::try_break;
}}

// @<Call |try_break| if |cur_p| is a legal breakpoint...@>=
pub(crate) macro Call_try_break_if_cur_p_is_a_legal_breakpoint__on_the_second_pass__also_try_to_hyphenate_the_next_word__if_cur_p_is_a_glue_node__then_advance_cur_p_to_the_next_node_of_the_paragraph_that_could_possibly_be_a_legal_breakpoint {
    ($globals:expr, $prev_p:expr, $auto_breaking:expr) => {{
        // begin if is_char_node(cur_p) then
        if is_char_node!($globals, $globals.cur_p) {
            // @<Advance \(c)|cur_p| to the node following the present
            //   string of characters@>;
            crate::section_0867::Advance_cur_p_to_the_node_following_the_present_string_of_characters!($globals, $prev_p);
        }
        crate::region_forward_label! {
        |'done5|
        {
            // case type(cur_p) of
            let type_cur_p = r#type!($globals, $globals.cur_p);
            // hlist_node,vlist_node,rule_node: act_width:=act_width+width(cur_p);
            if type_cur_p == hlist_node || type_cur_p == vlist_node || type_cur_p == rule_node {
                act_width!($globals) += width!($globals, $globals.cur_p);
            }
            // whatsit_node: @<Advance \(p)past a whatsit node in the \(l)|line_break| loop@>;
            else if type_cur_p == whatsit_node {
                crate::section_1363::Advance_past_a_whatsit_node_in_the_line_break_loop!($globals);
            }
            // glue_node: begin @<If node |cur_p| is a legal breakpoint, call |try_break|;
            //   then update the active widths by including the glue in |glue_ptr(cur_p)|@>;
            else if type_cur_p == glue_node {
                crate::section_0868::If_node_cur_p_is_a_legal_breakpoint__call_try_break__then_update_the_active_widths_by_including_the_glue_in_glue_ptr_cur_p!
                    ($globals, $prev_p, $auto_breaking);
                // if second_pass and auto_breaking then
                if $globals.second_pass && $auto_breaking {
                    // @<Try to hyphenate the following word@>;
                    crate::section_0894::Try_to_hyphenate_the_following_word!($globals);
                }
                // end;
            }
            // kern_node: if subtype(cur_p)=explicit then kern_break
            else if type_cur_p == kern_node {
                if subtype!($globals, $globals.cur_p) as integer == kern_node_subtype::explicit as integer {
                    todo!("kern_break");
                }
                // else act_width:=act_width+width(cur_p);
                else {
                    act_width!($globals) += width!($globals, $globals.cur_p);
                }
            }
            // ligature_node: begin f:=font(lig_char(cur_p));
            else if type_cur_p == ligature_node {
                let f: internal_font_number;
                f = font!($globals, lig_char!($globals.cur_p));
                // act_width:=act_width+char_width(f)(char_info(f)(character(lig_char(cur_p))));
                let c = character!($globals, lig_char!($globals.cur_p));
                act_width!($globals) += char_width!($globals, f, char_info!($globals, f, c.numeric_value()));
                // end;
            }
            // disc_node: @<Try to break after a discretionary fragment, then |goto done5|@>;
            else if type_cur_p == disc_node {
                crate::section_0869::Try_to_break_after_a_discretionary_fragment__then_goto_done5!($globals, $prev_p, 'done5);
            }
            // math_node: begin auto_breaking:=(subtype(cur_p)=after); kern_break;
            else if type_cur_p == math_node {
                $auto_breaking = subtype!($globals, $globals.cur_p) == math_node_subtype::after as _;
                kern_break!($globals, $auto_breaking);
                // end;
            }
            // penalty_node: try_break(penalty(cur_p),unhyphenated);
            else if type_cur_p == penalty_node {
                try_break($globals, penalty!($globals, $globals.cur_p), unhyphenated.into())?;
            }
            // mark_node,ins_node,adjust_node: do_nothing;
            else if type_cur_p == mark_node || type_cur_p == ins_node || type_cur_p == adjust_node {
                do_nothing!();
            }
            // othercases confusion("paragraph")
            else {
                confusion($globals, crate::strpool_str!("paragraph"))?;
            }
            // @:this can't happen paragraph}{\quad paragraph@>
            // endcases;@/
            // prev_p:=cur_p; cur_p:=link(cur_p);
            $prev_p = $globals.cur_p;
            $globals.cur_p = link!($globals, $globals.cur_p);
            }
            // done5:end
            'done5 <-
        };
        use crate::pascal::integer;
        use crate::section_0016::do_nothing;
        use crate::section_0095::confusion;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0133::subtype;
        use crate::section_0134::is_char_node;
        use crate::section_0134::font;
        use crate::section_0134::character;
        use crate::section_0135::hlist_node;
        use crate::section_0135::width;
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0140::ins_node;
        use crate::section_0141::mark_node;
        use crate::section_0142::adjust_node;
        use crate::section_0143::ligature_node;
        use crate::section_0143::lig_char;
        use crate::section_0145::disc_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0147::math_node;
        use crate::section_0147::math_node_subtype;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0155::kern_node_subtype;
        use crate::section_0157::penalty_node;
        use crate::section_0157::penalty;
        use crate::section_0548::internal_font_number;
        use crate::section_0554::char_width;
        use crate::section_0554::char_info;
        use crate::section_0819::unhyphenated;
        use crate::section_0829::try_break;
    }}
}
