//! @ The different types of code values have different legal ranges; the
//! following program is careful to check each case properly.
//
// @<Assignments@>=
macro_rules! Assignments_1232 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {
        // def_code: begin @<Let |n| be the largest legal code value, based on |cur_chr|@>;
        if $cur_cmd == def_code {
            let n: integer;
            let mut p: integer;
            Let_n_be_the_largest_legal_code_value__based_on_cur_chr!($globals, n, $globals.cur_chr);
            //   p:=cur_chr; scan_char_num; p:=p+cur_val; scan_optional_equals;
            p = $globals.cur_chr.get() as _;
            scan_char_num($globals)?;
            p += $globals.cur_val;
            scan_optional_equals($globals)?;
            // scan_int;
            scan_int($globals)?;
            //   if ((cur_val<0)and(p<del_code_base))or(cur_val>n) then
            if ($globals.cur_val < 0 && p < del_code_base as _) || ($globals.cur_val > n as _) {
                //     begin print_err("Invalid code ("); print_int(cur_val);
                // @.Invalid code@>
                //     if p<del_code_base then print("), should be in the range 0..")
                //     else print("), should be at most ");
                //     print_int(n);
                //     help1("I'm going to use 0 instead of that illegal code value.");@/
                //     error; cur_val:=0;
                //     end;
            }
            // if p<math_code_base then define(p,data,cur_val)
            if p < math_code_base as _ {
                define!($globals, $a, p as _, data, $globals.cur_val as _);
            }
            // else if p<del_code_base then define(p,data,hi(cur_val))
            else if p < del_code_base as _ {
                define!($globals, $a, p as _, data, hi!($globals.cur_val as halfword))
            }
            // else word_define(p,cur_val);
            else {
                word_define!($globals, $a, p as _, $globals.cur_val);
            }
            // end;
            use crate::section_0113::halfword;
            use crate::section_0230::{cat_code_base, math_code_base, sf_code_base};
            use crate::section_0236::del_code_base;
            use crate::section_0405::scan_optional_equals;
            use crate::section_0434::scan_char_num;
            use crate::section_0440::scan_int;
            true
        } else {
            false
        }
    }
}