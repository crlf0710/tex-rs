//! @ When a column begins, we assume that |cur_cmd| is either |omit| or else
//! the current token should be put back into the input until the \<u_j>
//! template has been scanned.  (Note that |cur_cmd| might be |tab_mark| or
//! |car_ret|.)  We also assume that |align_state| is approximately 1000000 at
//! this time.  We remain in the same mode, and start the template if it is
//! called for.
//
// @p procedure init_col;
pub(crate) fn init_col(globals: &mut TeXGlobals) {
    // begin extra_info(cur_align):=cur_cmd;
    extra_info!(globals, globals.cur_align) = globals.cur_cmd.into();
    // if cur_cmd=omit then align_state:=0
    if globals.cur_cmd == omit {
        globals.align_state = 0;
    }
    // else  begin back_input; begin_token_list(u_part(cur_align),u_template);
    else {
        back_input(globals);
        begin_token_list(globals, u_part!(globals, globals.cur_align) as _, u_template);
        // end; {now |align_state=1000000|}
        /// now `align_state=1000000`
        const _: () = ();
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::omit;
use crate::section_0307::u_template;
use crate::section_0323::begin_token_list;
use crate::section_0325::back_input;
