//! @ Here is a routine that manufactures the output file names, assuming that
//! |job_name<>0|. It ignores and changes the current settings of |cur_area|
//! and |cur_ext|.
//
// @d pack_cur_name==pack_file_name(cur_name,cur_area,cur_ext)
pub(crate) fn pack_cur_name(globals: &mut TeXGlobals) {
    pack_file_name(globals, globals.cur_name, globals.cur_area, globals.cur_ext);
}

// @p procedure pack_job_name(@!s:str_number); {|s = ".log"|, |".dvi"|, or
//   |format_extension|}
/// `s = ".log"`, `".dvi"`, or `format_extension`
pub(crate) fn pack_job_name(globals: &mut TeXGlobals, s: str_number) {
    // begin cur_area:=""; cur_ext:=s;
    globals.cur_area = strpool_str!("");
    globals.cur_ext = s;
    // cur_name:=job_name; pack_cur_name;
    globals.cur_name = globals.job_name;
    pack_cur_name(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0519::pack_file_name;
