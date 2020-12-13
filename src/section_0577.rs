//! @ Before we forget about the format of these tables, let's deal with two
//! of \TeX's basic scanning routines related to font information.
//
// @<Declare procedures that scan font-related stuff@>=
// procedure scan_font_ident;
#[allow(unused_variables)]
pub(crate) fn scan_font_ident(globals: &mut TeXGlobals) {
    todo!();
    // var f:internal_font_number;
    // @!m:halfword;
    // begin @<Get the next non-blank non-call...@>;
    // if cur_cmd=def_font then f:=cur_font
    // else if cur_cmd=set_font then f:=cur_chr
    // else if cur_cmd=def_family then
    //   begin m:=cur_chr; scan_four_bit_int; f:=equiv(m+cur_val);
    //   end
    // else  begin print_err("Missing font identifier");
    // @.Missing font identifier@>
    //   help2("I was looking for a control sequence whose")@/
    //   ("current meaning has been defined by \font.");
    //   back_error; f:=null_font;
    //   end;
    // cur_val:=f;
    // end;
}

use crate::section_0004::TeXGlobals;
