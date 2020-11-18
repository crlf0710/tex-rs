//! @ A |penalty_node| specifies the penalty associated with line or page
//! breaking, in its |penalty| field. This field is a fullword integer, but
//! the full range of integer values is not used: Any penalty |>=10000| is
//! treated as infinity, and no break will be allowed for such high values.
//! Similarly, any penalty |<=-10000| is treated as negative infinity, and a
//! break will be forced.
//
// @d penalty_node=12 {|type| of a penalty node}
/// `type` of a penalty node
pub(crate) const penalty_node: quarterword = 12;
// @d inf_penalty=inf_bad {``infinite'' penalty value}
// @d eject_penalty=-inf_penalty {``negatively infinite'' penalty value}
// @d penalty(#) == mem[#+1].int {the added cost of breaking a list here}
/// the added cost of breaking a list here
macro_rules! penalty {
    ($globals:expr, $ptr:expr) => {
        $globals.mem[$ptr + 1][crate::section_0113::MEMORY_WORD_INT]
    }
}

use crate::section_0113::quarterword;