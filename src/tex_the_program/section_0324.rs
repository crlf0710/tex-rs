//! @ When a token list has been fully scanned, the following computations
//! should be done as we leave that level of input. The |token_type| tends
//! to be equal to either |backed_up| or |inserted| about 2/3 of the time.
//! @^inner loop@>
//
// @p procedure end_token_list; {leave a token-list input level}
/// leave a token-list input level
#[allow(unused_variables)]
#[cfg_attr(feature = "trace_verbose", tracing::instrument(level = "trace"))]
pub(crate) fn end_token_list(globals: &mut TeXGlobals) {
    // begin if token_type>=backed_up then {token list to be deleted}
    if token_type!(globals) >= backed_up {
        /// token list to be deleted
        const _: () = ();
        // begin if token_type<=inserted then flush_list(start)
        if token_type!(globals) <= inserted {
            flush_list(globals, start!(globals));
        }
        // else  begin delete_token_ref(start); {update reference count}
        else {
            /// update reference count
            delete_token_ref(globals, start!(globals));
            // if token_type=macro then {parameters must be flushed}
            if token_type!(globals) == r#macro {
                /// parameters must be flushed
                const _: () = ();
                // while param_ptr>param_start do
                while globals.param_ptr.get() as halfword > param_start!(globals) {
                    // begin decr(param_ptr);
                    decr!(globals.param_ptr);
                    // flush_list(param_stack[param_ptr]);
                    flush_list(globals, globals.param_stack[globals.param_ptr]);
                    // end;
                }
                // end;
            }
            // end
        }
    }
    // else if token_type=u_template then
    else if token_type!(globals) == u_template {
        // if align_state>500000 then align_state:=0
        if globals.align_state > 500000 {
            globals.align_state = 0;
        }
        // else fatal_error("(interwoven alignment preambles are not allowed)");
        else {
            todo!();
        }
    }
    // @.interwoven alignment preambles...@>
    // pop_input;
    pop_input!(globals);
    // check_interrupt;
    check_interrupt!(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0096::check_interrupt;
use crate::section_0113::halfword;
use crate::section_0123::flush_list;
use crate::section_0200::delete_token_ref;
use crate::section_0302::start;
use crate::section_0307::backed_up;
use crate::section_0307::inserted;
use crate::section_0307::param_start;
use crate::section_0307::r#macro;
use crate::section_0307::token_type;
use crate::section_0307::u_template;
use crate::section_0322::pop_input;
