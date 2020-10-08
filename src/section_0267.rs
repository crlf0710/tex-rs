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
//!
//! @<Print the font identifier for |font(p)|@>=
//! print_esc(font_id_text(font(p)))
//!
