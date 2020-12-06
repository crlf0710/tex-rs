//! @ Similar routines are used to assign values to the numeric parameters.
//
// @<Assignments@>=
macro_rules! Assignments_1228 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        if $cur_cmd == assign_int {
            // assign_int: begin p:=cur_chr; scan_optional_equals; scan_int;
            let p = $globals.cur_chr.get();
            scan_optional_equals($globals)?;
            scan_int($globals)?;
            // word_define(p,cur_val);
            word_define!($globals, $a, p as _, $globals.cur_val);
            //   end;
            true
        } else if $cur_cmd == assign_dimen {
            // assign_dimen: begin p:=cur_chr; scan_optional_equals;
            let p = $globals.cur_chr.get();
            scan_optional_equals($globals)?;
            //   scan_normal_dimen; word_define(p,cur_val);
            scan_normal_dimen!($globals);
            word_define!($globals, $a, p as _, $globals.cur_val);
            //   end;
            true
        } else if $cur_cmd == assign_glue || $cur_cmd == assign_mu_glue {
            // assign_glue,assign_mu_glue: begin p:=cur_chr; n:=cur_cmd; scan_optional_equals;
            //   if n=assign_mu_glue then scan_glue(mu_val)@+else scan_glue(glue_val);
            //   trap_zero_glue;
            //   define(p,glue_ref,cur_val);
            //   end;
            todo!();
            true
        } else {
            false
        }
    }}
}
