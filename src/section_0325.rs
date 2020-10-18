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
pub(crate) fn back_input(globals: &mut TeXGlobals) {
    // var p:pointer; {a token list of length one}
    // begin while (state=token_list)and(loc=null)and(token_type<>v_template) do
    //   end_token_list; {conserve stack space}
    // p:=get_avail; info(p):=cur_tok;
    // if cur_tok<right_brace_limit then
    //   if cur_tok<left_brace_limit then decr(align_state)
    //   else incr(align_state);
    // push_input; state:=token_list; start:=p; token_type:=backed_up;
    // loc:=p; {that was |back_list(p)|, without procedure overhead}
    // end;
}

use crate::section_0004::TeXGlobals;
