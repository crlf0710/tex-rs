//! @ The variable |write_loc| just introduced is used to provide an
//! appropriate error message in case of ``runaway'' write texts.
//
// @<Glob...@>=
// @!write_loc:pointer; {|eqtb| address of \.{\\write}}
/// `eqtb` address of `\write`
#[globals_struct_field(TeXGlobals)]
pub(crate) static write_loc: pointer = pointer::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
