//! ` `

// @<Negate all three...@>=
pub(crate) macro Negate_all_three_glue_components_of_cur_val($globals:expr) {{
    // begin negate(width(cur_val));
    negate!(width!($globals, $globals.cur_val as pointer));
    // negate(stretch(cur_val));
    negate!(stretch!($globals, $globals.cur_val as pointer));
    // negate(shrink(cur_val));
    negate!(shrink!($globals, $globals.cur_val as pointer));
    // end
    use crate::section_0016::negate;
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
}}
