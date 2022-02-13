//! ` `
// @<Append tabskip glue and an empty box to list |u|...@>=
pub(crate) macro Append_tabskip_glue_and_an_empty_box_to_list_u__and_update_s_and_t_as_the_prototype_nodes_are_passed($globals:expr, $p:expr, $s:expr, $t:expr, $u:expr) {{
    /// registers for the list operations
    let v;
    // s:=link(s); v:=glue_ptr(s); link(u):=new_glue(v); u:=link(u);
    $s = link!($globals, $s);
    v = glue_ptr!($globals, $s);
    link!($globals, $u) = new_glue($globals, v)?;
    $u = link!($globals, $u);
    // subtype(u):=tab_skip_code+1; t:=t+width(v);
    subtype!($globals, $u) = tab_skip_code + 1;
    $t = $t + width!($globals, v);
    // if glue_sign(p)=stretching then
    if glue_sign!($globals, $p) == glue_sign::stretching as _ {
        // begin if stretch_order(v)=glue_order(p) then
        if stretch_order!($globals, v) == glue_order!($globals, $p) {
            // t:=t+round(float(glue_set(p))*stretch(v));
            $t = $t
                + scaled::new_from_inner(
                    (float!(glue_set!($globals, $p)) * stretch!($globals, v).inner_real()).round()
                        as _,
                );
            // @^real multiplication@>
        }
        // end
    }
    // else if glue_sign(p)=shrinking then
    else if glue_sign!($globals, $p) == glue_sign::shrinking as _ {
        // begin if shrink_order(v)=glue_order(p) then
        if shrink_order!($globals, v) == glue_order!($globals, $p) {
            // t:=t-round(float(glue_set(p))*shrink(v));
            $t = $t
                - scaled::new_from_inner(
                    (float!(glue_set!($globals, $p)) * shrink!($globals, v).inner_real()).round()
                        as _,
                );
        }
        // end;
    }
    // s:=link(s); link(u):=new_null_box; u:=link(u); t:=t+width(s);
    $s = link!($globals, $s);
    link!($globals, $u) = new_null_box($globals)?;
    $u = link!($globals, $u);
    $t = $t + width!($globals, $s);
    // if mode=-vmode then width(u):=width(s)@+else
    if mode!($globals) == -vmode {
        width!($globals, $u) = width!($globals, $s);
    } else {
        // begin type(u):=vlist_node; height(u):=width(s);
        r#type!($globals, $u) = vlist_node;
        height!($globals, $u) = width!($globals, $s);
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0109::float;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0136::new_null_box;
    use crate::section_0137::vlist_node;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0153::new_glue;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0224::tab_skip_code;
}}
