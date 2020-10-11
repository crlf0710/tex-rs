//! @ Like the preceding parameters, the following quantities can be changed
//! at compile time to extend or reduce \TeX's capacity. But if they are changed,
//! it is necessary to rerun the initialization program \.{INITEX}
//! @.INITEX@>
//! to generate new tables for the production \TeX\ program.
//! One can't simply make helter-skelter changes to the following constants,
//! since certain rather complex initialization
//! numbers are computed from them. They are defined here using
//! \.{WEB} macros, instead of being put into \PASCAL's |const| list, in order to
//! emphasize this distinction.

// @d mem_bot=0 {smallest index in the |mem| array dumped by \.{INITEX};
//   must not be less than |mem_min|}
// @d mem_top==30000 {largest index in the |mem| array dumped by \.{INITEX};
//   must be substantially larger than |mem_bot|
//   and not greater than |mem_max|}
// @d font_base=0 {smallest internal font number; must not be less
//   than |min_quarterword|}
// @d hash_size=2100 {maximum number of control sequences; it should be at most
//   about |(mem_max-mem_min)/10|}
/// maximum number of control sequences; it should be at most about `(mem_max-mem_min)/10`
pub(crate) const hash_size: halfword = 2100;
// @d hash_prime=1777 {a prime number equal to about 85\pct! of |hash_size|}
// @d hyph_size=307 {another prime; the number of \.{\\hyphenation} exceptions}
// @^system dependencies@>
//

use crate::section_0113::halfword;
