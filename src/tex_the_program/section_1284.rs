//! @ The |error| routine calls on |give_err_help| if help is requested from
//! the |err_help| parameter.
//
// @p procedure give_err_help;
pub(crate) fn give_err_help(globals: &mut TeXGlobals) {
    // begin token_show(err_help);
    token_show(globals, err_help!(globals));
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0295::token_show;
