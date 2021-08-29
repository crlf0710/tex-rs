//! @ Here is the messy routine that was just mentioned. It sets |name_of_file|
//! from the first |n| characters of |TEX_format_default|, followed by
//! |buffer[a..b]|, followed by the last |format_ext_length| characters of
//! |TEX_format_default|.
//!
//! We dare not give error messages here, since \TeX\ calls this routine before
//! the |error| routine is ready to roll. Instead, we simply drop excess characters,
//! since the error will be detected in another way when a strange file name
//! isn't found.
//! @^system dependencies@>
//
// @p procedure pack_buffered_name(@!n:small_number;@!a,@!b:integer);
#[allow(unused_variables)]
pub(crate) fn pack_buffered_name(globals: &mut TeXGlobals, n: small_number, a: integer, mut b: integer) {
    // var k:integer; {number of positions filled in |name_of_file|}
    /// number of positions filled in `name_of_file`
    let mut k: integer;
    // @!c: ASCII_code; {character being packed}
    // @!j:integer; {index into |buffer| or |TEX_format_default|}
    // begin if n+b-a+1+format_ext_length>file_name_size then
    if n.get() as integer + b - a + 1 + format_ext_length as integer > file_name_size as integer {
        // b:=a+file_name_size-n-1-format_ext_length;
        b = a + file_name_size as integer - n.get() as integer - 1 - format_ext_length as integer;
    }
    // k:=0;
    k = 0;
    // for j:=1 to n do append_to_name(xord[TEX_format_default[j]]);
    for j in 1..=n.get() {
        append_to_name!(globals, xord(globals.TEX_format_default[j as u16]), k);
    }
    // for j:=a to b do append_to_name(buffer[j]);
    for j in a..= b {
        append_to_name!(globals, globals.buffer[j as u16], k);
    }
    // for j:=format_default_length-format_ext_length+1 to format_default_length do
    for j in format_default_length - format_ext_length + 1 ..= format_default_length {
        // append_to_name(xord[TEX_format_default[j]]);
        append_to_name!(globals, xord(globals.TEX_format_default[j as u16]), k);
    }
    // if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
    if k <= file_name_size as integer {
        globals.name_length = u16_from_0_to_n::new(k as _);
    } else {
        globals.name_length = u16_from_0_to_n::new(file_name_size);
    }
    // for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
    for k in globals.name_length.get()+1..=file_name_size as _ {
        globals.name_of_file[k] = xchr(ASCII_code_literal!(b' '));
    }
    // end;
}

use crate::pascal::integer;
use crate::pascal::u16_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::file_name_size;
use crate::section_0020::xchr;
use crate::section_0020::xord;
use crate::section_0101::small_number;
use crate::section_0520::format_ext_length;
use crate::section_0520::format_default_length;
