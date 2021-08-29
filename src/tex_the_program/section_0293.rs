//! ` `

// @<Display token |p|...@>=
macro_rules! Display_token_p__and_return_if_there_are_problems {
    ($globals:expr, $p:expr, $n:expr, $match_chr:expr) => {{
        // if (p<hi_mem_min) or (p>mem_end) then
        if ($p as pointer) < $globals.hi_mem_min || ($p as pointer) > $globals.mem_end {
            trace_error_expr!("clobbered p = {}", $p as pointer);
            // begin print_esc("CLOBBERED."); return;
            print_esc($globals, strpool_str!("CLOBBERED."));
            return;
            // @.CLOBBERED@>
            // end;
        }
        let info_p = info_tok!($globals, $p as pointer);
        // if info(p)>=cs_token_flag then print_cs(info(p)-cs_token_flag)
        if let Some(cs) = info_p.get_cs() {
            print_cs($globals, cs as integer);
        }
        // else  begin m:=info(p) div @'400; c:=info(p) mod @'400;
        else {
            let (m, c) = info_p.get_cmd_and_chr().unwrap();
            // if info(p)<0 then print_esc("BAD.")
            #[allow(unused_comparisons)]
            if info_p.get() < 0 {
                print_esc($globals, strpool_str!("BAD."));
            }
            // @.BAD@>
            //   else @<Display the token $(|m|,|c|)$@>;
            else {
                Display_the_token_m_c!($globals, m, c, $n, $match_chr);
            }
        // end
        }

        use crate::section_0018::ASCII_code;
        use crate::section_0063::print_esc;
        use crate::section_0262::print_cs;
        use crate::section_0297::cur_tok_type;
    }}
}

