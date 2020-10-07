//! @ The user's terminal acts essentially like other files of text, except
//! that it is used both for input and for output. When the terminal is
//! considered an input file, the file variable is called |term_in|, and when it
//! is considered an output file the file variable is |term_out|.
//! @^system dependencies@>

// @<Glob...@>=
// @!term_in:alpha_file; {the terminal as an input file}

#[globals_struct_field(TeXGlobals)]
/// the terminal as an input file
pub(crate) static term_in: alpha_file = alpha_file::default();

// @!term_out:alpha_file; {the terminal as an output file}

#[globals_struct_field(TeXGlobals)]
/// the terminal as an output file
pub(crate) static term_out: alpha_file = alpha_file::default();

use globals_struct::globals_struct_field;
