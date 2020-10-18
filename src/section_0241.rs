//! @ The following procedure, which is called just before \TeX\ initializes its
//! input and output, establishes the initial values of the date and time.
//! @^system dependencies@>
//! Since standard \PASCAL\ cannot provide such information, something special
//! is needed. The program here simply specifies July 4, 1776, at noon; but
//! users probably want a better approximation to the truth.
//
// @p procedure fix_date_and_time;
#[allow(unused_variables)]
pub(crate) fn fix_date_and_time(globals: &mut TeXGlobals) {
    // begin time:=12*60; {minutes since midnight}
    /// minutes since midnight
    {
        time!(globals) = 12 * 60;
    }
    // day:=4; {fourth day of the month}
    /// fourth day of the month
    {
        day!(globals) = 4;
    }
    // month:=7; {seventh month of the year}
    /// seventh month of the year
    {
        month!(globals) = 7;
    }
    // year:=1776; {Anno Domini}
    /// Anno Domini
    {
        year!(globals) = 1776;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
