//! ` `

use crate::tex_the_program::section_0681::assign_fam_and_character;
// @<Declare act...@>=
// procedure math_ac;
pub(crate) fn math_ac(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if cur_cmd=accent then
    if globals.cur_cmd == accent {
        // @<Complain that the user should have said \.{\\mathaccent}@>;
        todo!("complain");
    }
    // tail_append(get_node(accent_noad_size));
    tail_append!(globals, get_node(globals, accent_noad_size as _)?);
    // type(tail):=accent_noad; subtype(tail):=normal;
    r#type!(globals, tail!(globals)) = accent_noad;
    subtype!(globals, tail!(globals)) = noad_subtype::normal as _;
    // mem[nucleus(tail)].hh:=empty_field;
    globals.mem[nucleus!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // mem[subscr(tail)].hh:=empty_field;
    globals.mem[subscr!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // mem[supscr(tail)].hh:=empty_field;
    globals.mem[supscr!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // math_type(accent_chr(tail)):=math_char;
    math_type!(globals, accent_chr!(tail!(globals))) = math_type_kind::math_char as _;
    // scan_fifteen_bit_int;
    scan_fifteen_bit_int(globals)?;
    // character(accent_chr(tail)):=qi(cur_val mod 256);
    let accent_chr_character = ASCII_code::from(qi!((globals.cur_val % 256) as u8) as integer);
    // if (cur_val>=var_code)and fam_in_range then fam(accent_chr(tail)):=cur_fam
    let accent_chr_fam = if globals.cur_val >= var_code && fam_in_range!(globals) {
        cur_fam!(globals)
    }
    // else fam(accent_chr(tail)):=(cur_val div 256) mod 16;
    else {
        (globals.cur_val / 256) % 16
    };
    assign_fam_and_character!(
        globals,
        accent_chr!(tail!(globals)),
        internal_font_number::new(accent_chr_fam as _),
        accent_chr_character
    );
    // scan_math(nucleus(tail));
    scan_math(globals, nucleus!(tail!(globals)))?;
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0112::qi;
use crate::section_0113::MEMORY_WORD_HH;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0208::accent;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0232::var_code;
use crate::section_0236::cur_fam;
use crate::section_0436::scan_fifteen_bit_int;
use crate::section_0548::internal_font_number;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0681::subscr;
use crate::section_0681::supscr;
use crate::section_0684::empty_field;
use crate::section_0686::noad_subtype;
use crate::section_0687::accent_chr;
use crate::section_0687::accent_noad;
use crate::section_0687::accent_noad_size;
use crate::section_1151::fam_in_range;
use crate::section_1151::scan_math;
