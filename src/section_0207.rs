//! @* \[15] The command codes.
//! Before we can go any further, we need to define symbolic names for the internal
//! code numbers that represent the various commands obeyed by \TeX. These codes
//! are somewhat arbitrary, but not completely so. For example, the command
//! codes for character types are fixed by the language, since a user says,
//! e.g., `\.{\\catcode \`\\\${} = 3}' to make \.{\char'44} a math delimiter,
//! and the command code |math_shift| is equal to~3. Some other codes have
//! been made adjacent so that |case| statements in the program need not consider
//! cases that are widely spaced, or so that |case| statements can be replaced
//! by |if| statements.
//!
//! At any rate, here is the list, for future reference. First come the
//! ``catcode'' commands, several of which share their numeric codes with
//! ordinary commands when the catcode cannot emerge from \TeX's scanning routine.
//
// @d escape=0 {escape delimiter (called \.\\ in {\sl The \TeX book\/})}
// @:TeXbook}{\sl The \TeX book@>
/// escape delimiter (called `\` in *The TEXbook*)
pub(crate) const escape: quarterword = 0;
// @d relax=0 {do nothing ( \.{\\relax} )}
/// do nothing ( `\relax` )
pub(crate) const relax: quarterword = 0;
// @d left_brace=1 {beginning of a group ( \.\{ )}
/// beginning of a group ( `{` )
pub(crate) const left_brace: quarterword = 1;
// @d right_brace=2 {ending of a group ( \.\} )}
/// ending of a group ( `}` )
pub(crate) const right_brace: quarterword = 2;
// @d math_shift=3 {mathematics shift character ( \.\$ )}
/// mathematics shift character ( `$` )
pub(crate) const math_shift: quarterword = 3;
// @d tab_mark=4 {alignment delimiter ( \.\&, \.{\\span} )}
/// alignment delimiter ( `&`, `\span` )
pub(crate) const tab_mark: quarterword = 4;
// @d car_ret=5 {end of line ( |carriage_return|, \.{\\cr}, \.{\\crcr} )}
/// end of line ( `carriage_return`, `\cr`, `\crcr` )
pub(crate) const car_ret: quarterword = 5;
// @d out_param=5 {output a macro parameter}
/// output a macro parameter
pub(crate) const out_param: quarterword = 5;
// @d mac_param=6 {macro parameter symbol ( \.\# )}
/// macro parameter symbol ( `#` )
pub(crate) const mac_param: quarterword = 6;
// @d sup_mark=7 {superscript ( \.{\char'136} )}
// @d sub_mark=8 {subscript ( \.{\char'137} )}
// @d ignore=9 {characters to ignore ( \.{\^\^@@} )}
/// characters to ignore ( `^^@` )
pub(crate) const ignore: quarterword = 9;
// @d endv=9 {end of \<v_j> list in alignment template}
// @d spacer=10 {characters equivalent to blank space ( \.{\ } )}
/// characters equivalent to blank space ( ` ` )
pub(crate) const spacer: quarterword = 10;
// @d letter=11 {characters regarded as letters ( \.{A..Z}, \.{a..z} )}
/// characters regarded as letters ( `A..Z`, `a..z` )
pub(crate) const letter: quarterword = 11;
// @d other_char=12 {none of the special character types}
/// none of the special character types
pub(crate) const other_char: quarterword = 12;
// @d active_char=13 {characters that invoke macros ( \.{\char`\~} )}
// @d par_end=13 {end of paragraph ( \.{\\par} )}
/// end of paragraph ( `\par` )
pub(crate) const par_end: quarterword = 13;
// @d match=13 {match a macro parameter}
// @d comment=14 {characters that introduce comments ( \.\% )}
/// characters that introduce comments ( `%` )
pub(crate) const comment: quarterword = 14;
// @d end_match=14 {end of parameters to macro}
// @d stop=14 {end of job ( \.{\\end}, \.{\\dump} )}
/// end of job ( `\end`, `\dump` )
pub(crate) const stop: quarterword = 14;
// @d invalid_char=15 {characters that shouldn't appear ( \.{\^\^?} )}
/// characters that shouldn't appear ( `^^?` )
pub(crate) const invalid_char: quarterword = 15;
// @d delim_num=15 {specify delimiter numerically ( \.{\\delimiter} )}
// @d max_char_code=15 {largest catcode for individual characters}
/// largest catcode for individual characters
pub(crate) const max_char_code: quarterword = 15;

use crate::section_0113::halfword;
use crate::section_0113::quarterword;
