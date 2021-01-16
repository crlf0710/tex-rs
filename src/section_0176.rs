//! @ The |show_node_list| routine requires some auxiliary subroutines: one to
//! print a font-and-character combination, one to print a token list without
//! its reference count, and one to print a rule dimension.
//
// @p procedure print_font_and_char(@!p:integer); {prints |char_node| data}
/// prints `char_node` data
#[allow(unused_variables)]
pub(crate) fn print_font_and_char(globals: &mut TeXGlobals, p: integer) {
    // begin if p>mem_end then print_esc("CLOBBERED.")
    // else  begin if (font(p)<font_base)or(font(p)>font_max) then print_char("*")
    // @.*\relax@>
    //   else @<Print the font identifier for |font(p)|@>;
    //   print_char(" "); print_ASCII(qo(character(p)));
    //   end;
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
