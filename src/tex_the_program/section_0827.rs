//! ` `
// @<Get ready to start...@>=
pub(crate) macro Get_ready_to_start_line_breaking_0827($globals:expr) {{
    /// miscellaneous nodes of temporary interest
    let (q, r): (pointer, pointer);
    // no_shrink_error_yet:=true;@/
    $globals.no_shrink_error_yet = true;
    // check_shrinkage(left_skip); check_shrinkage(right_skip);@/
    check_shrinkage!($globals, left_skip!($globals));
    check_shrinkage!($globals, right_skip!($globals));
    // q:=left_skip; r:=right_skip; background[1]:=width(q)+width(r);@/
    q = left_skip!($globals);
    r = right_skip!($globals);
    $globals.background[1] = width!($globals, q) + width!($globals, r);
    // background[2]:=0; background[3]:=0; background[4]:=0; background[5]:=0;@/
    $globals.background[2] = scaled::zero();
    $globals.background[3] = scaled::zero();
    $globals.background[4] = scaled::zero();
    $globals.background[5] = scaled::zero();
    // background[2+stretch_order(q)]:=stretch(q);@/
    $globals.background[2 + stretch_order!($globals, q)] = stretch!($globals, q);
    // background[2+stretch_order(r)]:=@|background[2+stretch_order(r)]+stretch(r);@/
    $globals.background[2 + stretch_order!($globals, r)] += stretch!($globals, r);
    // background[6]:=shrink(q)+shrink(r);
    $globals.background[6] = shrink!($globals, q) + shrink!($globals, r);
    use crate::section_0101::scaled;
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0224::left_skip;
    use crate::section_0224::right_skip;
    use crate::section_0825::check_shrinkage;
}}
