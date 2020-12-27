//! @ The \.{\\afterassignment} command puts a token into the global
//! variable |after_token|. This global variable is examined just after
//! every assignment has been performed.
//
// @<Glob...@>=
// @!after_token:halfword; {zero, or a saved token}
/// zero, or a saved token
#[globals_struct_field(TeXGlobals)]
pub(crate) static after_token: cur_tok_type = cur_tok_type::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0297::cur_tok_type;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
