//! ` `
// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_fifteen_bit_int;
pub(crate) fn scan_fifteen_bit_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>@'77777) then
    //   begin print_err("Bad mathchar");
    // @.Bad mathchar@>
    //   help2("A mathchar number must be between 0 and 32767.")@/
    //     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
    //   end;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0440::scan_int;
