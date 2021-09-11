//! ` `
// @<Output the font definitions...@>=
pub(crate) macro Output_the_font_definitions_for_all_fonts_that_were_used($globals:expr) {{
    // while font_ptr>font_base do
    while $globals.font_ptr.get() as integer > font_base as integer {
        // begin if font_used[font_ptr] then dvi_font_def(font_ptr);
        if $globals.font_used[$globals.font_ptr] {
            dvi_font_def($globals, $globals.font_ptr);
        }
        // decr(font_ptr);
        decr!($globals.font_ptr);
        // end
    }
    use crate::pascal::integer;
    use crate::section_0012::font_base;
    use crate::section_0016::decr;
    use crate::section_0602::dvi_font_def;
}}
