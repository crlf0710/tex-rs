//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0469($globals:expr, $cmd:expr, $chr_code:expr) {{
    // convert: case chr_code of
    let processed = if $cmd == convert {
        // number_code: print_esc("number");
        if $chr_code.get() == convert_code_kind::number_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("number"));
        }
        // roman_numeral_code: print_esc("romannumeral");
        else if $chr_code.get() == convert_code_kind::roman_numeral_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("romannumeral"));
        }
        // string_code: print_esc("string");
        else if $chr_code.get() == convert_code_kind::string_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("string"));
        }
        // meaning_code: print_esc("meaning");
        else if $chr_code.get() == convert_code_kind::meaning_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("meaning"));
        }
        // font_name_code: print_esc("fontname");
        else if $chr_code.get() == convert_code_kind::font_name_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("fontname"));
        }
        // othercases print_esc("jobname")
        else {
            print_esc($globals, crate::strpool_str!("jobname"));
        }
        // endcases;
        use crate::section_0063::print_esc;
        use crate::section_0297::chr_code_repr;
        use crate::section_0468::convert_code_kind;
        true
    } else {
        false
    };
    use crate::section_0210::*;
    processed
}}
