//! @ Of course we had better declare a few more global variables, if the previous
//! routines are going to work.
//
// @<Glob...@>=
// @!old_setting:0..max_selector;
#[globals_struct_field(TeXGlobals)]
pub(crate) static diag_old_setting: u8_from_0_to_n<max_selector_TYPENUM> = u8_from_0_to_n::default();
// @!sys_time,@!sys_day,@!sys_month,@!sys_year:integer;
//    {date and time supplied by external system}
/// date and time supplied by external system
#[globals_struct_field(TeXGlobals)]
pub(crate) static sys_time: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static sys_day: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static sys_month: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static sys_year: integer = 0;


#[globals_struct_use(TeXGlobals)]
use crate::section_0054::max_selector_TYPENUM;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u8_from_0_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
