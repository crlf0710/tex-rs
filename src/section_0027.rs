//! @ The \ph\ compiler with which the present version of \TeX\ was prepared has
//! extended the rules of \PASCAL\ in a very convenient way. To open file~|f|,
//! we can write
//! $$\vbox{\halign{#\hfil\qquad&#\hfil\cr
//! |reset(f,@t\\{name}@>,'/O')|&for input;\cr
//! |rewrite(f,@t\\{name}@>,'/O')|&for output.\cr}}$$
//! The `\\{name}' parameter, which is of type `{\bf packed array
//! $[\langle\\{any}\rangle]$ of \\{char}}', stands for the name of
//! the external file that is being opened for input or output.
//! Blank spaces that might appear in \\{name} are ignored.
//!
//! The `\.{/O}' parameter tells the operating system not to issue its own
//! error messages if something goes wrong. If a file of the specified name
//! cannot be found, or if such a file cannot be opened for some other reason
//! (e.g., someone may already be trying to write the same file), we will have
//! |@!erstat(f)<>0| after an unsuccessful |reset| or |rewrite|.  This allows
//! \TeX\ to undertake appropriate corrective action.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//!
//! \TeX's file-opening procedures return |false| if no file identified by
//! |name_of_file| could be opened.
//
// @d reset_OK(#)==erstat(#)=0
#[allow(unused_macros)]
macro_rules! reset_OK {
    ($f:expr) => {
        crate::pascal::erstat($f) == 0
    };
}
// @d rewrite_OK(#)==erstat(#)=0
#[allow(unused_macros)]
macro_rules! rewrite_OK {
    ($f:expr) => {
        crate::pascal::erstat($f) == 0
    };
}
//
// @p function a_open_in(var f:alpha_file):boolean;
//   {open a text file for input}
/// open a text file for input
pub(crate) fn a_open_in(globals: TeXGlobalsFilenameView<'_>, f: &mut alpha_file) -> boolean {
    // begin reset(f,name_of_file,'/O'); a_open_in:=reset_OK(f);
    reset(f, &*globals.name_of_file, "/O");
    return reset_OK!(f);
    // end;
}
// @#
// function a_open_out(var f:alpha_file):boolean;
//   {open a text file for output}
/// open a text file for output
pub(crate) fn a_open_out(globals: TeXGlobalsFilenameView<'_>, f: &mut alpha_file) -> boolean {
    // begin rewrite(f,name_of_file,'/O'); a_open_out:=rewrite_OK(f);
    rewrite(f, &*globals.name_of_file, "/O");
    return rewrite_OK!(f);
    // end;
}
// @#
// function b_open_in(var f:byte_file):boolean;
//   {open a binary file for input}
/// open a binary file for input
pub(crate) fn b_open_in(globals: TeXGlobalsFilenameView<'_>, f: &mut byte_file) -> boolean {
    // begin reset(f,name_of_file,'/O'); b_open_in:=reset_OK(f);
    reset(f, &*globals.name_of_file, "/O");
    return reset_OK!(f);
    // end;
}
// @#
// function b_open_out(var f:byte_file):boolean;
//   {open a binary file for output}
/// open a binary file for output
pub(crate) fn b_open_out(globals: TeXGlobalsFilenameView<'_>, f: &mut byte_file) -> boolean {
    // begin rewrite(f,name_of_file,'/O'); b_open_out:=rewrite_OK(f);
    rewrite(f, &*globals.name_of_file, "/O");
    return rewrite_OK!(f);
    // end;
}
// @#
// function w_open_in(var f:word_file):boolean;
//   {open a word file for input}
// begin reset(f,name_of_file,'/O'); w_open_in:=reset_OK(f);
// end;
// @#
// function w_open_out(var f:word_file):boolean;
//   {open a word file for output}
/// open a word file for output
pub(crate) fn w_open_out(globals: TeXGlobalsFilenameView<'_>, f: &mut word_file) -> boolean {
    // begin rewrite(f,name_of_file,'/O'); w_open_out:=rewrite_OK(f);
    rewrite(f, &*globals.name_of_file, "/O");
    return rewrite_OK!(f);
    // end;
}

use crate::pascal::boolean;
use crate::pascal::reset;
use crate::pascal::rewrite;
use crate::section_0004::TeXGlobalsFilenameView;
use crate::section_0025::alpha_file;
use crate::section_0025::byte_file;
use crate::section_0113::word_file;
