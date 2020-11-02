//! @ The input routines must also interact with the processing of
//! \.{\\halign} and \.{\\valign}, since the appearance of tab marks and
//! \.{\\cr} in certain places is supposed to trigger the beginning of special
//! \<v_j> template text in the scanner. This magic is accomplished by an
//! |align_state| variable that is increased by~1 when a `\.{\char'173}' is
//! scanned and decreased by~1 when a `\.{\char'175}' is scanned. The |align_state|
//! is nonzero during the \<u_j> template, after which it is set to zero; the
//! \<v_j> template begins when a tab mark or \.{\\cr} occurs at a time that
//! |align_state=0|.
//
// @<Glob...@>=
// @!align_state:integer; {group level with respect to current alignment}
/// group level with respect to current alignment
#[globals_struct_field(TeXGlobals)]
pub(crate) static align_state:integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use globals_struct::{globals_struct_field, globals_struct_use};
