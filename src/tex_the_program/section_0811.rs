//! ` `

// @<Make the unset node |r| into a |vlist_node| of height |w|...@>=
pub(crate) macro Make_the_unset_node_r_into_a_vlist_node_of_height_w__setting_the_glue_as_if_the_height_were_t($globals:expr, $q:expr, $r:expr, $w:expr, $t:expr) {{
    // begin width(r):=width(q);
    width!($globals, $r) = width!($globals, $q);
    // if t=height(r) then
    if $t == height!($globals, $r) {
        // begin glue_sign(r):=normal; glue_order(r):=normal;
        glue_sign!($globals, $r) = glue_sign::normal as _;
        glue_order!($globals, $r) = glue_ord::normal as _;
        // set_glue_ratio_zero(glue_set(r));
        set_glue_ratio_zero!(glue_set!($globals, $r));
        // end
    }
    // else if t>height(r) then
    else if $t > height!($globals, $r) {
        // begin glue_sign(r):=stretching;
        // if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
        // else glue_set(r):=unfloat((t-height(r))/glue_stretch(r));
        // @^real division@>
        // end
        todo!("t > height(r)");
    }
    // else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
    else {
        // if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
        // else if (glue_order(r)=normal)and(height(r)-t>glue_shrink(r)) then
        //   set_glue_ratio_one(glue_set(r))
        // else glue_set(r):=unfloat((height(r)-t)/glue_shrink(r));
        // end;
        todo!("t < height(r)");
    }
    // height(r):=w; type(r):=vlist_node;
    height!($globals, $r) = $w;
    r#type!($globals, $r) = vlist_node;
    // end
    use crate::section_0109::set_glue_ratio_zero;
    use crate::section_0133::r#type;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0150::glue_ord;
}}
