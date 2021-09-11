//! ` `
// @<Display ligature |p|@>=
pub(crate) macro Display_ligature_p($globals:expr, $p:expr) {{
    // begin print_font_and_char(lig_char(p)); print(" (ligature ");
    print_font_and_char($globals, lig_char!($p));
    print($globals, crate::strpool_str!(" (ligature ").get() as _);
    // if subtype(p)>1 then print_char("|");
    if subtype!($globals, $p as pointer) > 1 {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'|'),
        );
    }
    // font_in_short_display:=font(lig_char(p)); short_display(lig_ptr(p));
    $globals.font_in_short_display = font!($globals, lig_char!($p as pointer)).get() as _;
    short_display($globals, lig_ptr!($globals, $p as pointer) as _);
    // if odd(subtype(p)) then print_char("|");
    if subtype!($globals, $p as pointer).is_odd() {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'|'),
        );
    }
    // print_char(")");
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b')'),
    );
    // end
    use crate::pascal::IsOddOrEven;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0134::font;
    use crate::section_0143::lig_char;
    use crate::section_0143::lig_ptr;
    use crate::section_0174::short_display;
    use crate::section_0176::print_font_and_char;
}}
