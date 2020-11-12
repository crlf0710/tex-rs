//! @ We temporarily define |p| to be |relax|, so that an occurrence of |p|
//! while scanning the definition will simply stop the scanning instead of
//! producing an ``undefined control sequence'' error or expanding the
//! previous meaning.  This allows, for instance, `\.{\\chardef\\foo=123\\foo}'.
//!
// @<Assignments@>=
macro_rules! Assignments_1224 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // shorthand_def: begin n:=cur_chr; get_r_token; p:=cur_cs; define(p,relax,256);
        if $cur_cmd == shorthand_def {
            let n = $globals.cur_chr.get();
            get_r_token($globals)?;
            let p = $globals.cur_cs;
            define!($globals, $a, p as _, relax, 256);
            // scan_optional_equals;
            scan_optional_equals($globals)?;
            // case n of
            // char_def_code: begin scan_char_num; define(p,char_given,cur_val);
            if n == char_def_code as _ {
                scan_char_num($globals)?;
                define!($globals, $a, p as _, char_given, $globals.cur_val as _);
                // end;
            }
            // math_char_def_code: begin scan_fifteen_bit_int; define(p,math_given,cur_val);
            else if n == math_char_def_code as _ {
                todo!();
                // end;
            }
            // othercases begin scan_eight_bit_int;
            else {
                todo!();
                //   case n of
                //   count_def_code: define(p,assign_int,count_base+cur_val);
                //   dimen_def_code: define(p,assign_dimen,scaled_base+cur_val);
                //   skip_def_code: define(p,assign_glue,skip_base+cur_val);
                //   mu_skip_def_code: define(p,assign_mu_glue,mu_skip_base+cur_val);
                //   toks_def_code: define(p,assign_toks,toks_base+cur_val);
                //   end; {there are no other cases}
                //   end
                // endcases;
            }
            // end;
            use crate::section_0207::relax;
            use crate::section_0208::char_given;
            use crate::section_0405::scan_optional_equals;
            use crate::section_0434::scan_char_num;
            use crate::section_1215::get_r_token;
            use crate::section_1222::char_def_code;
            use crate::section_1222::math_char_def_code;
            true
        } else {
            false
        }
    }}
}