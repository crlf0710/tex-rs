//! @ We need to be careful that |w|, |v|, and |d| do not depend on any |glue_set|
//! values, since such values are subject to system-dependent rounding.
//! System-dependent numbers are not allowed to infiltrate parameters like
//! |pre_display_size|, since \TeX82 is supposed to make the same decisions on all
//! machines.
//
// @<Let |d| be the natural width of this glue...@>=
pub(crate) macro Let_d_be_the_natural_width_of_this_glue__if_stretching_or_shrinking__set_v_to_max_dimen__goto_found_in_the_case_of_leaders($globals:expr, $p:expr, $d:expr, $v:expr, $lbl_found:lifetime) {{
    /// glue specification when calculating `pre_display_size`
    let q;
    // begin q:=glue_ptr(p); d:=width(q);
    q = glue_ptr!($globals, $p);
    $d = width!($globals, q);
    // if glue_sign(just_box)=stretching then
    if glue_sign!($globals, $globals.just_box) == glue_sign::stretching as _ {
        // begin if (glue_order(just_box)=stretch_order(q))and@|
        //    (stretch(q)<>0) then
        if glue_order!($globals, $globals.just_box) == stretch_order!($globals, q)
            && stretch!($globals, q) != scaled::zero()
        {
            // v:=max_dimen;
            $v = scaled::new_from_inner(max_dimen);
        }
        // end
    }
    // else if glue_sign(just_box)=shrinking then
    else if glue_sign!($globals, $globals.just_box) == glue_sign::shrinking as _ {
        // begin if (glue_order(just_box)=shrink_order(q))and@|
        //    (shrink(q)<>0) then
        //   v:=max_dimen;
        // end;
        todo!("shrinking");
    }
    // if subtype(p)>=a_leaders then goto found;
    if subtype!($globals, $p) >= glue_node_subtype::a_leaders as _ {
        crate::goto_forward_label!($lbl_found);
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0133::subtype;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_sign;
    use crate::section_0135::width;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0421::max_dimen;
}}
