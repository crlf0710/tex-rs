//! @ When we begin to process a new \.{\\if}, we set |if_limit:=if_code|; then
//! if\/ \.{\\or} or \.{\\else} or \.{\\fi} occurs before the current \.{\\if}
//! condition has been evaluated, \.{\\relax} will be inserted.
//! For example, a sequence of commands like `\.{\\ifvoid1\\else...\\fi}'
//! would otherwise require something after the `\.1'.
//
// @<Push the condition stack@>=
pub(crate) macro Push_the_condition_stack($globals:expr) {{
    let p: pointer;
    // begin p:=get_node(if_node_size); link(p):=cond_ptr; type(p):=if_limit;
    p = get_node($globals, if_node_size)?;
    link!($globals, p) = $globals.cond_ptr;
    r#type!($globals, p) = $globals.if_limit.get();
    // subtype(p):=cur_if; if_line_field(p):=if_line;
    subtype!($globals, p) = $globals.cur_if.get();
    if_line_field!($globals, p) = $globals.if_line;
    // cond_ptr:=p; cur_if:=cur_chr; if_limit:=if_code; if_line:=line;
    $globals.cond_ptr = p;
    $globals.cur_if = ($globals.cur_chr.get() as u8).into();
    $globals.if_limit = if_code.into();
    $globals.if_line = $globals.line as _;
    // end
    use crate::section_0115::*;
    use crate::section_0118::link;
    use crate::section_0125::get_node;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0489::if_code;
    use crate::section_0489::if_line_field;
    use crate::section_0489::if_node_size;
}}
