//! ` `

// @<Set init...@>=
// null_character.b0:=min_quarterword; null_character.b1:=min_quarterword;
// null_character.b2:=min_quarterword; null_character.b3:=min_quarterword;
/// nonexistent character information
pub(crate) const NULL_CHARACTER: char_info =
    char_info::from_four_quarters(four_quarters::new_with_quarters(
        min_quarterword,
        min_quarterword,
        min_quarterword,
        min_quarterword,
    ));

use crate::section_0110::min_quarterword;
use crate::section_0113::four_quarters;
use crate::section_0554::char_info;
