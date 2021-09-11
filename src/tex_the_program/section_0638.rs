//! @ The |hlist_out| and |vlist_out| procedures are now complete, so we are
//! ready for the |ship_out| routine that gets them started in the first place.
//
// @p procedure ship_out(@!p:pointer); {output the box |p|}
/// output the box `p`
#[allow(unused_variables)]
pub(crate) fn ship_out(globals: &mut TeXGlobals, p: pointer) -> TeXResult<()> {
    // label done;
    // var page_loc:integer; {location of the current |bop|}
    // @!j,@!k:0..9; {indices to first ten count registers}
    /// indices to first ten count registers
    let mut j;
    // @!s:pool_pointer; {index into |str_pool|}
    // @!old_setting:0..max_selector; {saved |selector| setting}
    // begin if tracing_output>0 then
    if tracing_output!(globals) > 0 {
        // begin print_nl(""); print_ln;
        print_nl(globals, crate::strpool_str!(""));
        print_ln(make_globals_io_string_log_view!(globals));
        // print("Completed box being shipped out");
        print(
            globals,
            crate::strpool_str!("Completed box being shipped out").get() as _,
        );
        // @.Completed box...@>
        // end;
    }
    // if term_offset>max_print_line-9 then print_ln
    if globals.term_offset > globals.max_print_line - 9 {
        print_ln(make_globals_io_string_log_view!(globals));
    }
    // else if (term_offset>0)or(file_offset>0) then print_char(" ");
    else if globals.term_offset > 0 || globals.file_offset > 0 {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b' '),
        );
    }
    // print_char("["); j:=9;
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'['),
    );
    j = 9;
    // while (count(j)=0)and(j>0) do decr(j);
    while count!(globals, j) == 0 && j > 0 {
        decr!(j);
    }
    // for k:=0 to j do
    for k in 0..=j {
        // begin print_int(count(k));
        print_int(globals, count!(globals, k));
        // if k<j then print_char(".");
        if k < j {
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'.'),
            );
        }
        // end;
    }
    // update_terminal;
    update_terminal(globals);
    // if tracing_output>0 then
    if tracing_output!(globals) > 0 {
        // begin print_char("]");
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b']'),
        );
        // begin_diagnostic; show_box(p); end_diagnostic(true);
        begin_diagnostic(globals);
        show_box(globals, p);
        end_diagnostic(globals, true);
        // end;
    }
    // @<Ship box |p| out@>;
    crate::section_0640::Ship_box_p_out!(globals, p);
    // if tracing_output<=0 then print_char("]");
    if tracing_output!(globals) <= 0 {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b']'),
        );
    }
    // dead_cycles:=0;
    globals.dead_cycles = 0;
    // update_terminal; {progress report}
    /// progress report
    const _: () = ();
    update_terminal(globals);
    // @<Flush the box from memory, showing statistics if requested@>;
    crate::section_0639::Flush_the_box_from_memory__showing_statistics_if_requested!(globals, p);
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0016::decr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0034::update_terminal;
use crate::section_0057::print_ln;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0062::print_nl;
use crate::section_0065::print_int;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0198::show_box;
use crate::section_0236::count;
use crate::section_0236::tracing_output;
use crate::section_0245::begin_diagnostic;
use crate::section_0245::end_diagnostic;
