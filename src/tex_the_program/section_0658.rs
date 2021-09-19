//! ` `
// @<Determine horizontal glue stretch setting...@>=
pub(crate) macro Determine_horizontal_glue_stretch_setting__then_return_or_goto_common_ending($globals:expr, $r:expr, $x:expr, $lbl_common_ending:lifetime) {{
    /// order of infinity
    let o: glue_ord;
    // begin @<Determine the stretch order@>;
    crate::section_0659::Determine_the_stretch_order!($globals, o);
    // glue_order(r):=o; glue_sign(r):=stretching;
    glue_order!($globals, $r) = o as _;
    glue_sign!($globals, $r) = glue_sign::stretching as _;
    // if total_stretch[o]<>0 then glue_set(r):=unfloat(x/total_stretch[o])
    if $globals.total_stretch[o] != scaled::zero() {
        glue_set!($globals, $r) =
            unfloat!(($x.inner() as real) / ($globals.total_stretch[o].inner() as real));
    }
    // @^real division@>
    // else  begin glue_sign(r):=normal;
    else {
        glue_sign!($globals, $r) = glue_sign::normal as _;
        // set_glue_ratio_zero(glue_set(r)); {there's nothing to stretch}
        /// there's nothing to stretch
        set_glue_ratio_zero!(glue_set!($globals, $r));
        // end;
    }
    // if o=normal then if list_ptr(r)<>null then
    if o == glue_ord::normal && list_ptr!($globals, $r) != null {
        // @<Report an underfull hbox and |goto common_ending|, if this box
        //   is sufficiently bad@>;
        crate::section_0660::Report_an_underfull_hbox_and_goto_common_ending__if_this_box_is_sufficiently_bad!(
            $globals,
            $x,
            $lbl_common_ending
        );
    }
    // return;
    crate::return_nojump!($r);
    // end
    use crate::pascal::real;
    use crate::section_0101::scaled;
    use crate::section_0109::set_glue_ratio_zero;
    use crate::section_0109::unfloat;
    use crate::section_0115::null;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::list_ptr;
    use crate::section_0150::glue_ord;
}}
