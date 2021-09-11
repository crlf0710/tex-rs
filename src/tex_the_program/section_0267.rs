//! @ We will deal with the other primitives later, at some point in the program
//! where their |eq_type| and |equiv| values are more meaningful.  For example,
//! the primitives for math mode will be loaded when we consider the routines
//! that deal with formulas. It is easy to find where each particular
//! primitive was treated by looking in the index at the end; for example, the
//! section where |"radical"| entered |eqtb| is listed under `\.{\\radical}
//! primitive'. (Primitives consisting of a single nonalphabetic character,
//! @!like `\.{\\/}', are listed under `Single-character primitives'.)
//! @!@^Single-character primitives@>
//!
//! Meanwhile, this is a convenient place to catch up on something we were unable
//! to do before the hash table was defined:
//
// @<Print the font identifier for |font(p)|@>=
pub(crate) macro Print_the_font_identifier_for_font_p($globals:expr, $p:expr) {{
    // print_esc(font_id_text(font(p)))
    let f = font!($globals, $p);
    let t = font_id_text!($globals, f.get());
    print_esc($globals, str_number::new(t as _));
    use crate::section_0038::str_number;
    use crate::section_0063::print_esc;
    use crate::section_0134::font;
    use crate::section_0256::font_id_text;
}}
