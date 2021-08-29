//! @ Most of the parameters kept in |eqtb| can be changed freely, but there's
//! an exception:  The magnification should not be used with two different
//! values during any \TeX\ job, since a single magnification is applied to an
//! entire run. The global variable |mag_set| is set to the current magnification
//! whenever it becomes necessary to ``freeze'' it at a particular value.
//
// @<Glob...@>=
// @!mag_set:integer; {if nonzero, this magnification should be used henceforth}
/// if nonzero, this magnification should be used henceforth
#[globals_struct_field(TeXGlobals)]
pub(crate) static mag_set: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
