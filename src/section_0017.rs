//! \[2] The character set.
//!
//! In order to make \TeX\ readily portable to a wide variety of
//! computers, all of its input text is converted to an internal eight-bit
//! code that includes standard ASCII, the ``American Standard Code for
//! Information Interchange.''  This conversion is done immediately when each
//! character is read in. Conversely, characters are converted from ASCII to
//! the user's external representation just before they are output to a
//! text file.
//!
//! Such an internal code is relevant to users of \TeX\ primarily because it
//! governs the positions of characters in the fonts. For example, the
//! character `\.A' has ASCII code $65=@'101$, and when \TeX\ typesets
//! this letter it specifies character number 65 in the current font.
//! If that font actually has `\.A' in a different position, \TeX\ doesn't
//! know what the real position is; the program that does the actual printing from
//! \TeX's device-independent files is responsible for converting from ASCII to
//! a particular font encoding.
//! @^ASCII code@>
//!
//! \TeX's internal code also defines the value of constants
//! that begin with a reverse apostrophe; and it provides an index to the
//! \.{\\catcode}, \.{\\mathcode}, \.{\\uccode}, \.{\\lccode}, and \.{\\delcode}
//! tables.
//!

migration_complete!();
