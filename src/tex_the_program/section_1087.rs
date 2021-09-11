//! @ The height of a `\.{\\vtop}' box is inherited from the first item on its list,
//! if that item is an |hlist_node|, |vlist_node|, or |rule_node|; otherwise
//! the \.{\\vtop} height is zero.
//
// @<Readjust the height...@>=
pub(crate) macro Readjust_the_height_and_depth_of_cur_box_for_vtop($globals:expr) {{
    /// height of box
    let mut h: scaled;
    /// first node in a box
    let p: pointer;
    // begin h:=0; p:=list_ptr(cur_box);
    h = scaled::zero();
    p = list_ptr!($globals, $globals.cur_box);
    // if p<>null then if type(p)<=rule_node then h:=height(p);
    if p != null && r#type!($globals, p) <= rule_node {
        h = height!($globals, p);
    }
    // depth(cur_box):=depth(cur_box)-h+height(cur_box); height(cur_box):=h;
    depth!($globals, $globals.cur_box) =
        depth!($globals, $globals.cur_box) - h + height!($globals, p);
    height!($globals, $globals.cur_box) = h;
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0138::rule_node;
}}
