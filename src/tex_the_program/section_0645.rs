//! @ The parameters to |hpack| and |vpack| correspond to \TeX's primitives
//! like `\.{\\hbox} \.{to} \.{300pt}', `\.{\\hbox} \.{spread} \.{10pt}'; note
//! that `\.{\\hbox}' with no dimension following it is equivalent to
//! `\.{\\hbox} \.{spread} \.{0pt}'.  The |scan_spec| subroutine scans such
//! constructions in the user's input, including the mandatory left brace that
//! follows them, and it puts the specification onto |save_stack| so that the
//! desired box can later be obtained by executing the following code:
//! $$\vbox{\halign{#\hfil\cr
//! |save_ptr:=save_ptr-2;|\cr
//! |hpack(p,saved(1),saved(0)).|\cr}}$$
//! Special care is necessary to ensure that the special |save_stack| codes
//! are placed just below the new group code, because scanning can change
//! |save_stack| when \.{\\csname} appears.
//
// @p procedure scan_spec(@!c:group_code;@!three_codes:boolean);
//   {scans a box specification and left brace}
/// scans a box specification and left brace
pub(crate) fn scan_spec(globals: &mut TeXGlobals, c: group_code, three_codes: boolean) -> TeXResult<()> {
    // label found;
    // var @!s:integer; {temporarily saved value}
    /// temporarily saved value
    let mut s: integer = 0;
    // @!spec_code:exactly..additional;
    let spec_code;
    // begin if three_codes then s:=saved(0);
    if three_codes {
        s = saved!(globals, 0);
    }
    region_forward_label!(
    |'found|
    {
        // if scan_keyword("to") then spec_code:=exactly
        if scan_keyword(globals, strpool_str!("to"))? {
            spec_code = exactly;
        }
        // @.to@>
        // else if scan_keyword("spread") then spec_code:=additional
        else if scan_keyword(globals, strpool_str!("spread"))? {
            spec_code = additional;
        }
        // @.spread@>
        // else  begin spec_code:=additional; cur_val:=0;
        else {
            spec_code = additional;
            globals.cur_val = 0;
            // goto found;
            goto_forward_label!('found);
            // end;
        }
        // scan_normal_dimen;
        scan_normal_dimen!(globals)?;
    }
    // found: if three_codes then
    'found <-
    );
    
    if three_codes {
        // begin saved(0):=s; incr(save_ptr);
        saved!(globals, 0) = s;
        incr!(globals.save_ptr);
        // end;
    }
    // saved(0):=spec_code; saved(1):=cur_val; save_ptr:=save_ptr+2;
    saved!(globals, 0) = spec_code.into();
    saved!(globals, 1) = globals.cur_val;
    globals.save_ptr = globals.save_ptr + 2;
    // new_save_level(c); scan_left_brace;
    new_save_level(globals, c);
    scan_left_brace(globals)?;
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0269::group_code;
use crate::section_0274::new_save_level;
use crate::section_0403::scan_left_brace;
use crate::section_0407::scan_keyword;
use crate::section_0644::additional;
use crate::section_0644::exactly;
