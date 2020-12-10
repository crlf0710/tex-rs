//! @ The presence of `\.{\\immediate}' causes the |do_extension| procedure
//! to descend to one level of recursion. Nothing happens unless \.{\\immediate}
//! is followed by `\.{\\openout}', `\.{\\write}', or `\.{\\closeout}'.
//! @^recursion@>
//
// @<Implement \.{\\immediate}@>=
macro_rules! Implement_immediate {
    ($globals:expr) => {{
        // begin get_x_token;
        get_x_token($globals)?;
        // if (cur_cmd=extension)and(cur_chr<=close_node) then
        if $globals.cur_cmd == extension && $globals.cur_chr.get() <= close_node as _ {
            todo!("immediate");
            // begin p:=tail; do_extension; {append a whatsit node}
            // out_what(tail); {do the action immediately}
            // flush_node_list(tail); tail:=p; link(p):=null;
            // end
        }
        // else back_input;
        else {
            back_input($globals);
        }
        // end
        use crate::section_0208::extension;
        use crate::section_0325::back_input;
        use crate::section_0380::get_x_token;
        use crate::section_1341::close_node;
    }}
}
