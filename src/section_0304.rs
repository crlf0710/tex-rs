//! @ Additional information about the current line is available via the
//! |index| variable, which counts how many lines of characters are present
//! in the buffer below the current level. We have |index=0| when reading
//! from the terminal and prompting the user for each line; then if the user types,
//! e.g., `\.{\\input paper}', we will have |index=1| while reading
//! the file \.{paper.tex}. However, it does not follow that |index| is the
//! same as the input stack pointer, since many of the levels on the input
//! stack may come from token lists. For example, the instruction `\.{\\input
//! paper}' might occur in a token list.
//!
//! The global variable |in_open| is equal to the |index|
//! value of the highest non-token-list level. Thus, the number of partially read
//! lines in the buffer is |in_open+1|, and we have |in_open=index|
//! when we are not reading a token list.
//!
//! If we are not currently reading from the terminal, or from an input
//! stream, we are reading from the file variable |input_file[index]|. We use
//! the notation |terminal_input| as a convenient abbreviation for |name=0|,
//! and |cur_file| as an abbreviation for |input_file[index]|.
//!
//! The global variable |line| contains the line number in the topmost
//! open file, for use in error messages. If we are not reading from
//! the terminal, |line_stack[index]| holds the line number for the
//! enclosing level, so that |line| can be restored when the current
//! file has been read. Line numbers should never be negative, since the
//! negative of the current line number is used to identify the user's output
//! routine in the |mode_line| field of the semantic nest entries.
//!
//! If more information about the input state is needed, it can be
//! included in small arrays like those shown here. For example,
//! the current page or segment number in the input file might be
//! put into a variable |@!page|, maintained for enclosing levels in
//! `\ignorespaces|@!page_stack:array[1..max_in_open] of integer|\unskip'
//! by analogy with |line_stack|.
//! @^system dependencies@>
// @d terminal_input==(name=0) {are we reading from the terminal?}

/// are we reading from the terminal?
pub(crate) fn terminal_input(globals: &TeXGlobals) -> bool {
    name!(globals) == 0
}

// @d cur_file==input_file[index] {the current |alpha_file| variable}
/// the current `alpha_file` variable
macro_rules! cur_file {
    ($globals:expr) => {
        $globals.input_file[index!($globals)]
    };
}

// @<Glob...@>=
// @!in_open : 0..max_in_open; {the number of lines in the buffer, less one}
// @!open_parens : 0..max_in_open; {the number of open text files}

// @!input_file : array[1..max_in_open] of alpha_file;
#[globals_struct_field(TeXGlobals)]
pub(crate) static input_file: input_file_range_array<alpha_file> =
    input_file_range_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0304::input_file_range_array;

pub(crate) type input_file_range_array_LENGTH_TYPENUM = typenum::op!(max_in_open_TYPENUM - U1 + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) input_file_range_array[u8_from_m_to_n<U1, max_in_open_TYPENUM>] =>
    u8; U8; U1; input_file_range_array_LENGTH_TYPENUM
);

// @!line : integer; {current line number in the current source file}
#[globals_struct_field(TeXGlobals)]
/// current line number in the current source file
pub(crate) static line: integer = integer::default();

// @!line_stack : array[1..max_in_open] of integer;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::pascal::u8_from_m_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0011::max_in_open_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
