//! ` `
// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_char_num;
pub(crate) fn scan_char_num(globals: &mut TeXGlobals) {
    // begin scan_int;
    scan_int(globals);
    // if (cur_val<0)or(cur_val>255) then
    //   begin print_err("Bad character code");
    // @.Bad character code@>
    //   help2("A character number must be between 0 and 255.")@/
    //     ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0440::scan_int;