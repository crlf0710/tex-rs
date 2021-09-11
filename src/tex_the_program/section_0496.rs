//! ` `

// @<Pop the condition stack@>=
pub(crate) macro Pop_the_condition_stack($globals:expr) {{
    let p: pointer;
    // begin p:=cond_ptr; if_line:=if_line_field(p);
    p = $globals.cond_ptr;
    $globals.if_line = if_line_field!($globals, p);
    // cur_if:=subtype(p); if_limit:=type(p); cond_ptr:=link(p);
    $globals.cur_if = subtype!($globals, p).into();
    $globals.if_limit = r#type!($globals, p).into();
    $globals.cond_ptr = link!($globals, p);
    // free_node(p,if_node_size);
    free_node($globals, p, if_node_size as _);
    // end
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0489::if_line_field;
    use crate::section_0489::if_node_size;
}}
