//! ` `
// @<Show the font identifier in |eqtb[n]|@>=
#[cfg(feature = "statistics")]
pub(crate) macro Show_the_font_identifier_in_eqtb_n($globals:expr, $n:expr) {{
    // begin if n=cur_font_loc then print("current font")
    if ($n as integer) == cur_font_loc as integer {
        print($globals, crate::strpool_str!("current font").get() as _);
    }
    // else if n<math_font_base+16 then
    else if ($n as integer) < math_font_base as integer + 16 {
        // begin print_esc("textfont"); print_int(n-math_font_base);
        // end
        todo!("textfont");
    }
    // else if n<math_font_base+32 then
    else if ($n as integer) < math_font_base as integer + 32 {
        // begin print_esc("scriptfont"); print_int(n-math_font_base-16);
        // end
        todo!("scriptfont");
    }
    // else  begin print_esc("scriptscriptfont"); print_int(n-math_font_base-32);
    else {
        todo!("scriptscriptfont");
        // end;
    }
    // print_char("=");@/
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    // print_esc(hash[font_id_base+equiv(n)].rh);
    //   {that's |font_id_text(equiv(n))|}
    /// that's `font_id_text(equiv(n))`
    print_esc(
        $globals,
        str_number::new(
            $globals.hash[u16_from_m_to_n::from(font_id_base as u16 + equiv!($globals, $n))]
                [TWO_HALVES_RH] as _,
        ),
    );
    // end
    use crate::pascal::integer;
    use crate::pascal::u16_from_m_to_n;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0038::str_number;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0113::TWO_HALVES_RH;
    use crate::section_0221::equiv;
    use crate::section_0222::font_id_base;
    use crate::section_0230::cur_font_loc;
    use crate::section_0230::math_font_base;
}}

#[cfg(not(feature = "statistics"))]
crate::submit_strpool_str!("current font");
