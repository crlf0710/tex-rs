//! @ The forced line break at the paragraph's end will reduce the list of
//! breakpoints so that all active nodes represent breaks at |cur_p=null|.
//! On the first pass, we insist on finding an active node that has the
//! correct ``looseness.'' On the final pass, there will be at least one active
//! node, and we will match the desired looseness as well as we can.
//!
//! The global variable |best_bet| will be set to the active node for the best
//! way to break the paragraph, and a few other variables are used to
//! help determine what is best.
//
// @<Glob...@>=
// @!best_bet:pointer; {use this passive node and its predecessors}
// @!fewest_demerits:integer; {the demerits associated with |best_bet|}
// @!best_line:halfword; {line number following the last line of the new paragraph}
// @!actual_looseness:integer; {the difference between |line_number(best_bet)|
//   and the optimum |best_line|}
/// the difference between `line_number(best_bet)` and the optimum `best_line`
#[globals_struct_field(TeXGlobals)]
pub(crate) static actual_looseness: integer = 0;
// @!line_diff:integer; {the difference between the current line number and
//   the optimum |best_line|}

use globals_struct::{globals_struct_field, globals_struct_use};
