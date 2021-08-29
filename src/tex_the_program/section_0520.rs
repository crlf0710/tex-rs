//! @ A messier routine is also needed, since format file names must be scanned
//! before \TeX's string mechanism has been initialized. We shall use the
//! global variable |TEX_format_default| to supply the text for default system areas
//! and extensions related to format files.
//! @^system dependencies@>
//
// @d format_default_length=20 {length of the |TEX_format_default| string}
/// length of the `TEX_format_default` string
pub(crate) const format_default_length: usize = format_default_length_TYPENUM::USIZE;
pub(crate) type format_default_length_TYPENUM = U20;
// @d format_area_length=11 {length of its area part}
/// length of its area part
pub(crate) const format_area_length: usize = 11;
// @d format_ext_length=4 {length of its `\.{.fmt}' part}
/// length of its `.fmt` part
pub(crate) const format_ext_length: usize = 4;
// @d format_extension=".fmt" {the extension, as a \.{WEB} constant}
/// the extension, as a `WEB` constant
pub(crate) const format_extension: &'static str = ".fmt";

// @<Glob...@>=
// @!TEX_format_default:packed array[1..format_default_length] of char;
#[globals_struct_field(TeXGlobals)]
pub(crate) static TEX_format_default: format_default_array<crate::pascal::char> = format_default_array::default();

type format_default_length_LENGTH_TYPENUM = format_default_length_TYPENUM;

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) format_default_array[u16_from_m_to_n<U1, format_default_length_TYPENUM>] =>
    u16; U16; U1; format_default_length_LENGTH_TYPENUM
);

#[globals_struct_use(TeXGlobals)]
use crate::section_0520::format_default_array;

use crate::pascal::u16_from_m_to_n;
use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::Unsigned;
use typenum::{U1, U20};
