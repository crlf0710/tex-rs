//! @ Vertical lists that are subject to the |vert_break| procedure should not
//! contain infinite shrinkability, since that would permit any amount of
//! information to ``fit'' on one page.
//
// @<Update the current height and depth measurements with...@>=
pub(crate) macro Update_the_current_height_and_depth_measurements_with_respect_to_a_glue_or_kern_node_p($globals:expr, $p:expr, $prev_dp:expr) {{
    /// glue specifications
    let q;
    // if type(p)=kern_node then q:=p
    if r#type!($globals, $p) == kern_node {
        q = $p;
    }
    // else  begin q:=glue_ptr(p);
    else {
        q = glue_ptr!($globals, $p);
        // active_height[2+stretch_order(q)]:=@|
        //   active_height[2+stretch_order(q)]+stretch(q);@/
        active_height!($globals)[2 + stretch_order!($globals, q)] += stretch!($globals, q);
        // active_height[6]:=active_height[6]+shrink(q);
        active_height!($globals)[6] += shrink!($globals, q);
        // if (shrink_order(q)<>normal)and(shrink(q)<>0) then
        if shrink_order!($globals, q) != glue_ord::normal as _
            && shrink!($globals, q) != scaled::zero()
        {
            // begin@t@>@;@/
            // print_err("Infinite glue shrinkage found in box being split");@/
            // @.Infinite glue shrinkage...@>
            // help4("The box you are \vsplitting contains some infinitely")@/
            //   ("shrinkable glue, e.g., `\vss' or `\vskip 0pt minus 1fil'.")@/
            //   ("Such glue doesn't belong there; but you can safely proceed,")@/
            //   ("since the offensive shrinkability has been made finite.");
            // error; r:=new_spec(q); shrink_order(r):=normal; delete_glue_ref(q);
            // glue_ptr(p):=r; q:=r;
            // end;
            todo!("report error");
        }
        // end;
    }
    // cur_height:=cur_height+prev_dp+width(q); prev_dp:=0
    cur_height!($globals) += $prev_dp + width!($globals, q);
    $prev_dp = scaled::zero();
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0135::width;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::glue_ord;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0155::kern_node;
    use crate::section_0970::active_height;
    use crate::section_0970::cur_height;
}}
