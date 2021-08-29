//! ` `
// @<Set init...@>=
#[distributed_slice(SET_INIT_KEYVAR)]
fn set_initial_values_of_key_variables_0254(globals: &mut TeXGlobals) {
    // for k:=int_base to eqtb_size do xeq_level[k]:=level_one;
    for k in int_base..=eqtb_size {
        let k = k as pointer;
        globals.xeq_level[k] = level_one;
    }
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::SET_INIT_KEYVAR;
use crate::section_0115::pointer;
use crate::section_0221::level_one;
use crate::section_0230::int_base;
use crate::section_0247::eqtb_size;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
