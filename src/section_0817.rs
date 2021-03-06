//! @ When looking for optimal line breaks, \TeX\ creates a ``break node'' for
//! each break that is {\sl feasible}, in the sense that there is a way to end
//! a line at the given place without requiring any line to stretch more than
//! a given tolerance. A break node is characterized by three things: the position
//! of the break (which is a pointer to a |glue_node|, |math_node|, |penalty_node|,
//! or |disc_node|); the ordinal number of the line that will follow this
//! breakpoint; and the fitness classification of the line that has just
//! ended, i.e., |tight_fit|, |decent_fit|, |loose_fit|, or |very_loose_fit|.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub(crate) enum fit_class_kind {
    // @d tight_fit=3 {fitness classification for lines shrinking 0.5 to 1.0 of their
    //   shrinkability}
    /// fitness classification for lines shrinking 0.5 to 1.0 of their shrinkability
    tight_fit = 3,
    // @d loose_fit=1 {fitness classification for lines stretching 0.5 to 1.0 of their
    //   stretchability}
    /// fitness classification for lines stretching 0.5 to 1.0 of their stretchability
    loose_fit = 1,
    // @d very_loose_fit=0 {fitness classification for lines stretching more than
    //   their stretchability}
    /// fitness classification for lines stretching more than their stretchability
    very_loose_fit = 0,
    // @d decent_fit=2 {fitness classification for all other lines}
    /// fitness classification for all other lines
    decent_fit = 2,
}

impl fit_class_kind {
    pub(crate) fn get(self) -> u8 {
        self as u8
    }
}

impl From<u8> for fit_class_kind {
    fn from(val: u8) -> Self {
        if val == fit_class_kind::tight_fit as u8 {
            return fit_class_kind::tight_fit;
        } else if val == fit_class_kind::loose_fit as u8 {
            return fit_class_kind::loose_fit;
        } else if val == fit_class_kind::very_loose_fit as u8 {
            return fit_class_kind::very_loose_fit;
        } else if val == fit_class_kind::decent_fit as u8 {
            return fit_class_kind::decent_fit;
        } else {
            unreachable!();
        }
    }
}

pub(crate) type very_loose_fit_TYPENUM = U0;
pub(crate) type tight_fit_TYPENUM = U3;

use crate::section_0113::quarterword;
use typenum::{U0, U3};
