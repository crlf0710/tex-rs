//! ` `
//! The unset box |q| represents a row that contains one or more unset boxes,
//! depending on how soon \.{\\cr} occurred in that row.
//
// @<Set the unset box |q| and the unset boxes in it@>=
pub(crate) macro Set_the_unset_box_q_and_the_unset_boxes_in_it($globals:expr, $p:expr, $q:expr, $o:expr) {{
    /// registers for the list operations
    let (mut r, mut s);
    // begin if mode=-vmode then
    if mode!($globals) == -vmode {
        // begin type(q):=hlist_node; width(q):=width(p);
        r#type!($globals, $q) = hlist_node;
        width!($globals, $q) = width!($globals, $p);
        // end
    }
    // else  begin type(q):=vlist_node; height(q):=height(p);
    else {
        r#type!($globals, $q) = vlist_node;
        height!($globals, $q) = height!($globals, $p);
        // end;
    }
    // glue_order(q):=glue_order(p); glue_sign(q):=glue_sign(p);
    glue_order!($globals, $q) = glue_order!($globals, $p);
    glue_sign!($globals, $q) = glue_sign!($globals, $p);
    // glue_set(q):=glue_set(p); shift_amount(q):=o;
    glue_set!($globals, $q) = glue_set!($globals, $p);
    shift_amount!($globals, $q) = $o;
    // r:=link(list_ptr(q)); s:=link(list_ptr(p));
    r = link!($globals, list_ptr!($globals, $q));
    s = link!($globals, list_ptr!($globals, $p));
    // repeat @<Set the glue in node |r| and change it from an unset node@>;
    loop {
        crate::section_0808::Set_the_glue_in_node_r_and_change_it_from_an_unset_node!(
            $globals, $q, r, s
        );
        // r:=link(link(r)); s:=link(link(s));
        r = link!($globals, link!($globals, r));
        s = link!($globals, link!($globals, s));
        // until r=null;
        if r == null {
            break;
        }
    }
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::height;
    use crate::section_0135::hlist_node;
    use crate::section_0135::list_ptr;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
}}
