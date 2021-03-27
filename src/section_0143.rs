//! @ A |ligature_node|, which occurs only in horizontal lists, specifies
//! a character that was fabricated from the interaction of two or more
//! actual characters.  The second word of the node, which is called the
//! |lig_char| word, contains |font| and |character| fields just as in a
//! |char_node|. The characters that generated the ligature have not been
//! forgotten, since they are needed for diagnostic messages and for
//! hyphenation; the |lig_ptr| field points to a linked list of character
//! nodes for all original characters that have been deleted. (This list
//! might be empty if the characters that generated the ligature were
//! retained in other nodes.)
//!
//! The |subtype| field is 0, plus 2 and/or 1 if the original source of the
//! ligature included implicit left and/or right boundaries.
//
// @d ligature_node=6 {|type| of a ligature node}
/// `type` of a ligature node
pub(crate) const ligature_node: quarterword = 6;
// @d lig_char(#)==#+1 {the word where the ligature is to be found}
/// the word where the ligature is to be found
macro_rules! lig_char {
    ($p:expr) => {
        $p + 1
    };
}
// @d lig_ptr(#)==link(lig_char(#)) {the list of characters}
/// the list of characters
macro_rules! lig_ptr {
    ($globals:expr, $p:expr) => {
        link!($globals, lig_char!($p))
    };
}

use crate::section_0113::quarterword;
