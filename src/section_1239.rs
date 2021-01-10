//! ` `
// @<Compute the sum of two glue specs@>=
macro_rules! Compute_the_sum_of_two_glue_specs {
    ($globals:expr, $l:expr) => {{
        /// for list manipulation
        let (q, r): (pointer, pointer);
        // begin q:=new_spec(cur_val); r:=equiv(l);
        q = new_spec($globals, $globals.cur_val as _)?;
        r = equiv!($globals, $l);
        // delete_glue_ref(cur_val);
        delete_glue_ref($globals, $globals.cur_val as _);
        // width(q):=width(q)+width(r);
        width!($globals, q) = width!($globals, q) + width!($globals, r);
        // if stretch(q)=0 then stretch_order(q):=normal;
        if stretch!($globals, q) == scaled::zero() {
            stretch_order!($globals, q) = glue_ord::normal as _;
        }
        // if stretch_order(q)=stretch_order(r) then stretch(q):=stretch(q)+stretch(r)
        if stretch_order!($globals, q) == stretch_order!($globals, r) {
            stretch!($globals, q) = stretch!($globals, q) + stretch!($globals, r);
        }
        // else if (stretch_order(q)<stretch_order(r))and(stretch(r)<>0) then
        else if stretch_order!($globals, q) < stretch_order!($globals, r)
            && stretch!($globals, r) != scaled::zero()
        {
            // begin stretch(q):=stretch(r); stretch_order(q):=stretch_order(r);
            stretch!($globals, q) = stretch!($globals, r);
            stretch_order!($globals, q) = stretch_order!($globals, r);
            // end;
        }
        // if shrink(q)=0 then shrink_order(q):=normal;
        if shrink!($globals, q) == scaled::zero() {
            shrink_order!($globals, q) = glue_ord::normal as _;
        }
        // if shrink_order(q)=shrink_order(r) then shrink(q):=shrink(q)+shrink(r)
        if shrink_order!($globals, q) == shrink_order!($globals, r) {
            shrink!($globals, q) = shrink!($globals, q) + shrink!($globals, r);
        }
        // else if (shrink_order(q)<shrink_order(r))and(shrink(r)<>0) then
        else if shrink_order!($globals, q) < shrink_order!($globals, r)
            && shrink!($globals, r) != scaled::zero()
        {
            // begin shrink(q):=shrink(r); shrink_order(q):=shrink_order(r);
            shrink!($globals, q) = shrink!($globals, r);
            shrink_order!($globals, q) = shrink_order!($globals, r);
            // end;
        }
        // cur_val:=q;
        $globals.cur_val = q as _;
        // end
        use crate::section_0101::scaled;
        use crate::section_0150::glue_ord;
        use crate::section_0151::new_spec;
        use crate::section_0201::delete_glue_ref;
    }};
}
