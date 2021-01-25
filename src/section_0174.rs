//! @ Boxes, rules, inserts, whatsits, marks, and things in general that are
//! sort of ``complicated'' are indicated only by printing `\.{[]}'.
//
// @p procedure short_display(@!p:integer); {prints highlights of list |p|}
/// prints highlights of list `p`
pub(crate) fn short_display(globals: &mut TeXGlobals, mut p: integer) {
    // var n:integer; {for replacement counts}
    // begin while p>mem_min do
    while p > mem_min as integer {
        // begin if is_char_node(p) then
        if is_char_node!(globals, p as pointer) {
            // begin if p<=mem_end then
            if p <= globals.mem_end as integer {
                // begin if font(p)<>font_in_short_display then
                if font!(globals, p as pointer).get() as integer != globals.font_in_short_display {
                    // begin if (font(p)<font_base)or(font(p)>font_max) then
                    if (font!(globals, p as pointer).get() as integer) < font_base as integer
                        || (font!(globals, p as pointer).get() as integer) > font_max as integer
                    {
                        // print_char("*")
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
                    // print_char(" "); font_in_short_display:=font(p);
                    print_char(
                        make_globals_io_string_log_view!(globals),
                        ASCII_code_literal!(b' '),
                    );
                    globals.font_in_short_display = font!(globals, p as pointer).get() as _;
                    // end;
                }
                // print_ASCII(qo(character(p)));
                let c = character!(globals, p as pointer);
                print_ASCII(globals, c.numeric_value() as _);
                // end;
            }
        // end
        }
        // else @<Print a short indication of the contents of node |p|@>;
        else {
            todo!("Print a short indication");
        }
        // p:=link(p);
        p = link!(globals, p as pointer) as integer;
        // end;
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0011::font_max;
use crate::section_0011::mem_min;
use crate::section_0012::font_base;
use crate::section_0058::print_char;
use crate::section_0068::print_ASCII;
use crate::section_0115::pointer;
