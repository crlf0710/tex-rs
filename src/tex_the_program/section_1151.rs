//! @ Recall that the |nucleus|, |subscr|, and |supscr| fields in a noad are
//! broken down into subfields called |math_type| and either |info| or
//! |(fam,character)|. The job of |scan_math| is to figure out what to place
//! in one of these principal fields; it looks at the subformula that
//! comes next in the input, and places an encoding of that subformula
//! into a given word of |mem|.
//
// @d fam_in_range==((cur_fam>=0)and(cur_fam<16))
pub(crate) macro fam_in_range($globals:expr) {
    cur_fam!($globals) >= 0 && cur_fam!($globals) < 16
}

// @<Declare act...@>=
// procedure scan_math(@!p:pointer);
#[allow(unused_variables)]
pub(crate) fn scan_math(globals: &mut TeXGlobals, p: pointer) -> TeXResult<()> {
    // label restart,reswitch,exit;
    // var c:integer; {math character code}
    /// math character code
    let c: integer;
    // begin restart:@<Get the next non-blank non-relax...@>;
    crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!(globals);
    // reswitch:case cur_cmd of
    // letter,other_char,char_given: begin c:=ho(math_code(cur_chr));
    if globals.cur_cmd == letter || globals.cur_cmd == other_char || globals.cur_cmd == char_given {
        c = ho!(math_code!(globals, globals.cur_chr.into())) as _;
        // if c=@'100000 then
        if c == 0o100000 {
            // begin @<Treat |cur_chr| as an active character@>;
            // goto restart;
            // end;
            todo!("treat");
        }
        // end;
    }
    // char_num: begin scan_char_num; cur_chr:=cur_val; cur_cmd:=char_given;
    //   goto reswitch;
    //   end;
    // math_char_num: begin scan_fifteen_bit_int; c:=cur_val;
    //   end;
    // math_given: c:=cur_chr;
    // delim_num: begin scan_twenty_seven_bit_int; c:=cur_val div @'10000;
    //   end;
    // othercases @<Scan a subformula enclosed in braces and |return|@>
    else {
        todo!("scan a subformula");
    }
    // endcases;@/
    // math_type(p):=math_char; character(p):=qi(c mod 256);
    math_type!(globals, p) = math_type_kind::math_char as _;
    let ch = ASCII_code::from((c % 256) as integer);
    let fam: internal_font_number;
    // if (c>=var_code)and fam_in_range then fam(p):=cur_fam
    if c >= var_code && fam_in_range!(globals) {
        fam = internal_font_number::new(cur_fam!(globals) as _);
    }
    // else fam(p):=(c div 256) mod 16;
    else {
        fam = internal_font_number::new(((c / 256) % 16) as internal_font_number_repr);
    }
    assign_fam_and_character!(globals, p, fam, ch);
    // exit:end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0112::ho;
use crate::section_0115::pointer;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0230::math_code;
use crate::section_0232::var_code;
use crate::section_0236::cur_fam;
use crate::section_0548::internal_font_number;
use crate::section_0548::internal_font_number_repr;
use crate::section_0681::assign_fam_and_character;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
