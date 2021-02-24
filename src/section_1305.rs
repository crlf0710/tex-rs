//! @ Format files consist of |memory_word| items, and we use the following
//! macros to dump words of different types:
//
// @d dump_wd(#)==begin fmt_file^:=#; put(fmt_file);@+end
// @d dump_int(#)==begin fmt_file^.int:=#; put(fmt_file);@+end
// @d dump_hh(#)==begin fmt_file^.hh:=#; put(fmt_file);@+end
// @d dump_qqqq(#)==begin fmt_file^.qqqq:=#; put(fmt_file);@+end
//
// @<Glob...@>=
// @!fmt_file:word_file; {for input or output of format information}
/// for input or output of format information
#[globals_struct_field(TeXGlobals)]
pub(crate) static fmt_file: word_file = word_file::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::word_file;

use globals_struct::{globals_struct_field, globals_struct_use};
