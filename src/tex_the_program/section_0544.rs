//! @ The |tag| field in a |char_info_word| has four values that explain how to
//! interpret the |remainder| field.
//!
//! \yskip\hangg|tag=0| (|no_tag|) means that |remainder| is unused.\par
//! \hangg|tag=1| (|lig_tag|) means that this character has a ligature/kerning
//! program starting at position |remainder| in the |lig_kern| array.\par
//! \hangg|tag=2| (|list_tag|) means that this character is part of a chain of
//! characters of ascending sizes, and not the largest in the chain.  The
//! |remainder| field gives the character code of the next larger character.\par
//! \hangg|tag=3| (|ext_tag|) means that this character code represents an
//! extensible character, i.e., a character that is built up of smaller pieces
//! so that it can be made arbitrarily large. The pieces are specified in
//! |@!exten[remainder]|.\par
//! \yskip\noindent
//! Characters with |tag=2| and |tag=3| are treated as characters with |tag=0|
//! unless they are used in special circumstances in math formulas. For example,
//! the \.{\\sum} operation looks for a |list_tag|, and the \.{\\left}
//! operation looks for both |list_tag| and |ext_tag|.
//
// @d no_tag=0 {vanilla character}
// @d lig_tag=1 {character has a ligature/kerning program}
// @d list_tag=2 {character has a successor in a charlist}
// @d ext_tag=3 {character is extensible}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum char_tag {
    /// vanilla character
    no_tag = 0,
    /// character has a ligature/kerning program
    lig_tag = 1,
    /// character has a successor in a charlist
    list_tag = 2,
    /// character is extensible
    ext_tag = 3,
}

impl From<u8> for char_tag {
    fn from(v: u8) -> Self {
        match v {
            0 => char_tag::no_tag,
            1 => char_tag::lig_tag,
            2 => char_tag::list_tag,
            3 => char_tag::ext_tag,
            _ => unreachable!(),
        }
    }
}
