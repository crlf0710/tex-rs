//! ` `

// @<Glob...@>=
// @!hyph_word:array[hyph_pointer] of str_number; {exception words}
/// exception words
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyph_word: Box<hyph_pointer_array<str_number>> =
    Box::new(hyph_pointer_array::default());
// @!hyph_list:array[hyph_pointer] of pointer; {lists of hyphen positions}
/// lists of hyphen positions
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyph_list: Box<hyph_pointer_array<pointer>> =
    Box::new(hyph_pointer_array::default());
// @!hyph_count:hyph_pointer; {the number of words in the exception dictionary}
/// the number of words in the exception dictionary
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyph_count: hyph_pointer = hyph_pointer::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0925::hyph_pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0925::hyph_pointer_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
