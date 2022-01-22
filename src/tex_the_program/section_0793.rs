//! ` `

// @<Lengthen the preamble...@>=
pub(crate) macro Lengthen_the_preamble_periodically($globals:expr, $p:expr, $q:expr) {{
    // begin link(q):=new_null_box; p:=link(q); {a new alignrecord}
    /// a new alignrecord
    const _: () = ();
    link!($globals, $q) = new_null_box($globals)?;
    $p = link!($globals, $q);
    // info(p):=end_span; width(p):=null_flag; cur_loop:=link(cur_loop);
    info_inner!($globals, $p) = end_span;
    width!($globals, $p) = null_flag;
    $globals.cur_loop = link!($globals, $globals.cur_loop);
    // @<Copy the templates from node |cur_loop| into node |p|@>;
    crate::section_0794::Copy_the_templates_from_node_cur_loop_into_node_p!($globals, $p);
    // cur_loop:=link(cur_loop);
    $globals.cur_loop = link!($globals, $globals.cur_loop);
    // link(p):=new_glue(glue_ptr(cur_loop));
    let x = new_glue($globals, glue_ptr!($globals, $globals.cur_loop))?;
    link!($globals, $p) = x;
    // subtype(link(p)):=tab_skip_code+1;
    subtype!($globals, x) = tab_skip_code + 1;
    // end

    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0135::width;
    use crate::section_0136::new_null_box;
    use crate::section_0138::null_flag;
    use crate::section_0149::glue_ptr;
    use crate::section_0153::new_glue;
    use crate::section_0162::end_span;
    use crate::section_0224::tab_skip_code;
}}
