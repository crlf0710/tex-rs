//! ` `
// When |cur_p| is a discretionary break, the length of a line ``from |cur_p| to
// |cur_p|'' has to be defined properly so that the other calculations work out.
// Suppose that the pre-break text at |cur_p| has length $l_0$, the post-break
// text has length $l_1$, and the replacement text has length |l|. Suppose
// also that |q| is the node following the replacement text. Then length of a
// line from |cur_p| to |q| will be computed as $\gamma+\beta(q)-\alpha(|cur_p|)$,
// where $\beta(q)=\beta(|cur_p|)-l_0+l$. The actual length will be the background
// plus $l_1$, so the length from |cur_p| to |cur_p| should be $\gamma+l_0+l_1-l$.
// If the post-break text of the discretionary is empty, a break may also
// discard~|q|; in that unusual case we subtract the length of~|q| and any
// other nodes that will be discarded after the discretionary break.
//
// The value of $l_0$ need not be computed, since |line_break| will put
// it into the global variable |disc_width| before calling |try_break|.
//
// @<Glob...@>=
// @!disc_width:scaled; {the length of discretionary material preceding a break}
/// the length of discretionary material preceding a break
#[globals_struct_field(TeXGlobals)]
pub(crate) static disc_width: scaled = scaled::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::scaled;

use globals_struct::{globals_struct_field, globals_struct_use};
