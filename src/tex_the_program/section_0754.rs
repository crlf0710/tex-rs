//! @ When we get to the following part of the program, we have ``fallen through''
//! from cases that did not lead to |check_dimensions| or |done_with_noad| or
//! |done_with_node|. Thus, |q|~points to a noad whose nucleus may need to be
//! converted to an hlist, and whose subscripts and superscripts need to be
//! appended if they are present.
//!
//! If |nucleus(q)| is not a |math_char|, the variable |delta| is the amount
//! by which a superscript should be moved right with respect to a subscript
//! when both are present.
//! @^subscripts@>
//! @^superscripts@>
//
// @<Convert \(n)|nucleus(q)| to an hlist and attach the sub/superscripts@>=
pub(crate) macro Convert_nucleus_q_to_an_hlist_and_attach_the_sub_superscripts($globals:expr, $q:expr, $delta:expr, $lbl_check_dimensions:lifetime) {{
    /// temporary registers for list construction
    let (mut p,) = (null,);
    // case math_type(nucleus(q)) of
    let math_type_nucleus_q = math_type!($globals, nucleus!($q));
    // math_char, math_text_char:
    if math_type_nucleus_q == math_type_kind::math_char as _
        || math_type_nucleus_q == math_type_kind::math_text_char as _
    {
        // @<Create a character node |p| for |nucleus(q)|, possibly followed
        // by a kern node for the italic correction, and set |delta| to the
        // italic correction if a subscript is present@>;
        crate::section_0755::Create_a_character_node_p_for_nucleus_q__possibly_followed_by_a_kern_node_for_the_italic_correction__and_set_delta_to_the_italic_correction_if_a_subscript_is_present!($globals, $q, p, $delta);
    }
    // empty: p:=null;
    else if math_type_nucleus_q == math_type_kind::empty as _ {
        p = null;
    }
    // sub_box: p:=info(nucleus(q));
    else if math_type_nucleus_q == math_type_kind::sub_box as _ {
        p = info_inner!($globals, nucleus!($q));
    }
    // sub_mlist: begin cur_mlist:=info(nucleus(q)); save_style:=cur_style;
    else if math_type_nucleus_q == math_type_kind::sub_mlist as _ {
        /// holds `cur_style` during recursion
        let save_style;
        $globals.cur_mlist = info_inner!($globals, nucleus!($q));
        save_style = $globals.cur_style;
        // mlist_penalties:=false; mlist_to_hlist; {recursive call}
        $globals.mlist_penalties = false;
        /// recursive call
        mlist_to_hlist($globals)?;
        // @^recursion@>
        // cur_style:=save_style; @<Set up the values...@>;
        $globals.cur_style = save_style;
        crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!($globals);
        // p:=hpack(link(temp_head),natural);
        p = hpack($globals, link!($globals, temp_head), natural0!(), natural1!())?;
        // end;
    }
    // othercases confusion("mlist2")
    else {
        confusion($globals, crate::strpool_str!("mlist2"))?;
        // @:this can't happen mlist2}{\quad mlist2@>
    }
    // endcases;@/
    // new_hlist(q):=p;
    new_hlist!($globals, $q) = p as _;
    // if (math_type(subscr(q))=empty)and(math_type(supscr(q))=empty) then
    if math_type!($globals, subscr!($q)) == math_type_kind::empty as _
        && math_type!($globals, supscr!($q)) == math_type_kind::empty as _
    {
        // goto check_dimensions;
        crate::goto_forward_label!($lbl_check_dimensions);
    }
    // make_scripts(q,delta)
    make_scripts($globals, $q, $delta)?;
    use crate::section_0095::confusion;
    use crate::section_0115::null;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0162::temp_head;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0649::hpack;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0725::new_hlist;
    use crate::section_0726::mlist_to_hlist;
    use crate::section_0756::make_scripts;
}}
