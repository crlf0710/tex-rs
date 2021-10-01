//! @ The |post_break| list of a discretionary node is indicated by a prefixed
//! `\.{\char'174}' instead of the `\..' before the |pre_break| list.
//
// @<Display discretionary |p|@>=
pub(crate) macro Display_discretionary_p($globals:expr, $p:expr) {{
    // begin print_esc("discretionary");
    print_esc($globals, crate::strpool_str!("discretionary"));
    // if replace_count(p)>0 then
    if replace_count!($globals, $p as pointer) > 0 {
        // begin print(" replacing "); print_int(replace_count(p));
        print($globals, crate::strpool_str!(" replacing ").get() as _);
        print_int($globals, replace_count!($globals, $p as pointer) as _);
        // end;
    }
    // node_list_display(pre_break(p)); {recursive call}
    /// recursive call
    node_list_display!($globals, pre_break!($globals, $p as pointer));
    // append_char("|"); show_node_list(post_break(p)); flush_char; {recursive call}
    append_char(
        make_globals_string_view!($globals),
        ASCII_code_literal!(b'|'),
    );
    /// recursive call
    show_node_list($globals, post_break!($globals, $p as pointer) as _);
    flush_char($globals);
    // end
    use crate::section_0004::make_globals_string_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0042::append_char;
    use crate::section_0042::flush_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0115::pointer;
    use crate::section_0145::post_break;
    use crate::section_0145::pre_break;
    use crate::section_0145::replace_count;
    use crate::section_0180::node_list_display;
    use crate::section_0182::show_node_list;
}}
