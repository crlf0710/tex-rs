//! ` `
// @<Determine horizontal glue shrink setting...@>=
macro_rules! Determine_horizontal_glue_shrink_setting__then_return_or_goto_common_ending {
    ($globals:expr, $r:expr, $x:expr, $q:expr, $lbl_common_ending:lifetime) => {{
        /// order of infinity
        let o: glue_ord;
        // begin @<Determine the shrink order@>;
        Determine_the_shrink_order!($globals, o);
        // glue_order(r):=o; glue_sign(r):=shrinking;
        glue_order!($globals, $r) = o as _;
        glue_sign!($globals, $r) = glue_sign::shrinking as _;
        // if total_shrink[o]<>0 then glue_set(r):=unfloat((-x)/total_shrink[o])
        if $globals.total_shrink[o] != scaled::zero() {
            glue_set!($globals, $r) =
                unfloat!(((-$x.inner()) / $globals.total_shrink[o].inner()) as real);
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
        if $globals.total_shrink[o] < -$x
            && o == glue_ord::normal
            && list_ptr!($globals, $r) != null
        {
            // begin last_badness:=1000000;
            $globals.last_badness = 1000000;
            // set_glue_ratio_one(glue_set(r)); {use the maximum shrinkage}
            /// use the maximum shrinkage
            const _: () = ();
            set_glue_ratio_one!(glue_set!($globals, $r));
            // @<Report an overfull hbox and |goto common_ending|, if this box
            //   is sufficiently bad@>;
            Report_an_overfull_hbox_and_goto_common_ending__if_this_box_is_sufficiently_bad!
                ($globals, $x, $q, $lbl_common_ending);
            // end
        }
        // else if o=normal then if list_ptr(r)<>null then
        else if o == glue_ord::normal && list_ptr!($globals, $r) != null {
            // @<Report a tight hbox and |goto common_ending|, if this box
            //   is sufficiently bad@>;
            todo!("report a tight hbox");
        }
        // return;
        return_nojump!($r);
        // end
        use crate::pascal::real;
    }};
}
