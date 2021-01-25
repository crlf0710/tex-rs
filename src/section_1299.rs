//! @* \[50] Dumping and undumping the tables.
//! After \.{INITEX} has seen a collection of fonts and macros, it
//! can write all the necessary information on an auxiliary file so
//! that production versions of \TeX\ are able to initialize their
//! memory at high speed. The present section of the program takes
//! care of such output and input. We shall consider simultaneously
//! the processes of storing and restoring,
//! so that the inverse relation between them is clear.
//! @.INITEX@>
//!
//! The global variable |format_ident| is a string that is printed right
//! after the |banner| line when \TeX\ is ready to start. For \.{INITEX} this
//! string says simply `\.{(INITEX)}'; for other versions of \TeX\ it says,
//! for example, `\.{(preloaded format=plain 1982.11.19)}', showing the year,
//! month, and day that the format file was created. We have |format_ident=0|
//! before \TeX's tables are loaded.
//
// @<Glob...@>=
// @!format_ident:str_number;
#[globals_struct_field(TeXGlobals)]
pub(crate) static format_ident: str_number = str_number::zero();

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;

use globals_struct::{globals_struct_field, globals_struct_use};
