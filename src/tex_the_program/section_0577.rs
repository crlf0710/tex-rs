//! @ Before we forget about the format of these tables, let's deal with two
//! of \TeX's basic scanning routines related to font information.
//
// @<Declare procedures that scan font-related stuff@>=
// procedure scan_font_ident;
#[allow(unused_variables)]
pub(crate) fn scan_font_ident(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var f:internal_font_number;
    let f: internal_font_number;
    // @!m:halfword;
    // begin @<Get the next non-blank non-call...@>;
    crate::section_0406::Get_the_next_non_blank_non_call_token!(globals);
    // if cur_cmd=def_font then f:=cur_font
    if globals.cur_cmd == def_font {
        f = internal_font_number::new(cur_font!(globals) as _);
    }
    // else if cur_cmd=set_font then f:=cur_chr
    else if globals.cur_cmd == set_font {
        f = internal_font_number::new(globals.cur_chr.get() as _);
    }
    // else if cur_cmd=def_family then
    else if globals.cur_cmd == def_family {
        let m: integer;
        // begin m:=cur_chr; scan_four_bit_int; f:=equiv(m+cur_val);
        m = globals.cur_chr.get() as _;
        scan_four_bit_int(globals)?;
        f = equiv!(globals, m + globals.cur_val).into();
        // end
    }
    // else  begin print_err("Missing font identifier");
    else {
        todo!("error");
        // @.Missing font identifier@>
        //   help2("I was looking for a control sequence whose")@/
        //   ("current meaning has been defined by \font.");
        //   back_error; f:=null_font;
        //   end;
    }
    // cur_val:=f;
    globals.cur_val = f.get() as _;
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::halfword;
use crate::section_0209::*;
use crate::section_0221::equiv;
use crate::section_0230::cur_font;
use crate::section_0435::scan_four_bit_int;
use crate::section_0548::internal_font_number;
