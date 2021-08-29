//! ` `

// @<Set init...@>=interaction:=error_stop_mode;
#[distributed_slice(SET_INIT_KEYVAR)]
fn set_initial_values_of_key_variables_0074(globals: &mut TeXGlobals) {
    globals.interaction = error_stop_mode.into();
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::SET_INIT_KEYVAR;
use crate::section_0073::error_stop_mode;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
