//! @ Sometimes \TeX\ has read too far and wants to ``unscan'' what it has
//! seen. The |back_input| procedure takes care of this by putting the token
//! just scanned back into the input stream, ready to be read again. This
//! procedure can be used only if |cur_tok| represents the token to be
//! replaced. Some applications of \TeX\ use this procedure a lot,
//! so it has been slightly optimized for speed.
//! @^inner loop@>
//
// @p procedure back_input; {undoes one token of input}
/// undoes one token of input
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn back_input(globals: &mut TeXGlobals) {
    // var p:pointer; {a token list of length one}
    /// a token list of length one
    let p: pointer;
    // begin while (state=token_list)and(loc=null)and(token_type<>v_template) do
    //   end_token_list; {conserve stack space}
    // p:=get_avail; info(p):=cur_tok;
    p = get_avail(globals);
    info_tok_assign!(globals, p, globals.cur_tok);
    // if cur_tok<right_brace_limit then
    if globals.cur_tok < right_brace_limit {
        // if cur_tok<left_brace_limit then decr(align_state)
        if globals.cur_tok < left_brace_limit {
            decr!(globals.align_state);
        }
        // else incr(align_state);
        else {
            incr!(globals.align_state);
        }
    }
    // push_input; state:=token_list; start:=p; token_type:=backed_up;
    push_input!(globals);
    state!(globals) = token_list;
    start!(globals) = p;
    token_type!(globals) = backed_up;
    // loc:=p; {that was |back_list(p)|, without procedure overhead}
    loc!(globals) = p;
    /// that was `back_list(p)`, without procedure overhead
    const _ : () = ();
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0289::left_brace_limit;
use crate::section_0289::right_brace_limit;
use crate::section_0307::token_list;
use crate::section_0307::backed_up;
