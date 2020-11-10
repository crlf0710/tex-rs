//! @ When a |def| command has been scanned,
//! |cur_chr| is odd if the definition is supposed to be global, and
//! |cur_chr>=2| if the definition is supposed to be expanded.

// @<Assignments@>=
macro_rules! Assignments_1218 {
    ($globals:expr, $cur_cmd:expr, $a:expr) => {{
        // def: begin if odd(cur_chr)and not global and(global_defs>=0) then a:=a+4;
        if $cur_cmd == def {
            if $globals.cur_chr.get().is_odd() && !global!($a) && global_defs!($globals) >= 0 {
                $a = $a + 4;
            }
            // e:=(cur_chr>=2); get_r_token; p:=cur_cs;
            let e = $globals.cur_chr.get() >= 2;
            get_r_token($globals)?;
            let p = $globals.cur_cs;
            // q:=scan_toks(true,e); define(p,call+(a mod 4),def_ref);
            let q = scan_toks($globals, true, e);
            define!($globals, $a, p, call + ($a % 4) as quarterword, $globals.def_ref);
            // end;
            use crate::section_0113::quarterword;
            use crate::section_1215::get_r_token;
            use crate::section_0473::scan_toks;
            true
        } else {
            false
        }
    }}
}
