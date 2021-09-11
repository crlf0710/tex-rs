//! @ The following procedure, which is called just before \TeX\ initializes its
//! input and output, establishes the initial values of the date and time.
//! @^system dependencies@>
//! Since standard \PASCAL\ cannot provide such information, something special
//! is needed. The program here simply assumes that suitable values appear in
//! the global variables \\{sys\_time}, \\{sys\_day}, \\{sys\_month}, and
//! \\{sys\_year} (which are initialized to noon on 4 July 1776,
//! in case the implementor is careless).
//
// @p procedure fix_date_and_time;
#[allow(unused_variables)]
pub(crate) fn fix_date_and_time(globals: &mut TeXGlobals) {
    // begin sys_time:=12*60;
    {
        globals.sys_time = 12 * 60;
    }
    // sys_day:=4; sys_month:=7; sys_year:=1776;  {self-evident truths}
    /// self-evident truths
    {
        globals.sys_day = 4;
        globals.sys_month = 7;
        globals.sys_year = 1776;
    }
    // time:=sys_time; {minutes since midnight}
    /// minutes since midnight
    {
        time!(globals) = globals.sys_time;
    }
    // day:=sys_day; {day of the month}
    /// fourth day of the month
    {
        day!(globals) = globals.sys_day;
    }
    // month:=sys_month; {month of the year}
    /// seventh month of the year
    {
        month!(globals) = globals.sys_month;
    }
    // year:=sys_year; {Anno Domini}
    /// Anno Domini
    {
        year!(globals) = globals.sys_year;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0236::day;
use crate::section_0236::month;
use crate::section_0236::time;
use crate::section_0236::year;
