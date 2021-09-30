//! ` `

// @<Display rule |p|@>=
pub(crate) macro Display_rule_p($globals:expr, $p:expr) {{
    // begin print_esc("rule("); print_rule_dimen(height(p)); print_char("+");
    print_esc($globals, crate::strpool_str!("rule("));
    print_rule_dimen($globals, height!($globals, $p as pointer));
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'+'),
    );
    // print_rule_dimen(depth(p)); print(")x"); print_rule_dimen(width(p));
    print_rule_dimen($globals, depth!($globals, $p as pointer));
    print($globals, crate::strpool_str!(")x").get() as _);
    print_rule_dimen($globals, width!($globals, $p as pointer));
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0115::pointer;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0176::print_rule_dimen;
}}
