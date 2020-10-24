//! @ If some trouble arises when \TeX\ tries to open a file, the following
//! routine calls upon the user to supply another file name. Parameter~|s|
//! is used in the error message to identify the type of file; parameter~|e|
//! is the default extension if none is given. Upon exit from the routine,
//! variables |cur_name|, |cur_area|, |cur_ext|, and |name_of_file| are
//! ready for another attempt at file opening.
//
// @p procedure prompt_file_name(@!s,@!e:str_number);
#[allow(unused_variables)]
pub(crate) fn prompt_file_name(globals: &mut TeXGlobals, s: str_number, e: str_number) {
    // label done;
    // var k:0..buf_size; {index into |buffer|}
    // begin if interaction=scroll_mode then wake_up_terminal;
    // if s="input file name" then print_err("I can't find file `")
    // @.I can't find file x@>
    // else print_err("I can't write on file `");
    // @.I can't write on file x@>
    // print_file_name(cur_name,cur_area,cur_ext); print("'.");
    // if e=".tex" then show_context;
    // print_nl("Please type another "); print(s);
    print_nl(globals, strpool_str!("Please type another "));
    print(globals, s.into());
    // @.Please type...@>
    // if interaction<scroll_mode then
    //   fatal_error("*** (job aborted, file error in nonstop mode)");
    // @.job aborted, file error...@>
    // clear_terminal; prompt_input(": "); @<Scan file name in the buffer@>;
    // if cur_ext="" then cur_ext:=e;
    // pack_cur_name;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0059::print;
use crate::section_0062::print_nl;
