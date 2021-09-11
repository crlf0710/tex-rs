//! @ The global variable |force_eof| is normally |false|; it is set |true|
//! by an \.{\\endinput} command.
//
// @<Glob...@>=
// @!force_eof:boolean; {should the next \.{\\input} be aborted early?}
/// should the next `\input` be aborted early?
#[globals_struct_field(TeXGlobals)]
pub(crate) static force_eof: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use globals_struct::{globals_struct_field, globals_struct_use};
