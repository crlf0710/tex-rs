//! @ The |show_node_list| routine requires some auxiliary subroutines: one to
//! print a font-and-character combination, one to print a token list without
//! its reference count, and one to print a rule dimension.
//
// @p procedure print_font_and_char(@!p:integer); {prints |char_node| data}
/// prints `char_node` data
#[allow(unused_variables)]
pub(crate) fn print_font_and_char(globals: &mut TeXGlobals, p: integer) {
    // begin if p>mem_end then print_esc("CLOBBERED.")
    if p > globals.mem_end as integer {
        print_esc(globals, strpool_str!("CLOBBERED."));
    }
    // else  begin if (font(p)<font_base)or(font(p)>font_max) then print_char("*")
    else {
        if (font!(globals, p as pointer).get() as integer) < font_base as integer
            || (font!(globals, p as pointer).get() as integer) > font_max as integer
        {
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'*'),
            );
        }
        // @.*\relax@>
        // else @<Print the font identifier for |font(p)|@>;
        else {
            Print_the_font_identifier_for_font_p!(globals, p as pointer);
        }
        // print_char(" "); print_ASCII(qo(character(p)));
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b' '),
        );
        let c = character!(globals, p as pointer);
        print_ASCII(globals, c.numeric_value() as _);
        // end;
    }
    // end;
}
// @#
// procedure print_mark(@!p:integer); {prints token list data in braces}
// begin print_char("{");
// if (p<hi_mem_min)or(p>mem_end) then print_esc("CLOBBERED.")
// else show_token_list(link(p),null,max_print_line-10);
// print_char("}");
// end;
// @#
// procedure print_rule_dimen(@!d:scaled); {prints dimension in rule node}
// begin if is_running(d) then print_char("*") else print_scaled(d);
// @.*\relax@>
// end;
//

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0011::font_max;
use crate::section_0012::font_base;
use crate::section_0058::print_char;
use crate::section_0063::print_esc;
use crate::section_0068::print_ASCII;
use crate::section_0115::pointer;
