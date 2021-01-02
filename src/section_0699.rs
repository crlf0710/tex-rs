//! @* \[35] Subroutines for math mode.
//! In order to convert mlists to hlists, i.e., noads to nodes, we need several
//! subroutines that are conveniently dealt with now.
//!
//! Let us first introduce the macros that make it easy to get at the parameters and
//! other font information. A size code, which is a multiple of 16, is added to a
//! family number to get an index into the table of internal font numbers
//! for each combination of family and size.  (Be alert: Size codes get
//! larger as the type gets smaller.)
//
// @d text_size=0 {size code for the largest size in a family}
/// size code for the largest size in a family
pub(crate) const text_size: quarterword = 0;
// @d script_size=16 {size code for the medium size in a family}
/// size code for the medium size in a family
pub(crate) const script_size: quarterword = 16;
// @d script_script_size=32 {size code for the smallest size in a family}
/// size code for the smallest size in a family
pub(crate) const script_script_size: quarterword = 32;
//
// @<Basic printing procedures@>=
// procedure print_size(@!s:integer);
// begin if s=text_size then print_esc("textfont")
// else if s=script_size then print_esc("scriptfont")
// else print_esc("scriptscriptfont");
// end;
//

use crate::section_0113::quarterword;
