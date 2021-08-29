//! ` `
// @<Set init...@>=
#[distributed_slice(SET_INIT_KEYVAR)]
fn set_initial_values_of_key_variables_0521(globals: &mut TeXGlobals) {
    // TEX_format_default:='TeXformats:plain.fmt';
    for (idx, ch) in " TeXformats:plain.fmt".chars().enumerate().skip(1) {
        globals.TEX_format_default[idx as u16] = xchr(ASCII_code::from(ch as u32 as integer));
    }
    // @.TeXformats@>
    // @.plain@>
    // @^system dependencies@>
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0008::SET_INIT_KEYVAR;
use crate::section_0018::ASCII_code;
use crate::section_0020::xchr;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
