//! ` `
// @<Declare procedures that scan restricted classes of integers@>=
// procedure scan_char_num;
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn scan_char_num(globals: &mut TeXGlobals, extended_max: boolean) -> TeXResult<()> {
    // begin scan_int;
    scan_int(globals)?;
    let val_max = if extended_max {
        crate::pascal::char::MAX.0 as integer
    } else {
        255
    };
    // if (cur_val<0)or(cur_val>255) then
    if globals.cur_val < 0 || globals.cur_val > val_max {
        // begin print_err("Bad character code");
        print_err!(globals, strpool_str!("Bad character code"));
        // @.Bad character code@>
        // help2("A character number must be between 0 and 255.")@/
        //   ("I changed this one to zero."); int_error(cur_val); cur_val:=0;
        help2!(globals, strpool_str!("A character number must be between 0 and 255."),
            strpool_str!("I changed this one to zero."));
        int_error(globals, globals.cur_val)?;
        globals.cur_val = 0;
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0091::int_error;
use crate::section_0440::scan_int;