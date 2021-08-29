//! @ The simplest math formula is, of course, `\.{\${ }\$}', when no noads are
//! generated. The next simplest cases involve a single character, e.g.,
//! `\.{\$x\$}'. Even though such cases may not seem to be very interesting,
//! the reader can perhaps understand how happy the author was when `\.{\$x\$}'
//! was first properly typeset by \TeX. The code in this section was used.
//! @^Knuth, Donald Ervin@>
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1154 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // mmode+letter,mmode+other_char,mmode+char_given:
        if $abs_mode_plus_cur_cmd == mmode as u16 + letter as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + other_char as u16 ||
            $abs_mode_plus_cur_cmd == mmode as u16 + char_given as u16 {
            // set_math_char(ho(math_code(cur_chr)));
            set_math_char($globals, math_code!($globals, $globals.cur_chr.into()));

            use crate::section_1155::set_math_char;
            true
        }
        // mmode+char_num: begin scan_char_num; cur_chr:=cur_val;
        else if $abs_mode_plus_cur_cmd == mmode as u16 + char_num as u16 {
            scan_char_num($globals, $globals.allow_big_char_code);
            $globals.cur_chr = $globals.cur_val;
            // set_math_char(ho(math_code(cur_chr)));
            set_math_char($globals, math_code!($globals, $globals.cur_chr.into()));
            // end;
            use crate::section_0434::scan_char_num;
            use crate::section_1155::set_math_char;
            true
        }
        // mmode+math_char_num: begin scan_fifteen_bit_int; set_math_char(cur_val);
        else if $abs_mode_plus_cur_cmd == mmode as u16 + math_char_num as u16 {
            scan_fifteen_bit_int($globals);
            set_math_char($globals, $globals.cur_val);
            // end;
            use crate::section_0436::scan_fifteen_bit_int;
            use crate::section_1155::set_math_char;
            true
        }
        // mmode+math_given: set_math_char(cur_chr);
        else if $abs_mode_plus_cur_cmd == mmode as u16 + math_given as u16 {
            set_math_char($globals, $globals.cur_chr.into());

            use crate::section_1155::set_math_char;
            true
        }
        // mmode+delim_num: begin scan_twenty_seven_bit_int;
        else if $abs_mode_plus_cur_cmd == mmode as u16 + delim_num as u16 {
            scan_twenty_seven_bit_int($globals);
            // set_math_char(cur_val div @'10000);
            set_math_char($globals, $globals.cur_val / 0o10000);
            // end;
            use crate::section_1155::set_math_char;
            true
        }
        else {
            false
        }
    }}
}
