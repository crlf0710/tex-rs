//! @ The remaining command codes are extra special, since they cannot get through
//! \TeX's scanner to the main control routine. They have been given values higher
//! than |max_command| so that their special nature is easily discernible.
//! The ``expandable'' commands come first.
//
// @d undefined_cs=max_command+1 {initial state of most |eq_type| fields}
/// initial state of most `eq_type` fields
pub(crate) const undefined_cs: quarterword = max_command + 1;
// @d expand_after=max_command+2 {special expansion ( \.{\\expandafter} )}
// @d no_expand=max_command+3 {special nonexpansion ( \.{\\noexpand} )}
// @d input=max_command+4 {input a source file ( \.{\\input}, \.{\\endinput} )}
// @d if_test=max_command+5 {conditional text ( \.{\\if}, \.{\\ifcase}, etc.~)}
// @d fi_or_else=max_command+6 {delimiters for conditionals ( \.{\\else}, etc.~)}
// @d cs_name=max_command+7 {make a control sequence from tokens ( \.{\\csname} )}
// @d convert=max_command+8 {convert to text ( \.{\\number}, \.{\\string}, etc.~)}
// @d the=max_command+9 {expand an internal quantity ( \.{\\the} )}
/// expand an internal quantity ( `\the` )
pub(crate) const the: quarterword = max_command + 9;
// @d top_bot_mark=max_command+10 {inserted mark ( \.{\\topmark}, etc.~)}
// @d call=max_command+11 {non-long, non-outer control sequence}
/// non-long, non-outer control sequence
pub(crate) const call: quarterword = max_command + 11;
pub(crate) type call_TYPENUM = typenum::op!(max_command_TYPENUM + U11);
// @d long_call=max_command+12 {long, non-outer control sequence}
/// long, non-outer control sequence
pub(crate) const long_call: quarterword = max_command + 12;
// @d outer_call=max_command+13 {non-long, outer control sequence}
/// non-long, outer control sequence
pub(crate) const outer_call: quarterword = max_command + 13;
// @d long_outer_call=max_command+14 {long, outer control sequence}
/// long, outer control sequence
pub(crate) const long_outer_call: quarterword = max_command + 14;
pub(crate) type long_outer_call_TYPENUM = typenum::op!(max_command_TYPENUM + U14);
// @d end_template=max_command+15 {end of an alignment template}
/// end of an alignment template
pub(crate) const end_template: quarterword = max_command + 15;
// @d dont_expand=max_command+16 {the following token was marked by \.{\\noexpand}}
/// the following token was marked by `\noexpand`
pub(crate) const dont_expand: quarterword = max_command + 16;
// @d glue_ref=max_command+17 {the equivalent points to a glue specification}
// @d shape_ref=max_command+18 {the equivalent points to a parshape specification}
// @d box_ref=max_command+19 {the equivalent points to a box node, or is |null|}
// @d data=max_command+20 {the equivalent is simply a halfword number}
/// the equivalent is simply a halfword number
pub(crate) const data: quarterword = max_command + 20;

use crate::section_0113::quarterword;
use crate::section_0209::max_command;
use crate::section_0209::max_command_TYPENUM;
use typenum::{U11, U14};
