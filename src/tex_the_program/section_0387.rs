//! @ Now let's consider |macro_call| itself, which is invoked when \TeX\ is
//! scanning a control sequence whose |cur_cmd| is either |call|, |long_call|,
//! |outer_call|, or |long_outer_call|.  The control sequence definition
//! appears in the token list whose reference count is in location |cur_chr|
//! of |mem|.
//!
//! The global variable |long_state| will be set to |call| or to |long_call|,
//! depending on whether or not the control sequence disallows \.{\\par}
//! in its parameters. The |get_next| routine will set |long_state| to
//! |outer_call| and emit \.{\\par}, if a file ends or if an \.{\\outer}
//! control sequence occurs in the midst of an argument.

// @<Glob...@>=
// @!long_state:call..long_outer_call; {governs the acceptance of \.{\\par}}
/// governs the acceptance of `\par`
#[globals_struct_field(TeXGlobals)]
pub(crate) static long_state: u8_from_m_to_n<call_TYPENUM, long_outer_call_TYPENUM> = u8_from_m_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u8_from_m_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0210::call_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::section_0210::long_outer_call_TYPENUM;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};


