//! @ Single-character control sequences do not need to be looked up in a hash
//! table, since we can use the character code itself as a direct address.
//! The procedure |print_cs| prints the name of a control sequence, given
//! a pointer to its address in |eqtb|. A space is printed after the name
//! unless it is a single nonletter or an active character. This procedure
//! might be invoked with invalid data, so it is ``extra robust.'' The
//! individual characters must be printed one at a time using |print|, since
//! they may be unprintable.
//
// @<Basic printing...@>=
// procedure print_cs(@!p:integer); {prints a purported control sequence}
/// prints a purported control sequence
#[allow(unused_comparisons)]
pub(crate) fn print_cs(globals: &mut TeXGlobals, p: integer) {
    // begin if p<hash_base then {single character}
    if p < hash_base as integer {
        /// single character
        const _: () = ();
        // if p>=single_base then
        if p >= single_base as integer {
            // if p=null_cs then
            if p == null_cs as integer {
                // begin print_esc("csname"); print_esc("endcsname"); print_char(" ");
                print_esc(globals, crate::strpool_str!("csname"));
                print_esc(globals, crate::strpool_str!("endcsname"));
                print_char(
                    make_globals_io_string_log_view!(globals),
                    ASCII_code_literal!(b' '),
                );
            // end
            }
            // else  begin print_esc(p-single_base);
            else {
                print_esc(globals, str_number::new((p - single_base as integer) as _));
                // if cat_code(p-single_base)=letter then print_char(" ");
                if cat_code!(globals, ASCII_code::from(p - single_base as integer))
                    == letter as halfword
                {
                    print_char(
                        make_globals_io_string_log_view!(globals),
                        ASCII_code_literal!(b' '),
                    );
                }
                // end
            }
        }
        // else if p<active_base then print_esc("IMPOSSIBLE.")
        else if p < active_base as integer {
            print_esc(globals, crate::strpool_str!("IMPOSSIBLE."));
        }
        // @.IMPOSSIBLE@>
        // else print(p-active_base)
        else {
            print(globals, p - active_base as integer);
        }
    }
    // else if p>=undefined_control_sequence then print_esc("IMPOSSIBLE.")
    else if p >= undefined_control_sequence as _ {
        print_esc(globals, crate::strpool_str!("IMPOSSIBLE."));
    }
    // else if (text(p)<0)or(text(p)>=str_ptr) then print_esc("NONEXISTENT.")
    else if text!(globals, p as pointer) < 0
        || text!(globals, p as pointer) >= globals.str_ptr.get() as _
    {
        print_esc(globals, crate::strpool_str!("IMPOSSIBLE."));
    }
    // @.NONEXISTENT@>
    // else  begin print_esc(text(p));
    else {
        print_esc(globals, str_number::new(text!(globals, p as pointer) as _));
        // print_char(" ");
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b' '),
        );
        // end;
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code;
use crate::section_0018::ASCII_code_literal;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0207::letter;
use crate::section_0222::active_base;
use crate::section_0222::hash_base;
use crate::section_0222::null_cs;
use crate::section_0222::single_base;
use crate::section_0222::undefined_control_sequence;
use crate::section_0230::cat_code;
use crate::section_0256::text;
