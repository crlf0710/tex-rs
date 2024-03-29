//! @ When node |cur_p| is a glue node, we look at |prev_p| to see whether or not
//! a breakpoint is legal at |cur_p|, as explained above.
//
// @<If node |cur_p| is a legal breakpoint, call...@>=
pub(crate) macro If_node_cur_p_is_a_legal_breakpoint__call_try_break__then_update_the_active_widths_by_including_the_glue_in_glue_ptr_cur_p($globals:expr, $prev_p:expr, $auto_breaking:expr) {{
    /// miscellaneous nodes of temporary interest
    let q: pointer;

    // if auto_breaking then
    if $auto_breaking {
        // begin if is_char_node(prev_p) then try_break(0,unhyphenated)
        if is_char_node!($globals, $prev_p) {
            try_break($globals, 0, unhyphenated.into())?;
        }
        // else if precedes_break(prev_p) then try_break(0,unhyphenated)
        else if precedes_break!($globals, $prev_p) {
            try_break($globals, 0, unhyphenated.into())?;
        }
        // else if (type(prev_p)=kern_node)and(subtype(prev_p)<>explicit) then
        else if r#type!($globals, $prev_p) == kern_node
            && subtype!($globals, $prev_p) as integer != kern_node_subtype::explicit as integer
        {
            // try_break(0,unhyphenated);
            try_break($globals, 0, unhyphenated.into())?;
        }
        // end;
    }
    // check_shrinkage(glue_ptr(cur_p)); q:=glue_ptr(cur_p);
    check_shrinkage!($globals, glue_ptr!($globals, $globals.cur_p));
    q = glue_ptr!($globals, $globals.cur_p);
    // act_width:=act_width+width(q);@|
    act_width!($globals) += width!($globals, q);
    // active_width[2+stretch_order(q)]:=@|
    //   active_width[2+stretch_order(q)]+stretch(q);@/
    $globals.active_width[2 + stretch_order!($globals, q)] += stretch!($globals, q);
    // active_width[6]:=active_width[6]+shrink(q)
    $globals.active_width[6] += shrink!($globals, q);

    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0134::is_char_node;
    use crate::section_0135::width;
    use crate::section_0148::precedes_break;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0155::kern_node;
    use crate::section_0155::kern_node_subtype;
    use crate::section_0819::unhyphenated;
    use crate::section_0825::check_shrinkage;
    use crate::section_0829::try_break;
    use crate::section_0866::act_width;
}}
