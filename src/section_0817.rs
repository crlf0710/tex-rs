//! @ When looking for optimal line breaks, \TeX\ creates a ``break node'' for
//! each break that is {\sl feasible}, in the sense that there is a way to end
//! a line at the given place without requiring any line to stretch more than
//! a given tolerance. A break node is characterized by three things: the position
//! of the break (which is a pointer to a |glue_node|, |math_node|, |penalty_node|,
//! or |disc_node|); the ordinal number of the line that will follow this
//! breakpoint; and the fitness classification of the line that has just
//! ended, i.e., |tight_fit|, |decent_fit|, |loose_fit|, or |very_loose_fit|.
//
// @d tight_fit=3 {fitness classification for lines shrinking 0.5 to 1.0 of their
//   shrinkability}
// @d loose_fit=1 {fitness classification for lines stretching 0.5 to 1.0 of their
//   stretchability}
// @d very_loose_fit=0 {fitness classification for lines stretching more than
//   their stretchability}
// @d decent_fit=2 {fitness classification for all other lines}
/// fitness classification for all other lines
pub(crate) const decent_fit: quarterword = 2;

use crate::section_0113::quarterword;
