//! @ As we consider various ways to end a line at |cur_p|, in a given line number
//! class, we keep track of the best total demerits known, in an array with
//! one entry for each of the fitness classifications. For example,
//! |minimal_demerits[tight_fit]| contains the fewest total demerits of feasible
//! line breaks ending at |cur_p| with a |tight_fit| line; |best_place[tight_fit]|
//! points to the passive node for the break before~|cur_p| that achieves such
//! an optimum; and |best_pl_line[tight_fit]| is the |line_number| field in the
//! active node corresponding to |best_place[tight_fit]|. When no feasible break
//! sequence is known, the |minimal_demerits| entries will be equal to
//! |awful_bad|, which is $2^{30}-1$. Another variable, |minimum_demerits|,
//! keeps track of the smallest value in the |minimal_demerits| array.
//
// @d awful_bad==@'7777777777 {more than a billion demerits}
/// more than a billion demerits
pub(crate) const awful_bad: integer = 0o7777777777;

// @<Global...@>=
// @!minimal_demerits:array[very_loose_fit..tight_fit] of integer; {best total
//   demerits known for current line class and position, given the fitness}
// @!minimum_demerits:integer; {best total demerits known for current line class
//   and position}
/// best total demerits known for current line class and position
#[globals_struct_field(TeXGlobals)]
pub(crate) static minimum_demerits: integer = 0;
// @!best_place:array[very_loose_fit..tight_fit] of pointer; {how to achieve
//   |minimal_demerits|}
// @!best_pl_line:array[very_loose_fit..tight_fit] of halfword; {corresponding
//   line number}
//

use crate::pascal::integer;
use globals_struct::{globals_struct_field, globals_struct_use};
