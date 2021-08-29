//! @ Different systems have different ways to get started. But regardless of
//! what conventions are adopted, the routine that initializes the terminal
//! should satisfy the following specifications:
//!
//! \yskip\textindent{1)}It should open file |term_in| for input from the
//!   terminal. (The file |term_out| will already be open for output to the
//!   terminal.)
//!
//! \textindent{2)}If the user has given a command line, this line should be
//!   considered the first line of terminal input. Otherwise the
//!   user should be prompted with `\.{**}', and the first line of input
//!   should be whatever is typed in response.
//!
//! \textindent{3)}The first line of input, which might or might not be a
//!   command line, should appear in locations |first| to |last-1| of the
//!   |buffer| array.
//!
//! \textindent{4)}The global variable |loc| should be set so that the
//!   character to be read next by \TeX\ is in |buffer[loc]|. This
//!   character should not be blank, and we should have |loc<last|.
//!
//! \yskip\noindent(It may be necessary to prompt the user several times
//! before a non-blank line comes in. The prompt is `\.{**}' instead of the
//! later `\.*' because the meaning is slightly different: `\.{\\input}' need
//! not be typed immediately after~`\.{**}'.)

// @d loc==cur_input.loc_field {location of first unread character in |buffer|}
/// location of first unread character in `buffer`
macro_rules! loc {
    ($globals:expr) => {
        $globals.cur_input.loc_field
    };
}
