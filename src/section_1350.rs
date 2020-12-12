//! @ The next subroutine uses |cur_chr| to decide what sort of whatsit is
//! involved, and also inserts a |write_stream| number.
//
// @<Declare procedures needed in |do_ext...@>=
// procedure new_write_whatsit(@!w:small_number);
pub(crate) fn new_write_whatsit(globals: &mut TeXGlobals, w: small_number) -> TeXResult<()> {
    // begin new_whatsit(cur_chr,w);
    new_whatsit(globals, small_number::new(globals.cur_chr.get() as _), w)?;
    // if w<>write_node_size then scan_four_bit_int
    if w != write_node_size {
        scan_four_bit_int(globals)?;
    }
    // else  begin scan_int;
    else {
        scan_int(globals)?;
        // if cur_val<0 then cur_val:=17
        if globals.cur_val < 0 {
            globals.cur_val = 17;
        }
        // else if cur_val>15 then cur_val:=16;
        else if globals.cur_val > 15 {
            globals.cur_val = 16;
        }
        // end;
    }
    // write_stream(tail):=cur_val;
    write_stream!(globals, tail!(globals)) = globals.cur_val as _;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0435::scan_four_bit_int;
use crate::section_0440::scan_int;
use crate::section_1341::write_node_size;
use crate::section_1349::new_whatsit;

