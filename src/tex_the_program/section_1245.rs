//! ` `
// @<Declare subprocedures for |prefixed_command|@>=
// procedure alter_page_so_far;
pub(crate) fn alter_page_so_far(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:0..7; {index into |page_so_far|}
    /// index into `page_so_far`
    let c;
    // begin c:=cur_chr; scan_optional_equals; scan_normal_dimen;
    c = globals.cur_chr.get();
    scan_optional_equals(globals)?;
    scan_normal_dimen!(globals)?;
    // page_so_far[c]:=cur_val;
    globals.page_so_far[c as usize] = scaled::new_from_inner(globals.cur_val);
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0405::scan_optional_equals;
use crate::section_0448::scan_normal_dimen;
