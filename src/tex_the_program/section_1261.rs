//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1261($globals:expr, $cmd:expr, $chr_code:expr) {{
    // set_font:begin print("select font "); slow_print(font_name[chr_code]);
    let processed = if $cmd == set_font {
        print($globals, crate::strpool_str!("select font ").get() as _);
        let f = internal_font_number::new($chr_code.get() as _);
        slow_print($globals, $globals.font_name[f].get() as _);
        // if font_size[chr_code]<>font_dsize[chr_code] then
        if $globals.font_size[f] != $globals.font_dsize[f] {
            // begin print(" at "); print_scaled(font_size[chr_code]);
            print($globals, crate::strpool_str!(" at ").get() as _);
            print_scaled($globals, $globals.font_size[f]);
            // print("pt");
            print($globals, crate::strpool_str!("pt").get() as _);
            // end;
        }
        // end;
        use crate::section_0060::slow_print;
        use crate::section_0103::print_scaled;
        use crate::section_0548::internal_font_number;
        true
    } else {
        false
    };
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0209::*;
    processed
}}
