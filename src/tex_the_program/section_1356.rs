//! ` `
// @<Display the whatsit...@>=
pub(crate) macro Display_the_whatsit_node_p($globals:expr, $p:expr) {{
    // case subtype(p) of
    let subtype_p = subtype!($globals, $p as pointer);
    // open_node:begin print_write_whatsit("openout",p);
    if subtype_p == open_node {
        todo!("open_node");
        // print_char("="); print_file_name(open_name(p),open_area(p),open_ext(p));
        // end;
    }
    // write_node:begin print_write_whatsit("write",p);
    else if subtype_p == write_node {
        print_write_whatsit($globals, crate::strpool_str!("write"), $p as pointer);
        // print_mark(write_tokens(p));
        print_mark($globals, write_tokens!($globals, $p as pointer) as _);
        // end;
    }
    // close_node:print_write_whatsit("closeout",p);
    else if subtype_p == close_node {
        todo!("close_node");
    }
    // special_node:begin print_esc("special");
    else if subtype_p == special_node {
        todo!("special_node");
        // print_mark(write_tokens(p));
        // end;
    }
    // language_node:begin print_esc("setlanguage");
    else if subtype_p == language_node {
        print_esc($globals, crate::strpool_str!("setlanguage"));
        // print_int(what_lang(p)); print(" (hyphenmin ");
        print_int($globals, what_lang!($globals, $p as pointer) as _);
        print($globals, crate::strpool_str!(" (hyphenmin ").get() as _);
        // print_int(what_lhm(p)); print_char(",");
        print_int($globals, what_lhm!($globals, $p as pointer) as _);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b','),
        );
        // print_int(what_rhm(p)); print_char(")");
        print_int($globals, what_rhm!($globals, $p as pointer) as _);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b')'),
        );
        // end;
    }
    // othercases print("whatsit?")
    else {
        print($globals, crate::strpool_str!("whatsit?").get() as _);
    }
    // endcases
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0176::print_mark;
    use crate::section_1341::close_node;
    use crate::section_1341::language_node;
    use crate::section_1341::open_node;
    use crate::section_1341::special_node;
    use crate::section_1341::what_lang;
    use crate::section_1341::what_lhm;
    use crate::section_1341::what_rhm;
    use crate::section_1341::write_node;
    use crate::section_1341::write_tokens;
    use crate::section_1355::print_write_whatsit;
}}
