//! @* \[10] Data structures for boxes and their friends.
//! From the computer's standpoint, \TeX's chief mission is to create
//! horizontal and vertical lists. We shall now investigate how the elements
//! of these lists are represented internally as nodes in the dynamic memory.
//!
//! A horizontal or vertical list is linked together by |link| fields in
//! the first word of each node. Individual nodes represent boxes, glue,
//! penalties, or special things like discretionary hyphens; because of this
//! variety, some nodes are longer than others, and we must distinguish different
//! kinds of nodes. We do this by putting a `|type|' field in the first word,
//! together with the link and an optional `|subtype|'.
//
// @d type(#) == mem[#].hh.b0 {identifies what kind of node this is}
/// identifies what kind of node this is
macro_rules! r#type {
    ($globals:expr, $ptr:expr) => {
        $globals.mem[$ptr][crate::section_0113::MEMORY_WORD_HH_B0]
    }
}
// @d subtype(#) == mem[#].hh.b1 {secondary identification in some cases}
/// secondary identification in some cases
macro_rules! r#subtype {
    ($globals:expr, $ptr:expr) => {
        $globals.mem[$ptr][crate::section_0113::MEMORY_WORD_HH_B1]
    }
}