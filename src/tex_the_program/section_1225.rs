//! ` `

// @<Assignments@>=
pub(crate) macro Assignments_1225($globals:expr, $cur_cmd:expr, $a:expr) {{
    // read_to_cs: begin scan_int; n:=cur_val;
    let processed = if $cur_cmd == read_to_cs {
        /// for temporary short-term use
        let p: pointer;
        /// for temporary short-term use
        let n: integer;

        scan_int($globals)?;
        n = $globals.cur_val;
        // if not scan_keyword("to") then
        if !scan_keyword($globals, crate::strpool_str!("to"))? {
            todo!("error");
            // @.to@>
            //   begin print_err("Missing `to' inserted");
            // @.Missing `to'...@>
            //   help2("You should have said `\read<number> to \cs'.")@/
            //   ("I'm going to look for the \cs now."); error;
            //   end;
        }
        // get_r_token;
        get_r_token($globals)?;
        // p:=cur_cs; read_toks(n,p); define(p,call,cur_val);
        p = $globals.cur_cs;
        read_toks($globals, n, p)?;
        define!($globals, $a, p, call, $globals.cur_val as _);
        // end;
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0115::pointer;
    use crate::section_0209::*;
    use crate::section_0210::*;
    use crate::section_0407::scan_keyword;
    use crate::section_0440::scan_int;
    use crate::section_0482::read_toks;
    use crate::section_1214::define;
    use crate::section_1215::get_r_token;
    processed
}}
