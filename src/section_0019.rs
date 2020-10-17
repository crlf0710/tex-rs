//! @ The original \PASCAL\ compiler was designed in the late 60s, when six-bit
//! character sets were common, so it did not make provision for lowercase
//! letters. Nowadays, of course, we need to deal with both capital and small
//! letters in a convenient way, especially in a program for typesetting;
//! so the present specification of \TeX\ has been written under the assumption
//! that the \PASCAL\ compiler and run-time system permit the use of text files
//! with more than 64 distinguishable characters. More precisely, we assume that
//! the character set contains at least the letters and symbols associated
//! with ASCII codes @'40 through @'176; all of these characters are now
//! available on most computer terminals.
//!
//! Since we are dealing with more characters than were present in the first
//! \PASCAL\ compilers, we have to decide what to call the associated data
//! type. Some \PASCAL s use the original name |char| for the
//! characters in text files, even though there now are more than 64 such
//! characters, while other \PASCAL s consider |char| to be a 64-element
//! subrange of a larger data type that has some other name.
//!
//! In order to accommodate this difference, we shall use the name |text_char|
//! to stand for the data type of the characters that are converted to and
//! from |ASCII_code| when they are input and output. We shall also assume
//! that |text_char| consists of the elements |chr(first_text_char)| through
//! |chr(last_text_char)|, inclusive. The following definitions should be
//! adjusted if necessary.
//! @^system dependencies@>
//
// @d text_char == char {the data type of characters in text files}

/// the data type of characters in text files
pub(crate) type text_char = crate::pascal::char;

// @d first_text_char=0 {ordinal number of the smallest element of |text_char|}

/// ordinal number of the smallest element of [text_char]
const first_text_char: text_char = text_char::new(0);

// @d last_text_char=255 {ordinal number of the largest element of |text_char|}

/// ordinal number of the largest element of [text_char]
#[cfg(not(feature = "unicode_support"))]
const last_text_char: text_char = text_char::new(255);

/// ordinal number of the largest element of [text_char]
#[cfg(feature = "unicode_support")]
const last_text_char: text_char = text_char::new(u32::MAX);

//
// @<Local variables for init...@>=
// @!i:integer;
//
