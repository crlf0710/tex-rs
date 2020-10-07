//! @ Macro abbreviations for output to the terminal and to the log file are
//! defined here for convenience. Some systems need special conventions
//! for terminal output, and it is possible to adhere to those conventions
//! by changing |wterm|, |wterm_ln|, and |wterm_cr| in this section.
//! @^system dependencies@>
//
// @d wterm(#)==write(term_out,#)
pub(crate) fn wterm<T: Display>(globals: &TeXGlobals, val: T) {
    write(&globals.term_out, val);
}
// @d wterm_ln(#)==write_ln(term_out,#)
pub(crate) fn wterm_ln<T: Display>(globals: &TeXGlobals, val: T) {
    write_ln(&globals.term_out, val);
}
// @d wterm_cr==write_ln(term_out)
pub(crate) fn wterm_cr(globals: &TeXGlobals) {
    write_ln_noargs(&globals.term_out);
}
// @d wlog(#)==write(log_file,#)
pub(crate) fn wlog<T: Display>(globals: &TeXGlobals, val: T) {
    write(&globals.log_file, val);
}
// @d wlog_ln(#)==write_ln(log_file,#)
pub(crate) fn wlog_ln<T: Display>(globals: &TeXGlobals, val: T) {
    write_ln(&globals.log_file, val);
}
// @d wlog_cr==write_ln(log_file)
pub(crate) fn wlog_cr(globals: &TeXGlobals) {
    write_ln_noargs(&globals.log_file);
}

use crate::section_0004::TeXGlobals;
use crate::pascal::write;
use crate::pascal::write_ln;
use crate::pascal::write_ln_noargs;
use core::fmt::Display;