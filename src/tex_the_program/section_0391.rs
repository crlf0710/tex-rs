//! @ At this point, the reader will find it advisable to review the explanation
//! of token list format that was presented earlier, since many aspects of that
//! format are of importance chiefly in the |macro_call| routine.
//!
//! The token list might begin with a string of compulsory tokens before the
//! first |match| or |end_match|. In that case the macro name is supposed to be
//! followed by those tokens; the following program will set |s=null| to
//! represent this restriction. Otherwise |s| will be set to the first token of
//! a string that will delimit the next parameter.
//
// @<Scan the parameters and make |link(r)| point to the macro body...@>=

macro_rules! Scan_the_parameters_and_make_link_r_point_to_the_macro_body__but_return_if_an_illegal_par_is_detected {
    ($globals:expr, $match_chr:expr, $r:expr, $info_r:expr, $m:expr, $n:expr, $p:expr, $q:expr) => {{
        trace_span!("Scan the parameters and make `link(r)` point to the macro body...");
        /// unmatched left braces in current parameter
        let mut unbalance: halfword;
        /// backup pointer for parameter matching
        let mut s:pointer;

        // begin scanner_status:=matching; unbalance:=0;
        $globals.scanner_status = scanner_status_kind::matching;
        unbalance = 0;
        // long_state:=eq_type(cur_cs);
        $globals.long_state = eq_type!($globals, $globals.cur_cs).into();
        // if long_state>=outer_call then long_state:=long_state-2;
        if $globals.long_state >= outer_call {
            $globals.long_state -= 2;
        }
        // repeat link(temp_head):=null;
        loop {
            link!($globals, temp_head) = null;
            // if (info(r)>match_token+255)or(info(r)<match_token) then s:=null
            if $info_r > match_token + 255 || $info_r < match_token {
                s = null;
            }
            // else  begin match_chr:=info(r)-match_token; s:=link(r); r:=s;
            else {
                $match_chr = ASCII_code::from(($info_r.get() - match_token) as integer);
                s = link!($globals, $r);
                $r = s;
                $info_r = info_tok!($globals, $r);
                // p:=temp_head; m:=0;
                $p = temp_head;
                $m = 0;
                // end;
            }
            // @<Scan a parameter until its delimiter string has been found; or, if |s=null|,
            //   simply scan the delimiter string@>;@/
            Scan_a_parameter_until_its_delimiter_string_has_been_found_or_if_s_null_simply_scan_the_delimiter_string!
                ($globals, $match_chr, $r, $info_r, s, $m, $n, $p, $q, unbalance);
            // {now |info(r)| is a token whose command code is either |match| or |end_match|}
            // until info(r)=end_match_token;
            /// now `info(r)` is a token whose command code is either |match| or |end_match|
            if $info_r == end_match_token {
                break;
            }
        }
        // end
        use crate::pascal::integer;
        use crate::section_0210::outer_call;
        use crate::section_0115::null;
        use crate::section_0162::temp_head;
        use crate::section_0289::match_token;
    }}
}