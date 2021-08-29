//! @ The global variable |empty_field| is set up for initialization of empty
//! fields in new noads. Similarly, |null_delimiter| is for the initialization
//! of delimiter fields.
const empty: halfword = 0;
// @<Glob...@>=
// @!empty_field:two_halves;
pub(crate) const empty_field: two_halves = two_halves::new_with_halves(null, empty);
// @!null_delimiter:four_quarters;
pub(crate) const null_delimiter: four_quarters =
    four_quarters::new_with_quarters(0, min_quarterword, 0, min_quarterword);

use crate::section_0110::min_quarterword;
use crate::section_0113::four_quarters;
use crate::section_0113::two_halves;
use crate::section_0113::halfword;
use crate::section_0115::null;
