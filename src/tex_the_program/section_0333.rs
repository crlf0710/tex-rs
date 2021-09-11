//! @ The value of |par_loc| is the |eqtb| address of `\.{\\par}'. This quantity
//! is needed because a blank line of input is supposed to be exactly equivalent
//! to the appearance of \.{\\par}; we must set |cur_cs:=par_loc|
//! when detecting a blank line.
//
// @<Glob...@>=
// @!par_loc:pointer; {location of `\.{\\par}' in |eqtb|}
/// location of '`\par`' in `eqtb`
#[globals_struct_field(TeXGlobals)]
pub(crate) static par_loc: pointer = pointer::default();
// @!par_token:halfword; {token representing `\.{\\par}'}
/// token representing '`\par`'
#[globals_struct_field(TeXGlobals)]
pub(crate) static par_token: cur_tok_type = cur_tok_type::default();

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
