//! ` `
// @<Declare action...@>=
// procedure append_penalty;
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn append_penalty(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // begin scan_int; tail_append(new_penalty(cur_val));
    scan_int(globals)?;
    tail_append!(globals, new_penalty(globals, globals.cur_val)?);
    // if mode=vmode then build_page;
    if mode!(globals) == vmode {
        build_page(globals);
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0158::new_penalty;
use crate::section_0211::vmode;
use crate::section_0440::scan_int;
use crate::section_0994::build_page;
