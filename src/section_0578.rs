//! @ The following routine is used to implement `\.{\\fontdimen} |n| |f|'.
//! The boolean parameter |writing| is set |true| if the calling program
//! intends to change the parameter value.
//
// @<Declare procedures that scan font-related stuff@>=
// procedure find_font_dimen(@!writing:boolean);
//   {sets |cur_val| to |font_info| location}
/// sets `cur_val` to `font_info` location
pub(crate) fn find_font_dimen(globals: &mut TeXGlobals, writing: boolean) -> TeXResult<()> {
    // var f:internal_font_number;
    let f: internal_font_number;
    // @!n:integer; {the parameter number}
    /// the parameter number
    let n: integer;
    // begin scan_int; n:=cur_val; scan_font_ident; f:=cur_val;
    scan_int(globals)?;
    n = globals.cur_val;
    scan_font_ident(globals)?;
    f = internal_font_number::new(globals.cur_val as _);
    // if n<=0 then cur_val:=fmem_ptr
    if n <= 0 {
        globals.cur_val = globals.fmem_ptr.get() as _;
    }
    // else  begin if writing and(n<=space_shrink_code)and@|
    // (n>=space_code)and(font_glue[f]<>null) then
    else {
        if writing && n <= space_shrink_code.into() && n >= space_code.into() && globals.font_glue[f] != null {
            // begin delete_glue_ref(font_glue[f]);
            delete_glue_ref(globals, globals.font_glue[f]);
            // font_glue[f]:=null;
            globals.font_glue[f] = null;
            // end;
        }
        // if n>font_params[f] then
        if n as integer > globals.font_params[f].get() as integer {
            // if f<font_ptr then cur_val:=fmem_ptr
            if f < globals.font_ptr {
                globals.cur_val = globals.fmem_ptr.get() as _;
            }
            // else @<Increase the number of parameters in the last font@>
            else {
                Increase_the_number_of_parameters_in_the_last_font!(globals, n, f);
            }
        }
        // else cur_val:=n+param_base[f];
        else {
            globals.cur_val = n + globals.param_base[f];
        }
        // end;
    }
    // @<Issue an error message if |cur_val=fmem_ptr|@>;
    Issue_an_error_message_if_cur_val_eq_fmem_ptr!(globals);
    // end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0201::delete_glue_ref;
use crate::section_0440::scan_int;
use crate::section_0547::space_code;
use crate::section_0547::space_shrink_code;
use crate::section_0548::internal_font_number;
use crate::section_0577::scan_font_ident;
