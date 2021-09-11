//! @ If the user has set the |pausing| parameter to some positive value,
//! and if nonstop mode has not been selected, each line of input is displayed
//! on the terminal and the transcript file, followed by `\.{=>}'.
//! \TeX\ waits for a response. If the response is simply |carriage_return|, the
//! line is accepted as it stands, otherwise the line typed is
//! used instead of the line in the file.
//
// @p procedure firm_up_the_line;
pub(crate) fn firm_up_the_line(globals: &mut TeXGlobals) {
    // var k:0..buf_size; {an index into |buffer|}
    // begin limit:=last;
    limit!(globals) = globals.last.get();
    // if pausing>0 then if interaction>nonstop_mode then
    //   begin wake_up_terminal; print_ln;
    //   if start<limit then for k:=start to limit-1 do print(buffer[k]);
    //   first:=limit; prompt_input("=>"); {wait for user response}
    // @.=>@>
    //   if last>first then
    //     begin for k:=first to last-1 do {move line down in buffer}
    //       buffer[k+start-first]:=buffer[k];
    //     limit:=start+last-first;
    //     end;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0302::limit;
