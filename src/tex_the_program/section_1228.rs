//! @ Similar routines are used to assign values to the numeric parameters.
//
// @<Assignments@>=
pub(crate) macro Assignments_1228($globals:expr, $cur_cmd:expr, $a:expr) {{
    let processed = if $cur_cmd == assign_int {
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
        scan_normal_dimen!($globals)?;
        word_define!($globals, $a, p as _, $globals.cur_val);
        //   end;
        true
    } else if $cur_cmd == assign_glue || $cur_cmd == assign_mu_glue {
        // assign_glue,assign_mu_glue: begin p:=cur_chr; n:=cur_cmd; scan_optional_equals;
        let p = $globals.cur_chr.get();
        let n = $globals.cur_cmd;
        scan_optional_equals($globals)?;
        // if n=assign_mu_glue then scan_glue(mu_val)@+else scan_glue(glue_val);
        if n == assign_mu_glue {
            scan_glue($globals, small_number::new(cur_val_level_kind::mu_val as _))?;
        } else {
            scan_glue(
                $globals,
                small_number::new(cur_val_level_kind::glue_val as _),
            )?;
        }
        // trap_zero_glue;
        trap_zero_glue($globals);
        // define(p,glue_ref,cur_val);
        define!($globals, $a, p as _, glue_ref, $globals.cur_val as _);
        // end;
        true
    } else {
        false
    };
    use crate::section_0101::small_number;
    use crate::section_0209::*;
    use crate::section_0210::*;
    use crate::section_0405::scan_optional_equals;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0440::scan_int;
    use crate::section_0448::scan_normal_dimen;
    use crate::section_0461::scan_glue;
    use crate::section_1214::define;
    use crate::section_1214::word_define;
    use crate::section_1229::trap_zero_glue;
    processed
}}
