//! ` `

// @<Make the unset node |r| into an |hlist_node| of width |w|...@>=
pub(crate) macro Make_the_unset_node_r_into_an_hlist_node_of_width_w__setting_the_glue_as_if_the_width_were_t($globals:expr, $q:expr, $r:expr, $w:expr, $t:expr) {{
    // begin height(r):=height(q); depth(r):=depth(q);
    height!($globals, $r) = height!($globals, $q);
    depth!($globals, $r) = depth!($globals, $q);
    // if t=width(r) then
    if $t == width!($globals, $r) {
        // begin glue_sign(r):=normal; glue_order(r):=normal;
        glue_sign!($globals, $r) = glue_sign::normal as _;
        glue_order!($globals, $r) = glue_ord::normal as _;
        // set_glue_ratio_zero(glue_set(r));
        set_glue_ratio_zero!(glue_set!($globals, $r));
        // end
    }
    // else if t>width(r) then
    else if $t > width!($globals, $r) {
        // begin glue_sign(r):=stretching;
        glue_sign!($globals, $r) = glue_sign::stretching as _;
        // if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
        if glue_stretch!($globals, $r) == scaled::zero() {
            set_glue_ratio_zero!(glue_set!($globals, $r));
        }
        // else glue_set(r):=unfloat((t-width(r))/glue_stretch(r));
        else {
            glue_set!($globals, $r) = unfloat!(
                ($t - width!($globals, $r)).inner_real() / glue_stretch!($globals, $r).inner_real()
            );
            // @^real division@>
        }
        // end
    }
    // else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
    else {
        // if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
        // else if (glue_order(r)=normal)and(width(r)-t>glue_shrink(r)) then
        //   set_glue_ratio_one(glue_set(r))
        // else glue_set(r):=unfloat((width(r)-t)/glue_shrink(r));
        // end;
        todo!("t<width(r)");
    }
    // width(r):=w; type(r):=hlist_node;
    width!($globals, $r) = $w;
    r#type!($globals, $r) = hlist_node;
    // end
    use crate::section_0101::scaled;
    use crate::section_0109::set_glue_ratio_zero;
    use crate::section_0109::unfloat;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::height;
    use crate::section_0135::hlist_node;
    use crate::section_0135::width;
    use crate::section_0150::glue_ord;
    use crate::section_0159::glue_stretch;
}}
