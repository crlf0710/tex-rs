//! @ The simplest math formula is, of course, `\.{\${ }\$}', when no noads are
//! generated. The next simplest cases involve a single character, e.g.,
//! `\.{\$x\$}'. Even though such cases may not seem to be very interesting,
//! the reader can perhaps understand how happy the author was when `\.{\$x\$}'
//! was first properly typeset by \TeX. The code in this section was used.
//! @^Knuth, Donald Ervin@>
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1154($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+letter,mmode+other_char,mmode+char_given:
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + letter as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + other_char as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + char_given as u16
    {
        // set_math_char(ho(math_code(cur_chr)));
        set_math_char(
            $globals,
            ho!(math_code!($globals, ASCII_code::from($globals.cur_chr))) as _,
        )?;
        true
    }
    // mmode+char_num: begin scan_char_num; cur_chr:=cur_val;
    else if $abs_mode_plus_cur_cmd == mmode as u16 + char_num as u16 {
        // set_math_char(ho(math_code(cur_chr)));
        // end;
        todo!("mmode + char_num");
        true
    }
    // mmode+math_char_num: begin scan_fifteen_bit_int; set_math_char(cur_val);
    else if $abs_mode_plus_cur_cmd == mmode as u16 + math_char_num as u16 {
        // end;
        todo!("mmode + math_char_num");
        true
    }
    // mmode+math_given: set_math_char(cur_chr);
    else if $abs_mode_plus_cur_cmd == mmode as u16 + math_given as u16 {
        set_math_char($globals, $globals.cur_chr.get() as _)?;
        true
    }
    // mmode+delim_num: begin scan_twenty_seven_bit_int;
    else if $abs_mode_plus_cur_cmd == mmode as u16 + math_given as u16 {
        // set_math_char(cur_val div @'10000);
        // end;
        todo!("mmode + delim_num");
        true
    } else {
        false
    };
    use crate::section_0018::ASCII_code;
    use crate::section_0112::ho;
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_0230::math_code;
    use crate::section_1155::set_math_char;
    processed
}}
