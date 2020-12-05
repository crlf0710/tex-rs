//! @ If |info(r)| is a |match| or |end_match| command, it cannot be equal to
//! any token found by |get_token|. Therefore an undelimited parameter---i.e.,
//! a |match| that is immediately followed by |match| or |end_match|---will
//! always fail the test `|cur_tok=info(r)|' in the following algorithm.
//
// @<Scan a parameter until its delimiter string has been found; or, ...@>=
macro_rules! Scan_a_parameter_until_its_delimiter_string_has_been_found_or_if_s_null_simply_scan_the_delimiter_string {
    ($globals:expr, $match_chr:expr, $r:expr, $info_r:expr, $s:expr, $m:expr, $n:expr, $p:expr, $q:expr) => {{
        trace_span!("Scan a parameter until its delimiter string has been found; or, ...");
        region_backward_label! {
            // continue: get_token; {set |cur_tok| to the next token of input}
            'continue_ <-
            {
                /// set `cur_tok` to the next token of input
                get_token($globals)?;
                // if cur_tok=info(r) then
                if $globals.cur_tok == $info_r {
                    // @<Advance \(r)|r|; |goto found| if the parameter delimiter has been
                    //   fully matched, otherwise |goto continue|@>;
                    todo!("advance");
                }
                // @<Contribute the recently matched tokens to the current parameter, and
                //   |goto continue| if a partial match is still in effect;
                //   but abort if |s=null|@>;
                Contribute_the_recently_matched_tokens_to_the_current_parameter__and_goto_continue_if_a_partial_match_is_still_in_effect__but_abort_if_s_null!
                    ($globals, $r, $info_r, $m, $s, $p, $q);
                // if cur_tok=par_token then if long_state<>long_call then
                if $globals.cur_tok == $globals.par_token && $globals.long_state != long_call {
                    // @<Report a runaway argument and abort@>;
                    todo!("report");
                }
                // if cur_tok<right_brace_limit then
                if $globals.cur_tok < right_brace_limit {
                    // if cur_tok<left_brace_limit then
                    if $globals.cur_tok < left_brace_limit {
                        // @<Contribute an entire group to the current parameter@>
                        todo!("contribute group");
                    }
                    // else @<Report an extra right brace and |goto continue|@>
                    else {
                        todo!("extra brace");
                    }
                }
                // else @<Store the current token, but |goto continue| if it is
                //    a blank space that would become an undelimited parameter@>;
                else {
                    Store_the_current_token__but_goto_continue_if_it_is_a_blank_space_that_would_become_an_undelimited_parameter!
                        ($globals, $info_r, $p, $q, 'continue_);
                }
                // incr(m);
                incr!($m);
                // if info(r)>end_match_token then goto continue;
                if $info_r.get() > end_match_token {
                    goto_backward_label!('continue_);
                }
                // if info(r)<match_token then goto continue;
                if $info_r.get() < match_token {
                    goto_backward_label!('continue_);
                }
                // found: if s<>null then @<Tidy up the parameter just scanned, and tuck it away@>
                if $s != null {
                    Tidy_up_the_parameter_just_scanned__and_tuck_it_away!
                        ($globals, $match_chr, $m, $n, $p, $q);
                }
            }
            |'continue_|
        }
        use crate::section_0210::long_call;
        use crate::section_0289::left_brace_limit;
        use crate::section_0289::right_brace_limit;
        use crate::section_0365::get_token;
    }}
}
