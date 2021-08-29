//! @ In order to study the memory requirements of particular applications, it
//! is possible to prepare a version of \TeX\ that keeps track of current and
//! maximum memory usage. When code between the delimiters |@!stat| $\ldots$
//! |tats| is not ``commented out,'' \TeX\ will run a bit slower but it will
//! report these statistics when |tracing_stats| is sufficiently large.
//
// @<Glob...@>=
// @!var_used, @!dyn_used : integer; {how much memory is in use}
/// how much memory is in use
#[globals_struct_field(TeXGlobals)]
pub(crate) static var_used: integer = 0;

#[globals_struct_field(TeXGlobals)]
pub(crate) static dyn_used: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
