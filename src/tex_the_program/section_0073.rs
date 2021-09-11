//! @ The global variable |interaction| has four settings, representing increasing
//! amounts of user interaction:
//
// @d batch_mode=0 {omits all stops and omits terminal output}
/// omits all stops and omits terminal output
pub(crate) const batch_mode: u8 = batch_mode_TYPENUM::U8;
pub(crate) type batch_mode_TYPENUM = U0;
// @d nonstop_mode=1 {omits all stops}
/// omits all stops
pub(crate) const nonstop_mode: u8 = nonstop_mode_TYPENUM::U8;
pub(crate) type nonstop_mode_TYPENUM = U1;
// @d scroll_mode=2 {omits error stops}
/// omits error stops
pub(crate) const scroll_mode: u8 = scroll_mode_TYPENUM::U8;
pub(crate) type scroll_mode_TYPENUM = U2;
// @d error_stop_mode=3 {stops at every opportunity to interact}
/// stops at every opportunity to interact
pub(crate) const error_stop_mode: u8 = error_stop_mode_TYPENUM::U8;
pub(crate) type error_stop_mode_TYPENUM = U3;
// @d print_err(#)==begin if interaction=error_stop_mode then wake_up_terminal;
pub(crate) macro print_err($globals:expr, $val:expr) {{
    if $globals.interaction == error_stop_mode {
        wake_up_terminal($globals);
    }
    // print_nl("! "); print(#);
    print_nl($globals, crate::strpool_str!("! "));
    print($globals, $val.into());
    // end
    use crate::section_0034::wake_up_terminal;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0073::error_stop_mode;
}}

// @<Glob...@>=
// @!interaction:batch_mode..error_stop_mode; {current level of interaction}
/// current level of interaction
#[globals_struct_field(TeXGlobals)]
pub(crate) static interaction: u8_from_m_to_n<batch_mode_TYPENUM, error_stop_mode_TYPENUM> =
    u8_from_m_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u8_from_m_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0073::batch_mode_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::section_0073::error_stop_mode_TYPENUM;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::Unsigned;
use typenum::{U0, U1, U2, U3};
