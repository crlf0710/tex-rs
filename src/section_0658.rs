//! ` `
// @<Determine horizontal glue stretch setting...@>=
macro_rules! Determine_horizontal_glue_stretch_setting__then_return_or_goto_common_ending {
    ($globals:expr, $r:expr, $x:expr, $lbl_common_ending:lifetime) => {{
        /// order of infinity
        let o: glue_ord;
        // begin @<Determine the stretch order@>;
        Determine_the_stretch_order!($globals, o);
        // glue_order(r):=o; glue_sign(r):=stretching;
        glue_order!($globals, $r) = o as _;
        glue_sign!($globals, $r) = glue_sign::stretching as _;
        // if total_stretch[o]<>0 then glue_set(r):=unfloat(x/total_stretch[o])
        if $globals.total_stretch[o] != scaled::zero() {
            glue_set!($globals, $r) =
                unfloat!(($x.inner() / $globals.total_stretch[o].inner()) as real);
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
            Report_an_underfull_hbox_and_goto_common_ending__if_this_box_is_sufficiently_bad!
                ($globals, $x, $lbl_common_ending);
        }
        // return;
        return_nojump!($r);
        // end
        use crate::pascal::real;
    }};
}
