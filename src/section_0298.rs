//! @ The |print_cmd_chr| routine prints a symbolic interpretation of a
//! command code and its modifier. This is used in certain `\.{You can\'t}'
//! error messages, and in the implementation of diagnostic routines like
//! \.{\\show}.
//!
//! The body of |print_cmd_chr| is a rather tedious listing of print
//! commands, and most of it is essentially an inverse to the |primitive|
//! routine that enters a \TeX\ primitive into |eqtb|. Therefore much of
//! this procedure appears elsewhere in the program,
//! together with the corresponding |primitive| calls.

// @d chr_cmd(#)==begin print(#); print_ASCII(chr_code);
//   end
macro_rules! chr_cmd {
    ($globals:expr, $chr_code:expr, $val:expr) => {{
        print($globals, $val.get() as _);
        print_ASCII($globals, $chr_code.get() as _);
        
        use crate::section_0059::print;
        use crate::section_0068::print_ASCII;
    }}
}

macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        if false {
            unreachable!();
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0266!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0335!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0412!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0488!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1053!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1059!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1209!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1223!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1231!
            ($globals, $cmd, $chr_code) {
            true
        } else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1278!
            ($globals, $cmd, $chr_code) {
            true
        } else {
            false
        }
    }}
}

// @<Declare the procedure called |print_cmd_chr|@>=
// procedure print_cmd_chr(@!cmd:quarterword;@!chr_code:halfword);
#[allow(unused_variables)]
pub(crate) fn print_cmd_chr(globals: &mut TeXGlobals, cmd: quarterword, chr_code: chr_code_type) {
    // begin case cmd of
    // left_brace: chr_cmd("begin-group character ");
    if cmd == left_brace {
        chr_cmd!(globals, chr_code, strpool_str!("begin-group character "));
    }
    // right_brace: chr_cmd("end-group character ");
    // math_shift: chr_cmd("math shift character ");
    // mac_param: chr_cmd("macro parameter character ");
    // sup_mark: chr_cmd("superscript character ");
    // sub_mark: chr_cmd("subscript character ");
    // endv: print("end of alignment template");
    // spacer: chr_cmd("blank space ");
    else if cmd == spacer {
        chr_cmd!(globals, chr_code, strpool_str!("blank space "));
    }
    // letter: chr_cmd("the letter ");
    else if cmd == letter {
        chr_cmd!(globals, chr_code, strpool_str!("the letter "));
    }
    // other_char: chr_cmd("the character ");
    else if cmd == other_char {
        chr_cmd!(globals, chr_code, strpool_str!("the character "));
    }
    // @t\4@>@<Cases of |print_cmd_chr| for symbolic printing of primitives@>@/
    else if Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives!(globals, cmd, chr_code) {
        /// already processed
        do_nothing!();
    }
    // othercases print("[unknown command code!]")
    else {
        trace_error_expr!("cmd = {}", cmd);
        print(globals, strpool_str!("[unknown command code!]").get() as _);
    }
    // endcases;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0113::quarterword;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0209::*;
use crate::section_0210::*;
use crate::section_0297::chr_code_type;