//! @ We move the post-break list from inside node |q| to the main list by
//! re\-attaching it just before the present node |r|, then resetting |r|.
//
// @<Transplant the post-break list@>=
pub(crate) macro Transplant_the_post_break_list($globals:expr, $q:expr, $r:expr, $post_disc_break:expr) {{
    /// temporary registers for list manipulation
    let mut s;
    // begin s:=post_break(q);
    s = post_break!($globals, $q);
    // while link(s)<>null do s:=link(s);
    while link!($globals, s) != null {
        s = link!($globals, s);
    }
    // link(s):=r; r:=post_break(q); post_break(q):=null; post_disc_break:=true;
    link!($globals, s) = $r;
    $r = post_break!($globals, $q);
    post_break!($globals, $q) = null;
    $post_disc_break = true;
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::post_break;
}}
