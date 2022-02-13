//! ` `
// @<Declare subprocedures for |prefixed_command|@>=
// procedure alter_prev_graf;
pub(crate) fn alter_prev_graf(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p:0..nest_size; {index into |nest|}
    /// index into `nest`
    let mut p;
    // begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
    globals.nest[globals.nest_ptr] = globals.cur_list;
    p = globals.nest_ptr;
    // while abs(nest[p].mode_field)<>vmode do decr(p);
    while globals.nest[p].mode_field.get().abs() != vmode {
        decr!(p);
    }
    // scan_optional_equals; scan_int;
    scan_optional_equals(globals)?;
    scan_int(globals)?;
    // if cur_val<0 then
    if globals.cur_val < 0 {
        // begin print_err("Bad "); print_esc("prevgraf");
        // @.Bad \\prevgraf@>
        // help1("I allow only nonnegative values here.");
        // int_error(cur_val);
        // end
        todo!("bad prevgraf");
    }
    // else  begin nest[p].pg_field:=cur_val; cur_list:=nest[nest_ptr];
    else {
        globals.nest[p].pg_field = globals.cur_val;
        globals.cur_list = globals.nest[globals.nest_ptr];
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0081::TeXResult;
use crate::section_0211::vmode;
use crate::section_0405::scan_optional_equals;
use crate::section_0440::scan_int;
