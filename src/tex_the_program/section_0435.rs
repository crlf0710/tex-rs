//! @ While we're at it, we might as well deal with similar routines that
//! will be needed later.
//
// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_four_bit_int;
pub(crate) fn scan_four_bit_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>15) then
    if globals.cur_val < 0 || globals.cur_val > 15 {
        todo!("not yet implemented in {}", file!());
        // begin print_err("Bad number");
        // @.Bad number@>
        // help2("Since I expected to read a number between 0 and 15,")@/
        //   ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0440::scan_int;
