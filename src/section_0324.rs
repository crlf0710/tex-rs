//! @ When a token list has been fully scanned, the following computations
//! should be done as we leave that level of input. The |token_type| tends
//! to be equal to either |backed_up| or |inserted| about 2/3 of the time.
//! @^inner loop@>
//
// @p procedure end_token_list; {leave a token-list input level}
/// leave a token-list input level
#[allow(unused_variables)]
pub(crate) fn end_token_list(globals: &mut TeXGlobals) {
    // begin if token_type>=backed_up then {token list to be deleted}
    //   begin if token_type<=inserted then flush_list(start)
    //   else  begin delete_token_ref(start); {update reference count}
    //     if token_type=macro then {parameters must be flushed}
    //       while param_ptr>param_start do
    //         begin decr(param_ptr);
    //         flush_list(param_stack[param_ptr]);
    //         end;
    //     end;
    //   end
    // else if token_type=u_template then
    //   if align_state>500000 then align_state:=0
    //   else fatal_error("(interwoven alignment preambles are not allowed)");
    // @.interwoven alignment preambles...@>
    // pop_input;
    pop_input!(globals);
    // check_interrupt;
    // end;
}

use crate::section_0004::TeXGlobals;
