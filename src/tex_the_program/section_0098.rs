//! @ When an interrupt has been detected, the program goes into its
//! highest interaction level and lets the user have nearly the full flexibility of
//! the |error| routine.  \TeX\ checks for interrupts only at times when it is
//! safe to do this.
//
// @p procedure pause_for_instructions;
#[allow(unused_variables)]
pub(crate) fn pause_for_instructions(globals: &mut TeXGlobals) {
    // begin if OK_to_interrupt then
    //   begin interaction:=error_stop_mode;
    //   if (selector=log_only)or(selector=no_print) then
    //     incr(selector);
    //   print_err("Interruption");
    // @.Interruption@>
    //   help3("You rang?")@/
    //   ("Try to insert an instruction for me (e.g., `I\showlists'),")@/
    //   ("unless you just want to quit by typing `X'.");
    //   deletions_allowed:=false; error; deletions_allowed:=true;
    //   interrupt:=0;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
