//! ` `

// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_eight_bit_int;
pub(crate) fn scan_eight_bit_int(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>255) then
    if globals.cur_val < 0 || globals.cur_val > 255 {
        // begin print_err("Bad register code");
        print_err!(globals, crate::strpool_str!("Bad register code"));
        // @.Bad register code@>
        // help2("A register number must be between 0 and 255.")@/
        //   ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
        help2!(
            globals,
            crate::strpool_str!("A register number must be between 0 and 255."),
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
