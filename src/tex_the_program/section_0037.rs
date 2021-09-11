//! @ The following program does the required initialization
//! without retrieving a possible command line.
//! It should be clear how to modify this routine to deal with command lines,
//! if the system permits them.
//! @^system dependencies@>
//
// @p function init_terminal:boolean; {gets the terminal input started}
/// gets the terminal input started
pub(crate) fn init_terminal(globals: &mut TeXGlobals) -> boolean {
    // label exit;
    // begin t_open_in;
    t_open_in(globals);
    // loop@+begin wake_up_terminal; write(term_out,'**'); update_terminal;
    loop {
        wake_up_terminal(globals);
        write(&mut globals.term_out, "**");
        update_terminal(globals);
        // @.**@>
        //   if not input_ln(term_in,true) then {this shouldn't happen}
        if !input_ln(make_globals_io_view!(globals), &mut globals.term_in, true) {
            /// this shouldn't happen
            const _: () = ();
            // begin write_ln(term_out);
            write_ln_noargs(&mut globals.term_out);
            // write(term_out,'! End of file on the terminal... why?');
            write(
                &mut globals.term_out,
                "! End of file on the terminal... why?",
            );
            // @.End of file on the terminal@>
            // init_terminal:=false; return;
            return false;
            // end;
        }
        // loc:=first;
        loc!(globals) = globals.first.get();
        // while (loc<last)and(buffer[loc]=" ") do incr(loc);
        while loc!(globals) < globals.last.get()
            && globals.buffer[loc!(globals)] == ASCII_code_literal!(b' ')
        {
            incr!(loc!(globals));
        }
        // if loc<last then
        if loc!(globals) < globals.last.get() {
            // begin init_terminal:=true;
            // return; {return unless the line was all blank}
            /// return unless the line was all blank
            return true;
            // end;
        }
        // write_ln(term_out,'Please type the name of your input file.');
        write_ln(
            &mut globals.term_out,
            "Please type the name of your input file.",
        );
        // end;
    }
    // exit:end;
}

use crate::pascal::{boolean, write, write_ln, write_ln_noargs};
use crate::section_0004::make_globals_io_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0031::input_ln;
use crate::section_0033::t_open_in;
use crate::section_0034::update_terminal;
use crate::section_0034::wake_up_terminal;
use crate::section_0036::loc;
