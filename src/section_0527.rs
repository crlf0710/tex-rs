//! @ The global variable |name_in_progress| is used to prevent recursive
//! use of |scan_file_name|, since the |begin_name| and other procedures
//! communicate via global variables. Recursion would arise only by
//! devious tricks like `\.{\\input\\input f}'; such attempts at sabotage
//! must be thwarted. Furthermore, |name_in_progress| prevents \.{\\input}
//! @^recursion@>
//! from being initiated when a font size specification is being scanned.
//!
//! Another global variable, |job_name|, contains the file name that was first
//! \.{\\input} by the user. This name is extended by `\.{.log}' and `\.{.dvi}'
//! and `\.{.fmt}' in the names of \TeX's output files.
//
// @<Glob...@>=
// @!name_in_progress:boolean; {is a file name being scanned?}
/// is a file name being scanned?
#[globals_struct_field(TeXGlobals)]
pub(crate) static name_in_progress: boolean = false;
// @!job_name:str_number; {principal file name}
/// principal file name
#[globals_struct_field(TeXGlobals)]
pub(crate) static job_name: str_number = str_number::zero();
// @!log_opened:boolean; {has the transcript file been opened?}
/// has the transcript file been opened?
#[globals_struct_field(TeXGlobals)]
pub(crate) static log_opened: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;


use globals_struct::{globals_struct_field, globals_struct_use};
