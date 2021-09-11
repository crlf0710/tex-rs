//! ` `
// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_fifteen_bit_int;
pub(crate) fn scan_fifteen_bit_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>@'77777) then
    if globals.cur_val < 0 || globals.cur_val > 0o77777 {
        // begin print_err("Bad mathchar");
        print_err!(globals, crate::strpool_str!("Bad mathchar"));
        // @.Bad mathchar@>
        // help2("A mathchar number must be between 0 and 32767.")@/
        //   ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
        help2!(
            globals,
            crate::strpool_str!("A mathchar number must be between 0 and 32767."),
            crate::strpool_str!("I changed this one to zero.")
        );
        int_error(globals, globals.cur_val)?;
        globals.cur_val = 0;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0073::print_err;
use crate::section_0079::help2;
use crate::section_0081::TeXResult;
use crate::section_0091::int_error;
use crate::section_0440::scan_int;
