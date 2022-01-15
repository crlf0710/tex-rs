//! ` `

// @<Display leaders |p|@>=
pub(crate) macro Display_leaders_p($globals:expr, $p:expr) {{
    // begin print_esc("");
    print_esc($globals, crate::strpool_str!(""));
    // if subtype(p)=c_leaders then print_char("c")
    if subtype!($globals, $p as pointer) == glue_node_subtype::c_leaders as _ {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'c'),
        );
    }
    // else if subtype(p)=x_leaders then print_char("x");
    else if subtype!($globals, $p as pointer) == glue_node_subtype::x_leaders as _ {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'x'),
        );
    }
    // print("leaders "); print_spec(glue_ptr(p),0);
    print($globals, crate::strpool_str!("leaders ").get() as _);
    print_spec(
        $globals,
        glue_ptr!($globals, $p as pointer) as _,
        str_number::new(0),
    );
    // node_list_display(leader_ptr(p)); {recursive call}
    /// recursive call
    node_list_display!($globals, leader_ptr!($globals, $p as pointer));
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0038::str_number;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0149::leader_ptr;
    use crate::section_0178::print_spec;
    use crate::section_0180::node_list_display;
}}
