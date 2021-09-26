//! ` `
// @<Compute the discretionary |break...@>=
pub(crate) macro Compute_the_discretionary_break_width_values($globals:expr, $s:expr) {{
    // begin t:=replace_count(cur_p); v:=cur_p; s:=post_break(cur_p);
    let t = replace_count!($globals, $globals.cur_p);
    let v = $globals.cur_p;
    $s = post_break!($globals, $globals.cur_p);
    // while t>0 do
    while t > 0 {
        // begin decr(t); v:=link(v);
        // @<Subtract the width of node |v| from |break_width|@>;
        // end;
        todo!("t>0");
    }
    // while s<>null do
    while $s != null {
        // begin @<Add the width of node |s| to |break_width|@>;
        // s:=link(s);
        // end;
        todo!("s<>null");
    }
    // break_width[1]:=break_width[1]+disc_width;
    $globals.break_width[1] += $globals.disc_width;
    // if post_break(cur_p)=null then s:=link(v);
    //           {nodes may be discardable after the break}
    if post_break!($globals, $globals.cur_p) == null {
        $s = link!($globals, v);
    }
    /// nodes may be discardable after the break
    const _: () = ();
    // end
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0145::post_break;
    use crate::section_0145::replace_count;
}}
