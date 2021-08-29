//! @ When the \.{WEB} system program called \.{TANGLE} processes the \.{TEX.WEB}
//! description that you are now reading, it outputs the \PASCAL\ program
//! \.{TEX.PAS} and also a string pool file called \.{TEX.POOL}. The \.{INITEX}
//! @.WEB@>@.INITEX@>
//! program reads the latter file, where each string appears as a two-digit decimal
//! length followed by the string itself, and the information is recorded in
//! \TeX's string memory.
//
// @<Glob...@>=
// @!init @!pool_file:alpha_file; {the string-pool file output by \.{TANGLE}}

#[cfg(feature = "initex")]
#[globals_struct_field(TeXGlobals)]
pub(crate) static pool_file: alpha_file = alpha_file::default();
// tini

use crate::section_0004::TeXGlobals;
use globals_struct::globals_struct_field;
