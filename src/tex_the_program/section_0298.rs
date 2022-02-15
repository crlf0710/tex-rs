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
pub(crate) macro chr_cmd($globals:expr, $chr_code:expr, $val:expr) {{
    print($globals, $val.get() as _);
    print_ASCII($globals, $chr_code.get() as _);

    use crate::section_0059::print;
    use crate::section_0068::print_ASCII;
}}

pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives($globals:expr, $cmd:expr, $chr_code:expr) {{
    if false {
        unreachable!();
    } else if crate::section_0227::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0227!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0231::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0231!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0239::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0239!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0249::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0249!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0266::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0266!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0335::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0335!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0377::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0377!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0412::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0412!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0417::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0417!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0469::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0469!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0488::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0488!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0492::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0492!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0781::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0781!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_0984::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0984!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1053::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1053!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1059::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1059!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1072::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1072!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1089::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1089!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1108::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1108!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1115::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1115!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1143::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1143!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1157::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1157!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1170::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1170!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1179::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1179!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1189::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1189!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1209::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1209!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1220::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1220!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1223::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1223!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1231::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1231!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1251::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1251!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1255::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1255!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1261::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1261!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1263::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1263!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1273::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1273!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1278::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1278!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1287::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1287!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1292::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1292!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1295::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1295!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else if crate::section_1346::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1346!(
        $globals, $cmd, $chr_code
    ) {
        true
    } else {
        false
    }
}}

// @<Declare the procedure called |print_cmd_chr|@>=
// procedure print_cmd_chr(@!cmd:quarterword;@!chr_code:halfword);
#[allow(unused_variables)]
pub(crate) fn print_cmd_chr(globals: &mut TeXGlobals, cmd: quarterword, chr_code: chr_code_type) {
    // begin case cmd of
    // left_brace: chr_cmd("begin-group character ");
    if cmd == left_brace {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("begin-group character ")
        );
    }
    // right_brace: chr_cmd("end-group character ");
    else if cmd == right_brace {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("end-group character ")
        );
    }
    // math_shift: chr_cmd("math shift character ");
    else if cmd == math_shift {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("math shift character ")
        );
    }
    // mac_param: chr_cmd("macro parameter character ");
    else if cmd == mac_param {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("macro parameter character ")
        );
    }
    // sup_mark: chr_cmd("superscript character ");
    else if cmd == sup_mark {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("superscript character ")
        );
    }
    // sub_mark: chr_cmd("subscript character ");
    else if cmd == sub_mark {
        chr_cmd!(
            globals,
            chr_code,
            crate::strpool_str!("subscript character ")
        );
    }
    // endv: print("end of alignment template");
    else if cmd == endv {
        print(
            globals,
            crate::strpool_str!("end of alignment template").get() as _,
        );
    }
    // spacer: chr_cmd("blank space ");
    else if cmd == spacer {
        chr_cmd!(globals, chr_code, crate::strpool_str!("blank space "));
    }
    // letter: chr_cmd("the letter ");
    else if cmd == letter {
        chr_cmd!(globals, chr_code, crate::strpool_str!("the letter "));
    }
    // other_char: chr_cmd("the character ");
    else if cmd == other_char {
        chr_cmd!(globals, chr_code, crate::strpool_str!("the character "));
    }
    // @t\4@>@<Cases of |print_cmd_chr| for symbolic printing of primitives@>@/
    else if crate::section_0298::Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives!(
        globals, cmd, chr_code
    ) {
        /// already processed
        do_nothing!();
    }
    // othercases print("[unknown command code!]")
    else {
        crate::trace_error_expr!("cmd = {}", cmd);
        print(
            globals,
            crate::strpool_str!("[unknown command code!]").get() as _,
        );
    }
    // endcases;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0016::do_nothing;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0065::print_int;
use crate::section_0113::quarterword;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0209::*;
use crate::section_0210::*;
use crate::section_0230::int_base;
use crate::section_0236::count_base;
use crate::section_0237::print_param;
use crate::section_0297::chr_code_type;
