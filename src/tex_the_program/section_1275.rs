//! ` `

// @<Declare act...@>=
// procedure open_or_close_in;
pub(crate) fn open_or_close_in(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:0..1; {1 for \.{\\openin}, 0 for \.{\\closein}}
    /// 1 for `\openin`, 0 for `\closein`
    let c: u8_from_0_to_n<U1>;
    // @!n:0..15; {stream number}
    /// stream number
    let n: u8_from_0_to_n<U15>;
    // begin c:=cur_chr; scan_four_bit_int; n:=cur_val;
    c = (globals.cur_chr.get() as u8).into();
    scan_four_bit_int(globals)?;
    n = (globals.cur_val as u8).into();
    // if read_open[n]<>closed then
    if globals.read_open[n.get()] != read_open_kind::closed {
        // begin a_close(read_file[n]); read_open[n]:=closed;
        a_close(&mut globals.read_file[n]);
        globals.read_open[n.get()] = read_open_kind::closed;
        // end;
    }
    // if c<>0 then
    if c != 0 {
        // begin scan_optional_equals; scan_file_name;
        scan_optional_equals(globals)?;
        scan_file_name(globals)?;
        // if cur_ext="" then cur_ext:=".tex";
        if globals.cur_ext == strpool_str!("") {
            globals.cur_ext = strpool_str!(".tex");
        }
        // pack_cur_name;
        pack_cur_name(globals);
        // if a_open_in(read_file[n]) then read_open[n]:=just_open;
        if a_open_in(
            make_globals_filename_view!(globals),
            &mut globals.read_file[n.get()],
        ) {
            globals.read_open[n.get()] = read_open_kind::just_open;
        }
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::pascal::u8_from_0_to_n;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsFilenameView;
use crate::section_0027::a_open_in;
use crate::section_0028::a_close;
use crate::section_0081::TeXResult;
use crate::section_0405::scan_optional_equals;
use crate::section_0435::scan_four_bit_int;
use crate::section_0480::read_open_kind;
use crate::section_0526::scan_file_name;
use crate::section_0529::pack_cur_name;
use typenum::{U1, U15};
