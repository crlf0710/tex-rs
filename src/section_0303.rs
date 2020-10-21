//! @ Let's look more closely now at the control variables
//! (|state|,~|index|,~|start|,~|loc|,~|limit|,~|name|),
//! assuming that \TeX\ is reading a line of characters that have been input
//! from some file or from the user's terminal. There is an array called
//! |buffer| that acts as a stack of all lines of characters that are
//! currently being read from files, including all lines on subsidiary
//! levels of the input stack that are not yet completed. \TeX\ will return to
//! the other lines when it is finished with the present input file.
//!
//! (Incidentally, on a machine with byte-oriented addressing, it might be
//! appropriate to combine |buffer| with the |str_pool| array,
//! letting the buffer entries grow downward from the top of the string pool
//! and checking that these two tables don't bump into each other.)
//!
//! The line we are currently working on begins in position |start| of the
//! buffer; the next character we are about to read is |buffer[loc]|; and
//! |limit| is the location of the last character present.  If |loc>limit|,
//! the line has been completely read. Usually |buffer[limit]| is the
//! |end_line_char|, denoting the end of a line, but this is not
//! true if the current line is an insertion that was entered on the user's
//! terminal in response to an error message.
//!
//! The |name| variable is a string number that designates the name of
//! the current file, if we are reading a text file. It is zero if we
//! are reading from the terminal; it is |n+1| if we are reading from
//! input stream |n|, where |0<=n<=16|. (Input stream 16 stands for
//! an invalid stream number; in such cases the input is actually from
//! the terminal, under control of the procedure |read_toks|.)
//!
//! The |state| variable has one of three values, when we are scanning such
//! files:
//! $$\baselineskip 15pt\vbox{\halign{#\hfil\cr
//! 1) |state=mid_line| is the normal state.\cr
//! 2) |state=skip_blanks| is like |mid_line|, but blanks are ignored.\cr
//! 3) |state=new_line| is the state at the beginning of a line.\cr}}$$
//! These state values are assigned numeric codes so that if we add the state
//! code to the next character's command code, we get distinct values. For
//! example, `|mid_line+spacer|' stands for the case that a blank
//! space character occurs in the middle of a line when it is not being
//! ignored; after this case is processed, the next value of |state| will
//! be |skip_blanks|.

// @d mid_line=1 {|state| code when scanning a line of characters}
/// `state` code when scanning a line of characters
pub(crate) const mid_line: quarterword = 1;
// @d skip_blanks=2+max_char_code {|state| code when ignoring blanks}
// @d new_line=3+max_char_code+max_char_code {|state| code at start of line}
/// `state` code at start of line
pub(crate) const new_line: quarterword = 3 + max_char_code + max_char_code;

use crate::section_0113::quarterword;
use crate::section_0207::max_char_code;
