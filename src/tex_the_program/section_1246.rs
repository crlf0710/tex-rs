//! ` `
// @<Declare subprocedures for |prefixed_command|@>=
// procedure alter_integer;
pub(crate) fn alter_integer(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:0..1; {0 for \.{\\deadcycles}, 1 for \.{\\insertpenalties}}
    /// 0 for `\deadcycles`, 1 for `\insertpenalties`
    let c;
    // begin c:=cur_chr; scan_optional_equals; scan_int;
    c = globals.cur_chr.get();
    scan_optional_equals(globals)?;
    scan_int(globals)?;
    // if c=0 then dead_cycles:=cur_val
    if c == 0 {
        globals.dead_cycles = globals.cur_val;
    }
    // else insert_penalties:=cur_val;
    else {
        globals.insert_penalties = globals.cur_val;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0405::scan_optional_equals;
use crate::section_0440::scan_int;
