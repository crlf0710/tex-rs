//! @ The |set_math_char| procedure creates a new noad appropriate to a given
//! math code, and appends it to the current mlist. However, if the math code
//! is sufficiently large, the |cur_chr| is treated as an active character and
//! nothing is appended.
//
// @<Declare act...@>=
// procedure set_math_char(@!c:integer);
pub(crate) fn set_math_char(globals: &mut TeXGlobals, c: integer) -> TeXResult<()> {
    // var p:pointer; {the new noad}
    // begin if c>=@'100000 then
    if c >= 0o100000 {
        // @<Treat |cur_chr|...@>
        crate::section_1152::Treat_cur_chr_as_an_active_character!(globals);
    }
    // else  begin p:=new_noad; math_type(nucleus(p)):=math_char;
    else {
        /// the new noad
        let p;
        p = new_noad(globals)?;
        math_type!(globals, nucleus!(p)) = math_type_kind::math_char as _;
        // character(nucleus(p)):=qi(c mod 256);
        // fam(nucleus(p)):=(c div 256) mod 16;
        let ch = qi!(((c as word) % 256) as u8);
        let mut fam = internal_font_number::new(((c as word) / 256 % 16) as _);
        // if c>=var_code then
        if c >= var_code {
            // begin if fam_in_range then fam(nucleus(p)):=cur_fam;
            if fam_in_range!(globals) {
                fam = internal_font_number::new(cur_fam!(globals) as _);
            }
            // type(p):=ord_noad;
            r#type!(globals, p) = ord_noad;
            // end
        }
        // else  type(p):=ord_noad+(c div @'10000);
        else {
            let t = ((c as word) / 0o10000) as quarterword;
            r#type!(globals, p) = ord_noad + t;
        }
        assign_fam_and_character!(globals, nucleus!(p), fam, ASCII_code::from(ch as integer));
        // link(tail):=p; tail:=p;
        link!(globals, tail!(globals)) = p;
        tail!(globals) = p;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0112::qi;
use crate::section_0113::quarterword;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0213::tail;
use crate::section_0232::var_code;
use crate::section_0236::cur_fam;
use crate::section_0548::internal_font_number;
use crate::section_0681::assign_fam_and_character;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0682::ord_noad;
use crate::section_0686::new_noad;
use crate::section_1151::fam_in_range;
