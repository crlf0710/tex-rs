//! ` `

// @<Update the current page measurements with respect to the glue...@>=
macro_rules! Update_the_current_page_measurements_with_respect_to_the_glue_or_kern_specified_by_node_p {
    ($globals:expr, $p:expr) => {{
        /// nodes being examined
        let q: pointer;
        // if type(p)=kern_node then q:=p
        if r#type!($globals, $p) == kern_node {
            q = $p;
        }
        // else begin q:=glue_ptr(p);
        else {
            q = glue_ptr!($globals, $p);
            // page_so_far[2+stretch_order(q)]:=@|
            //   page_so_far[2+stretch_order(q)]+stretch(q);@/
            $globals.page_so_far[2 + stretch_order!($globals, q) as usize] += stretch!($globals, q);
            // page_shrink:=page_shrink+shrink(q);
            page_shrink!($globals) += shrink!($globals, q);
            // if (shrink_order(q)<>normal)and(shrink(q)<>0) then
            if shrink_order!($globals, q) as integer != glue_ord::normal as integer
                && shrink!($globals, q) != scaled::zero()
            {
                todo!("infinite shrinkage");
                // begin@t@>@;@/
                // print_err("Infinite glue shrinkage found on current page");@/
                // @.Infinite glue shrinkage...@>
                // help4("The page about to be output contains some infinitely")@/
                //   ("shrinkable glue, e.g., `\vss' or `\vskip 0pt minus 1fil'.")@/
                //   ("Such glue doesn't belong there; but you can safely proceed,")@/
                //   ("since the offensive shrinkability has been made finite.");
                // error;
                // r:=new_spec(q); shrink_order(r):=normal; delete_glue_ref(q);
                // glue_ptr(p):=r; q:=r;
                // end;
            }
            // end;
        }
        // page_total:=page_total+page_depth+width(q); page_depth:=0
        page_total!($globals) += page_depth!($globals) + width!($globals, q);
        page_depth!($globals) = scaled::zero();
        use crate::section_0101::scaled;
        use crate::section_0150::glue_ord;
    }};
}
