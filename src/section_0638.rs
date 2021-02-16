//! @ The |hlist_out| and |vlist_out| procedures are now complete, so we are
//! ready for the |ship_out| routine that gets them started in the first place.
//
// @p procedure ship_out(@!p:pointer); {output the box |p|}
/// output the box `p`
#[allow(unused_variables)]
pub(crate) fn ship_out(globals: &mut TeXGlobals, p: pointer) {
    // label done;
    // var page_loc:integer; {location of the current |bop|}
    // @!j,@!k:0..9; {indices to first ten count registers}
    // @!s:pool_pointer; {index into |str_pool|}
    // @!old_setting:0..max_selector; {saved |selector| setting}
    // begin if tracing_output>0 then
    //   begin print_nl(""); print_ln;
    //   print("Completed box being shipped out");
    // @.Completed box...@>
    //   end;
    // if term_offset>max_print_line-9 then print_ln
    // else if (term_offset>0)or(file_offset>0) then print_char(" ");
    // print_char("["); j:=9;
    // while (count(j)=0)and(j>0) do decr(j);
    // for k:=0 to j do
    //   begin print_int(count(k));
    //   if k<j then print_char(".");
    //   end;
    // update_terminal;
    // if tracing_output>0 then
    //   begin print_char("]");
    //   begin_diagnostic; show_box(p); end_diagnostic(true);
    //   end;
    // @<Ship box |p| out@>;
    // if tracing_output<=0 then print_char("]");
    // dead_cycles:=0;
    // update_terminal; {progress report}
    // @<Flush the box from memory, showing statistics if requested@>;
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
