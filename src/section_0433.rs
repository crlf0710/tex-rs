//! ` `

// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_eight_bit_int;
pub(crate) fn scan_eight_bit_int(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // begin scan_int;
    scan_int(globals)?;
    // if (cur_val<0)or(cur_val>255) then
    //   begin print_err("Bad register code");
    // @.Bad register code@>
    //   help2("A register number must be between 0 and 255.")@/
    //     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
    //   end;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0440::scan_int;