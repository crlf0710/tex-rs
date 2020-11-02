//! @ Some of the ASCII codes without visible characters have been given symbolic
//! names in this program because they are used with a special meaning.
//
// @d null_code=@'0 {ASCII code that might disappear}
/// ASCII code that might disappear
pub(crate) const null_code: u8 = 0o0;
// @d carriage_return=@'15 {ASCII code used at end of line}
/// ASCII code used at end of line
pub(crate) const carriage_return: u8 = 0o15;
// @d invalid_code=@'177 {ASCII code that many systems prohibit in text files}
/// ASCII code that many systems prohibit in text files
pub(crate) const invalid_code: u8 = 0o177;
