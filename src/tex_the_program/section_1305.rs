//! @ Format files consist of |memory_word| items, and we use the following
//! macros to dump words of different types:
//
// @d dump_wd(#)==begin fmt_file^:=#; put(fmt_file);@+end
macro_rules! dump_wd {
    ($globals:expr, $val:expr) => {{
        let wd = $val;
        buffer_variable_assign(&mut $globals.fmt_file, wd);
        put(&mut $globals.fmt_file);
        use crate::pascal::buffer_variable_assign;
        use crate::pascal::put;
    }}
}
// @d dump_int(#)==begin fmt_file^.int:=#; put(fmt_file);@+end
macro_rules! dump_int {
    ($globals:expr, $val:expr) => {{
        let mut wd = memory_word::default();
        wd[MEMORY_WORD_INT] = $val;
        buffer_variable_assign(&mut $globals.fmt_file, wd);
        put(&mut $globals.fmt_file);
        use crate::pascal::buffer_variable_assign;
        use crate::pascal::put;
        use crate::section_0113::MEMORY_WORD_INT;
        use crate::section_0113::memory_word;
    }}
}
// @d dump_hh(#)==begin fmt_file^.hh:=#; put(fmt_file);@+end
macro_rules! dump_hh {
    ($globals:expr, $val:expr) => {{
        let mut wd = memory_word::default();
        wd[MEMORY_WORD_HH] = $val;
        buffer_variable_assign(&mut $globals.fmt_file, wd);
        put(&mut $globals.fmt_file);
        use crate::pascal::buffer_variable_assign;
        use crate::pascal::put;
        use crate::section_0113::MEMORY_WORD_HH;
        use crate::section_0113::memory_word;
    }}
}
// @d dump_qqqq(#)==begin fmt_file^.qqqq:=#; put(fmt_file);@+end
macro_rules! dump_qqqq {
    ($globals:expr, $val:expr) => {{
        let mut wd = memory_word::default();
        wd[MEMORY_WORD_QQQQ] = $val;
        buffer_variable_assign(&mut $globals.fmt_file, wd);
        put(&mut $globals.fmt_file);
        use crate::pascal::buffer_variable_assign;
        use crate::pascal::put;
        use crate::section_0113::MEMORY_WORD_QQQQ;
        use crate::section_0113::memory_word;
    }}
}
//
// @<Glob...@>=
// @!fmt_file:word_file; {for input or output of format information}
/// for input or output of format information
#[globals_struct_field(TeXGlobals)]
pub(crate) static fmt_file: word_file = word_file::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::word_file;

use globals_struct::{globals_struct_field, globals_struct_use};
