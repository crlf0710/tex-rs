//! @ All of \TeX's parameters are kept in |eqtb| except the font information,
//! the interaction mode, and the hyphenation tables; these are strictly global.
//
// @<Assignments@>=
macro_rules! Assignments_1253 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // assign_font_dimen: begin find_font_dimen(true); k:=cur_val;
        if $cur_cmd == assign_font_dimen {
            /// index into `font_info`
            let k: font_index;
            find_font_dimen($globals, true)?;
            k = font_index::new($globals.cur_val as _);
            // scan_optional_equals; scan_normal_dimen; font_info[k].sc:=cur_val;
            scan_optional_equals($globals)?;
            scan_normal_dimen!($globals)?;
            $globals.font_info[k][MEMORY_WORD_SC] = scaled::new_from_inner($globals.cur_val);
            // end;
            use crate::section_0101::scaled;
            use crate::section_0101::MEMORY_WORD_SC;
            use crate::section_0548::font_index;
            use crate::section_0578::find_font_dimen;
            true
        }
        // assign_font_int: begin n:=cur_chr; scan_font_ident; f:=cur_val;
        else if $cur_cmd == assign_font_int {
            /// for temporary short-term use
            let n: integer;
            /// identifies a font
            let f: internal_font_number;

            n = $globals.cur_chr.get() as _;
            scan_font_ident($globals)?;
            f = internal_font_number::new($globals.cur_val as _);
            // scan_optional_equals; scan_int;
            scan_optional_equals($globals)?;
            scan_int($globals)?;
            // if n=0 then hyphen_char[f]:=cur_val@+else skew_char[f]:=cur_val;
            if n == 0 {
                $globals.hyphen_char[f] = $globals.cur_val;
            } else {
                $globals.skew_char[f] = $globals.cur_val;
            }
            // end;
            use crate::section_0548::internal_font_number;
            use crate::section_0577::scan_font_ident;
            true
        } else {
            false
        }
    }}
}
