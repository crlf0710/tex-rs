//! ` `

// @<Assignments@>=
pub(crate) macro Assignments_1234($globals:expr, $cur_cmd:expr, $a:expr) {{
    // def_family: begin p:=cur_chr; scan_four_bit_int; p:=p+cur_val;
    let processed = if $cur_cmd == def_family {
        /// for temporary short-term use
        let mut p: pointer;
        p = $globals.cur_chr.get() as _;
        scan_four_bit_int($globals)?;
        p = p + $globals.cur_val as pointer;
        // scan_optional_equals; scan_font_ident; define(p,data,cur_val);
        scan_optional_equals($globals)?;
        scan_font_ident($globals)?;
        define!($globals, $a, p, data, $globals.cur_val as _);
        // end;
        true
    } else {
        false
    };
    use crate::section_0115::pointer;
    use crate::section_0209::*;
    use crate::section_0210::*;
    use crate::section_0405::scan_optional_equals;
    use crate::section_0435::scan_four_bit_int;
    use crate::section_0577::scan_font_ident;
    use crate::section_1214::define;
    processed
}}
