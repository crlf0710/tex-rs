//! `

// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_twenty_seven_bit_int;
pub(crate) fn scan_twenty_seven_bit_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>@'777777777) then
    if globals.cur_val < 0 || globals.cur_val > 0o777777777 {
        // begin print_err("Bad delimiter code");
        // @.Bad delimiter code@>
        // help2("A numeric delimiter code must be between 0 and 2^{27}-1.")@/
        //   ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
        // end;
        todo!("bad input");
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0440::scan_int;
