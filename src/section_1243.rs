//! ` `
// @<Declare subprocedures for |prefixed_command|@>=
// procedure alter_aux;
pub(crate) fn alter_aux(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:halfword; {|hmode| or |vmode|}
    /// `hmode` or `vmode`
    let c;
    // begin if cur_chr<>abs(mode) then report_illegal_case
    if globals.cur_chr.get() as integer != mode!(globals).get().abs() as integer {
        report_illegal_case(globals)?;
    }
    // else  begin c:=cur_chr; scan_optional_equals;
    else {
        c = globals.cur_chr.get() as integer;
        scan_optional_equals(globals)?;
        // if c=vmode then
        if c == vmode as integer {
            // begin scan_normal_dimen; prev_depth:=cur_val;
            scan_normal_dimen!(globals)?;
            prev_depth!(globals) = scaled::new_from_inner(globals.cur_val);
            // end
        }
        // else  begin scan_int;
        else {
            scan_int(globals)?;
            // if (cur_val<=0)or(cur_val>32767) then
            if globals.cur_val <= 0 || globals.cur_val > 32767 {
                // begin print_err("Bad space factor");
                // @.Bad space factor@>
                // help1("I allow only values in the range 1..32767 here.");
                // int_error(cur_val);
                // end
                todo!("range error");
            }
            // else space_factor:=cur_val;
            else {
                space_factor!(globals) = globals.cur_val as _;
            }
            // end;
        }
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0211::vmode;
use crate::section_0405::scan_optional_equals;
use crate::section_0440::scan_int;
use crate::section_1050::report_illegal_case;
