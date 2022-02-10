//! @ Merging of two span-node lists is a typical exercise in the manipulation of
//! linearly linked data structures. The essential invariant in the following
//! |repeat| loop is that we want to dispense with node |r|, in |q|'s list,
//! and |u| is its successor; all nodes of |p|'s list up to and including |s|
//! have been processed, and the successor of |s| matches |r| or precedes |r|
//! or follows |r|, according as |link(r)=n| or |link(r)>n| or |link(r)<n|.
//
// @<Merge the widths...@>=
pub(crate) macro Merge_the_widths_in_the_span_nodes_of_q_with_those_of_p__destroying_the_span_nodes_of_q($globals:expr, $p:expr, $q:expr) {{
    /// registers for the list operations
    let (mut r, mut s, mut u);
    /// width of column
    let t;
    /// matching span amount
    let mut n;
    // begin t:=width(q)+width(glue_ptr(link(q)));
    t = width!($globals, $q) + width!($globals, glue_ptr!($globals, link!($globals, $q)));
    // r:=info(q); s:=end_span; info(s):=p; n:=min_quarterword+1;
    r = info_inner!($globals, $q);
    s = end_span;
    info_inner!($globals, s) = $p;
    n = (min_quarterword + 1) as halfword;
    // repeat width(r):=width(r)-t; u:=info(r);
    loop {
        width!($globals, r) -= t;
        u = info_inner!($globals, r);
        // while link(r)>n do
        while link!($globals, r) > n {
            // begin s:=info(s); n:=link(info(s))+1;
            s = info_inner!($globals, s);
            n = link!($globals, info_inner!($globals, s)) + 1;
            // end;
        }
        // if link(r)<n then
        if link!($globals, r) < n {
            // begin info(r):=info(s); info(s):=r; decr(link(r)); s:=r;
            info_inner!($globals, r) = info_inner!($globals, s);
            info_inner!($globals, s) = r;
            decr!(link!($globals, r));
            s = r;
            // end
        }
        // else  begin if width(r)>width(info(s)) then width(info(s)):=width(r);
        else {
            let info_s = info_inner!($globals, s);
            if width!($globals, r) > width!($globals, info_s) {
                width!($globals, info_s) = width!($globals, r);
            }
            // free_node(r,span_node_size);
            free_node($globals, r, span_node_size as _);
            // end;
        }
        // r:=u;
        r = u;
        // until r=end_span;
        if r == end_span {
            break;
        }
    }
    // end
    use crate::section_0016::decr;
    use crate::section_0110::min_quarterword;
    use crate::section_0113::halfword;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0135::width;
    use crate::section_0149::glue_ptr;
    use crate::section_0162::end_span;
    use crate::section_0797::span_node_size;
}}
