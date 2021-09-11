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
/// smallest index in the `mem` array dumped by `INITEX`;
///   must not be less than |mem_min|
pub(crate) const mem_bot: pointer = 0;
// @d mem_top==30000 {largest index in the |mem| array dumped by \.{INITEX};
//   must be substantially larger than |mem_bot|
//   and not greater than |mem_max|}
/// largest index in the `mem` array dumped by `INITEX`;
/// must be substantially larger than `mem_bot` and not
/// greater than `mem_max`
pub(crate) const mem_top: pointer = 30000;
// @d font_base=0 {smallest internal font number; must not be less
//   than |min_quarterword|}
/// smallest internal font number; must not be less than `min_quarterword`
pub(crate) const font_base: quarterword = font_base_TYPENUM::U8;
pub(crate) type font_base_TYPENUM = U0;
// @d hash_size=2100 {maximum number of control sequences; it should be at most
//   about |(mem_max-mem_min)/10|}
/// maximum number of control sequences; it should be at most about `(mem_max-mem_min)/10`
pub(crate) const hash_size: halfword = hash_size_TYPENUM::U16;
pub(crate) type hash_size_TYPENUM = U2100;
// @d hash_prime=1777 {a prime number equal to about 85\pct! of |hash_size|}
/// a prime number equal to about 85% of `hash_size`
pub(crate) const hash_prime: integer = 1777;
// @d hyph_size=307 {another prime; the number of \.{\\hyphenation} exceptions}
/// another prime; the number of `\hyphenation` exceptions
pub(crate) const hyph_size: integer = hyph_size_TYPENUM::U16 as _;
pub(crate) type hyph_size_TYPENUM = U307;
// @^system dependencies@>
//

type U2100 = typenum::op!(U1000 + U1000 + U100);
use crate::pascal::integer;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use typenum::Unsigned;
use typenum::{U0, U100, U1000, U307};
