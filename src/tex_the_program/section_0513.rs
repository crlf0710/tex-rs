//! @ The file names we shall deal with for illustrative purposes have the
//! following structure:  If the name contains `\.>' or `\.:', the file area
//! consists of all characters up to and including the final such character;
//! otherwise the file area is null.  If the remaining file name contains
//! `\..', the file extension consists of all such characters from the first
//! remaining `\..' to the end, otherwise the file extension is null.
//! @^system dependencies@>
//!
//! We can scan such file names easily by using two global variables that keep track
//! of the occurrences of area and extension delimiters:
//
// @<Glob...@>=
// @!area_delimiter:pool_pointer; {the most recent `\.>' or `\.:', if any}
/// the most recent `>' or `:', if any
#[globals_struct_field(TeXGlobals)]
pub(crate) static area_delimiter: pool_pointer = pool_pointer::default();
// @!ext_delimiter:pool_pointer; {the relevant `\..', if any}
/// the relevant `.', if any
#[globals_struct_field(TeXGlobals)]
pub(crate) static ext_delimiter: pool_pointer = pool_pointer::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::pool_pointer;

use globals_struct::{globals_struct_field, globals_struct_use};
use crate::section_0004::TeXGlobals;
