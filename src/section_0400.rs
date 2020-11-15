//! @ If the parameter consists of a single group enclosed in braces, we must
//! strip off the enclosing braces. That's why |rbrace_ptr| was introduced.
//
// @<Tidy up the parameter just scanned, and tuck it away@>=
macro_rules! Tidy_up_the_parameter_just_scanned__and_tuck_it_away {
    ($globals:expr, $match_chr:expr, $m:expr, $n:expr, $p:expr, $q:expr) => {{
        // begin if (m=1)and(info(p)<right_brace_limit)and(p<>temp_head) then
        if $m == 1 && info_tok!($globals, $p) < right_brace_limit && $p != temp_head {
            // begin link(rbrace_ptr):=null; free_avail(p);
            // p:=link(temp_head); pstack[n]:=link(p); free_avail(p);
            // end
            todo!("tidy1")
        }
        // else pstack[n]:=link(temp_head);
        else {
            $globals.pstack[$n.get()] = link!($globals, temp_head);
        }
        // incr(n);
        incr!($n);
        // if tracing_macros>0 then
        if tracing_macros!($globals) > 0 {
            // begin begin_diagnostic; print_nl(match_chr); print_int(n);
            begin_diagnostic($globals);
            print_nl($globals, str_number::new($match_chr.numeric_value() as _));
            print_int($globals, $n.get() as _);
            // print("<-"); show_token_list(pstack[n-1],null,1000);
            print($globals, strpool_str!("<-").get() as _);
            show_token_list($globals, $globals.pstack[$n.get()-1] as _, null as _, 1000);
            // end_diagnostic(false);
            end_diagnostic($globals, false);
            // end;
        }
        // end
        use crate::section_0038::str_number;
        use crate::section_0059::print;
        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
        use crate::section_0245::begin_diagnostic;
        use crate::section_0245::end_diagnostic;
        use crate::section_0292::show_token_list;
    }}
}

