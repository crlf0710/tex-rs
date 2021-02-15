//! ` `
// @<Set init...@>=
#[distributed_slice(SET_INIT_KEYVAR)]
fn set_initial_values_of_key_variables_0593(globals: &mut TeXGlobals) {
    // total_pages:=0; max_v:=0; max_h:=0; max_push:=0; last_bop:=-1;
    globals.total_pages = 0;
    globals.max_v = scaled::zero();
    globals.max_h = scaled::zero();
    globals.max_push = 0;
    globals.last_bop = -1;
    // doing_leaders:=false; dead_cycles:=0; cur_s:=-1;
    globals.doing_leaders = false;
    globals.dead_cycles = 0;
    globals.cur_s = -1;
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::SET_INIT_KEYVAR;
use crate::section_0101::scaled;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
