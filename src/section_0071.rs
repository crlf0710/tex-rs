//! @ Here is a procedure that asks the user to type a line of input,
//! assuming that the |selector| setting is either |term_only| or |term_and_log|.
//! The input is placed into locations |first| through |last-1| of the
//! |buffer| array, and echoed on the transcript file if appropriate.
//!
//! This procedure is never called when |interaction<scroll_mode|.
//
// @d prompt_input(#)==begin wake_up_terminal; print(#); term_input;
//     end {prints a string and gets a line of input}
/// prints a string and gets a line of input
macro_rules! prompt_input {
    ($globals:expr, $val:expr) => {{
        trace_span!("prompt_input");
        wake_up_terminal($globals);
        print($globals, ($val).into());
        term_input($globals)?;

        use crate::section_0034::wake_up_terminal;
        use crate::section_0059::print;
        use crate::section_0071::term_input;
    }};
}

// @p procedure term_input; {gets a line from the terminal}
/// gets a line from the terminal
#[cfg_attr(
    feature = "trace",
    tracing::instrument(level = "trace", err),
    allow(unreachable_code)
)]
pub(crate) fn term_input(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var k:0..buf_size; {index into |buffer|}
    // begin update_terminal; {now the user sees the prompt for sure}
    /// now the user sees the prompt for sure
    update_terminal(globals);
    // if not input_ln(term_in,true) then fatal_error("End of file on the terminal!");
    if !input_ln(make_globals_io_view!(globals), &mut globals.term_in, true) {
        fatal_error(globals, strpool_str!("End of file on the terminal!"))?;
    }
    // @.End of file on the terminal@>
    // term_offset:=0; {the user's line ended with \<\rm return>}
    /// the user's line ended with \<\rm return>
    const _: () = ();
    globals.term_offset = 0.into();
    // decr(selector); {prepare to echo the input}
    /// prepare to echo the input
    const _: () = ();
    decr!(globals.selector);
    // if last<>first then for k:=first to last-1 do print(buffer[k]);
    if globals.last != globals.first {
        for k in globals.first.get()..=(globals.last.get() - 1) {
            print(globals, globals.buffer[k].numeric_value() as _);
        }
    }
    // print_ln; incr(selector); {restore previous status}
    /// restore previous status
    const _: () = ();
    print_ln(make_globals_io_string_log_view!(globals));
    incr!(globals.selector);
    // end;
    return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0031::input_ln;
use crate::section_0034::update_terminal;
use crate::section_0057::print_ln;
use crate::section_0059::print;
use crate::section_0081::TeXResult;
use crate::section_0093::fatal_error;
