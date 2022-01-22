//! @ Delimiter fields of noads are filled in by the |scan_delimiter| routine.
//! The first parameter of this procedure is the |mem| address where the
//! delimiter is to be placed; the second tells if this delimiter follows
//! \.{\\radical} or not.
//
// @<Declare act...@>=
// procedure scan_delimiter(@!p:pointer;@!r:boolean);
pub(crate) fn scan_delimiter(globals: &mut TeXGlobals, p: pointer, r: boolean) -> TeXResult<()> {
    // begin if r then scan_twenty_seven_bit_int
    if r {
        scan_twenty_seven_bit_int(globals)?;
    }
    // else  begin @<Get the next non-blank non-relax...@>;
    else {
        crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!(globals);
        // case cur_cmd of
        // letter,other_char: cur_val:=del_code(cur_chr);
        if globals.cur_cmd == letter || globals.cur_cmd == other_char {
            globals.cur_val = del_code!(globals, globals.cur_chr.into());
        }
        // delim_num: scan_twenty_seven_bit_int;
        else if globals.cur_cmd == delim_num {
            scan_twenty_seven_bit_int(globals)?;
        }
        // othercases cur_val:=-1
        else {
            globals.cur_val = -1;
            // endcases;
        }
        // end;
    }
    // if cur_val<0 then @<Report that an invalid delimiter code is being changed
    //    to null; set~|cur_val:=0|@>;
    if globals.cur_val < 0 {
        todo!("report error");
    }
    // small_fam(p):=(cur_val div @'4000000) mod 16;
    // small_char(p):=qi((cur_val div @'10000) mod 256);
    let small_fam = internal_font_number::new(((globals.cur_val / 0o4000000) % 16) as _);
    let small_char =
        ASCII_code::from(qi!(((globals.cur_val / 0o10000) % 256) as quarterword) as integer);
    assign_small_fam_and_char!(globals, p, small_fam, small_char);
    // large_fam(p):=(cur_val div 256) mod 16;
    // large_char(p):=qi(cur_val mod 256);
    let large_fam = internal_font_number::new(((globals.cur_val / 256) % 16) as _);
    let large_char = ASCII_code::from(qi!((globals.cur_val % 256) as quarterword) as integer);
    assign_large_fam_and_char!(globals, p, large_fam, large_char);
    // end;
    crate::ok_nojump!()
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0112::qi;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0207::delim_num;
use crate::section_0207::letter;
use crate::section_0207::other_char;
use crate::section_0236::del_code;
use crate::section_0437::scan_twenty_seven_bit_int;
use crate::section_0548::internal_font_number;
use crate::section_0683::assign_large_fam_and_char;
use crate::section_0683::assign_small_fam_and_char;
