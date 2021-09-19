//! @ The |scan_optional_equals| routine looks for an optional `\.=' sign preceded
//! by optional spaces; `\.{\\relax}' is not ignored here.
//
// @p procedure scan_optional_equals;
#[allow(unused_variables)]
#[cfg_attr(feature = "trace_verbose", tracing::instrument(level = "trace"))]
pub(crate) fn scan_optional_equals(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin  @<Get the next non-blank non-call token@>;
    crate::section_0406::Get_the_next_non_blank_non_call_token!(globals);
    // if cur_tok<>other_token+"=" then back_input;
    if globals.cur_tok.get() != other_token + b'=' as cur_tok_repr {
        back_input(globals);
    }
    // end;
    crate::return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0289::other_token;
use crate::section_0297::cur_tok_repr;
use crate::section_0325::back_input;
