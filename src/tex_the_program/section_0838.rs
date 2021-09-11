//! ` `
// @<Subtract glue from |break...@>=
pub(crate) macro Subtract_glue_from_break_width($globals:expr, $s:expr) {{
    /// points to a glue specification or a node ahead of `cur_p`
    let v: pointer;
    // begin v:=glue_ptr(s); break_width[1]:=break_width[1]-width(v);
    v = glue_ptr!($globals, $s);
    $globals.break_width[1] -= width!($globals, v);
    // break_width[2+stretch_order(v)]:=break_width[2+stretch_order(v)]-stretch(v);
    $globals.break_width[2 + stretch_order!($globals, v)] -= stretch!($globals, v);
    // break_width[6]:=break_width[6]-shrink(v);
    $globals.break_width[6] -= shrink!($globals, v);
    // end
    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0149::glue_ptr;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
}}
