//! @ Of course we had better declare another global variable, if the previous
//! routines are going to work.
//
// @<Glob...@>=
// @!old_setting:0..max_selector;
#[globals_struct_field(TeXGlobals)]
pub(crate) static old_setting: u8_from_0_to_n<max_selector_TYPENUM> = u8_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0054::max_selector_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u8_from_0_to_n;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
