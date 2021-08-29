//! @ The global variable |null_character| is set up to be a word of
//! |char_info| for a character that doesn't exist. Such a word provides a
//! convenient way to deal with erroneous situations.
//!
//! @<Glob...@>=
//! @!null_character:four_quarters; {nonexistent character information}
//!
//! @ @<Set init...@>=
//! null_character.b0:=min_quarterword; null_character.b1:=min_quarterword;
//! null_character.b2:=min_quarterword; null_character.b3:=min_quarterword;
//!
