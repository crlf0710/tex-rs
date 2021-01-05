//! ` `

// @<Declare act...@>=
// procedure open_or_close_in;
pub(crate) fn open_or_close_in(_globals: &mut TeXGlobals) {
    todo!();
    // var c:0..1; {1 for \.{\\openin}, 0 for \.{\\closein}}
    // @!n:0..15; {stream number}
    // begin c:=cur_chr; scan_four_bit_int; n:=cur_val;
    // if read_open[n]<>closed then
    //   begin a_close(read_file[n]); read_open[n]:=closed;
    //   end;
    // if c<>0 then
    //   begin scan_optional_equals; scan_file_name;
    //   if cur_ext="" then cur_ext:=".tex";
    //   pack_cur_name;
    //   if a_open_in(read_file[n]) then read_open[n]:=just_open;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
