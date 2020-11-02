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
#[allow(unused_variables)]
pub(crate) fn make_name_string(globals: TeXGlobalsIoFilenameView<'_>) -> str_number {
    // var k:1..file_name_size; {index into |name_of_file|}
    // begin if (pool_ptr+name_length>pool_size)or(str_ptr=max_strings)or
    //  (cur_length>0) then
    //   make_name_string:="?"
    // else  begin for k:=1 to name_length do append_char(xord[name_of_file[k]]);
    //   make_name_string:=make_string;
    //   end;
    // end;
    todo!();
}

// function a_make_name_string(var f:alpha_file):str_number;
pub(crate) fn a_make_name_string(globals: TeXGlobalsIoFilenameView<'_>, _: &mut alpha_file) -> str_number {
    // begin a_make_name_string:=make_name_string;
    return make_name_string(globals);
    // end;
}

// function b_make_name_string(var f:byte_file):str_number;
// begin b_make_name_string:=make_name_string;
// end;
// function w_make_name_string(var f:word_file):str_number;
// begin w_make_name_string:=make_name_string;
// end;
//

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoFilenameView;
use crate::section_0004::alpha_file;
use crate::section_0038::str_number;

