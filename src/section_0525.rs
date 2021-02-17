//! @ Operating systems often make it possible to determine the exact name (and
//! possible version number) of a file that has been opened. The following routine,
//! which simply makes a \TeX\ string from the value of |name_of_file|, should
//! ideally be changed to deduce the full name of file~|f|, which is the file
//! most recently opened, if it is possible to do this in a \PASCAL\ program.
//! @^system dependencies@>
//!
//! This routine might be called after string memory has overflowed, hence
//! we dare not use `|str_room|'.
//
// @p function make_name_string:str_number;
pub(crate) fn make_name_string(mut globals: TeXGlobalsIoStringView<'_>) -> str_number {
    // var k:1..file_name_size; {index into |name_of_file|}
    // begin if (pool_ptr+name_length>pool_size)or(str_ptr=max_strings)or
    //  (cur_length>0) then
    if globals.pool_ptr.get() + globals.name_length.get() as pool_pointer_repr
        > pool_size as pool_pointer_repr
        || globals.str_ptr.get() == max_strings as str_number_repr
        || cur_length!(globals) > 0
    {
        // make_name_string:="?"
        return strpool_str!("?");
    }
    // else  begin for k:=1 to name_length do append_char(xord[name_of_file[k]]);
    else {
        for k in 1..=globals.name_length.get() {
            append_char(
                make_globals_string_view!(globals),
                xord(globals.name_of_file[k]),
            );
        }
        // make_name_string:=make_string;
        return make_string(make_globals_string_view!(globals));
        // end;
    }
    // end;
}

// function a_make_name_string(var f:alpha_file):str_number;
pub(crate) fn a_make_name_string(
    globals: TeXGlobalsIoStringView<'_>,
    _: &mut alpha_file,
) -> str_number {
    // begin a_make_name_string:=make_name_string;
    return make_name_string(globals);
    // end;
}

// function b_make_name_string(var f:byte_file):str_number;
pub(crate) fn b_make_name_string(
    globals: TeXGlobalsIoStringView<'_>,
    _: &mut byte_file,
) -> str_number {
    // begin b_make_name_string:=make_name_string;
    return make_name_string(globals);
    // end;
}

// function w_make_name_string(var f:word_file):str_number;
pub(crate) fn w_make_name_string(
    globals: TeXGlobalsIoStringView<'_>,
    _: &mut word_file,
) -> str_number {
    // begin w_make_name_string:=make_name_string;
    return make_name_string(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0011::max_strings;
use crate::section_0011::pool_size;
use crate::section_0020::xord;
use crate::section_0025::alpha_file;
use crate::section_0025::byte_file;
use crate::section_0038::pool_pointer_repr;
use crate::section_0038::str_number;
use crate::section_0038::str_number_repr;
use crate::section_0042::append_char;
use crate::section_0043::make_string;
use crate::section_0113::word_file;