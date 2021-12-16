//! ` `

// @<Determine vertical glue shrink setting...@>=
pub(crate) macro Determine_vertical_glue_shrink_setting__then_return_or_goto_common_ending($globals:expr, $r:expr, $x:expr, $lbl_common_ending:lifetime) {{
    /// order of infinity
    let o: glue_ord;
    // begin @<Determine the shrink order@>;
    crate::section_0665::Determine_the_shrink_order!($globals, o);
    // glue_order(r):=o; glue_sign(r):=shrinking;
    glue_order!($globals, $r) = o as _;
    glue_sign!($globals, $r) = glue_sign::shrinking as _;
    // if total_shrink[o]<>0 then glue_set(r):=unfloat((-x)/total_shrink[o])
    if $globals.total_shrink[o] != scaled::zero() {
        glue_set!($globals, $r) =
            unfloat!((-$x.inner_real()) / ($globals.total_shrink[o].inner_real()));
    }
    // @^real division@>
    // else  begin glue_sign(r):=normal;
    else {
        glue_sign!($globals, $r) = glue_sign::normal as _;
        // set_glue_ratio_zero(glue_set(r)); {there's nothing to shrink}
        /// there's nothing to shrink
        set_glue_ratio_zero!(glue_set!($globals, $r));
        // end;
    }
    // if (total_shrink[o]<-x)and(o=normal)and(list_ptr(r)<>null) then
    if $globals.total_shrink[o] < -$x && o == glue_ord::normal && list_ptr!($globals, $r) != null {
        // begin last_badness:=1000000;
        // set_glue_ratio_one(glue_set(r)); {use the maximum shrinkage}
        // @<Report an overfull vbox and |goto common_ending|, if this box
        //   is sufficiently bad@>;
        // end
        todo!("shrink 1");
    }
    // else if o=normal then if list_ptr(r)<>null then
    else if o == glue_ord::normal && list_ptr!($globals, $r) != null {
        // @<Report a tight vbox and |goto common_ending|, if this box
        //   is sufficiently bad@>;
        todo!("shrink 2");
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
