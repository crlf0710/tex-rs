//! ` `
//! When the data for a discretionary break is being displayed, we will have
//! printed the |pre_break| and |post_break| lists; we want to skip over the
//! third list, so that the discretionary data will not appear twice.  The
//! following code is performed at the very end of |try_break|.
//
// @<Update the value of |printed_node|...@>=
pub(crate) macro Update_the_value_of_printed_node_for_symbolic_displays($globals:expr) {{
    // if cur_p=printed_node then if cur_p<>null then if type(cur_p)=disc_node then
    if $globals.cur_p == $globals.printed_node
        && $globals.cur_p != null
        && r#type!($globals, $globals.cur_p) == disc_node
    {
        /// node count, if `cur_p` is a discretionary node
        let mut t;

        // begin t:=replace_count(cur_p);
        t = replace_count!($globals, $globals.cur_p);
        // while t>0 do
        while t > 0 {
            // begin decr(t); printed_node:=link(printed_node);
            decr!(t);
            $globals.printed_node = link!($globals, $globals.printed_node);
            // end;
        }
        // end
    }
    use crate::section_0016::decr;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0145::disc_node;
    use crate::section_0145::replace_count;
}}
