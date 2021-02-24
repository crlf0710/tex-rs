//! @ A messier routine is also needed, since format file names must be scanned
//! before \TeX's string mechanism has been initialized. We shall use the
//! global variable |TEX_format_default| to supply the text for default system areas
//! and extensions related to format files.
//! @^system dependencies@>
//
// @d format_default_length=20 {length of the |TEX_format_default| string}
// @d format_area_length=11 {length of its area part}
// @d format_ext_length=4 {length of its `\.{.fmt}' part}
// @d format_extension=".fmt" {the extension, as a \.{WEB} constant}
/// the extension, as a `WEB` constant
pub(crate) const format_extension: &'static str = ".fmt";

// @<Glob...@>=
// @!TEX_format_default:packed array[1..format_default_length] of char;
//
