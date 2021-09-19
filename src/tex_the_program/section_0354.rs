//! @ Control sequence names are scanned only when they appear in some line of
//! a file; once they have been scanned the first time, their |eqtb| location
//! serves as a unique identification, so \TeX\ doesn't need to refer to the
//! original name any more except when it prints the equivalent in symbolic form.
//!
//! The program that scans a control sequence has been written carefully
//! in order to avoid the blowups that might otherwise occur if a malicious
//! user tried something like `\.{\\catcode\'15=0}'. The algorithm might
//! look at |buffer[limit+1]|, but it never looks at |buffer[limit+2]|.
//!
//! If expanded characters like `\.{\^\^A}' or `\.{\^\^df}'
//! appear in or just following
//! a control sequence name, they are converted to single characters in the
//! buffer and the process is repeated, slowly but surely.

// @<Scan a control...@>=
pub(crate) macro Scan_a_control_sequence_and_set_state_skip_blanks_or_mid_line {
    ($globals:expr) => {{
        crate::trace_span_verbose!("Scan a control...");
        crate::region_forward_label! {
        |'found|
        {
        crate::trace_expr_verbose!("cs loc, limit = ({:?}, {:?})", loc!($globals), limit!($globals));
        // begin if loc>limit then cur_cs:=null_cs {|state| is irrelevant in this case}
        if loc!($globals) > limit!($globals) {
            /// `state` is irrelevant in this case
            const _ : () = ();
            $globals.cur_cs = null_cs;
        } else {
            crate::region_backward_label! {
            'start_cs <-
            {
                // else  begin start_cs: k:=loc; cur_chr:=buffer[k]; cat:=cat_code(cur_chr);
                let mut k = loc!($globals);
                $globals.cur_chr = $globals.buffer[k].into();
                crate::trace_expr_verbose!("cs cur_chr = {:?}", $globals.cur_chr);
                let mut cat = cat_code!($globals, $globals.buffer[k]) as quarterword;
                // incr(k);
                incr!(k);
                // if cat=letter then state:=skip_blanks
                if cat == letter {
                    state!($globals) = skip_blanks;
                } else if cat == spacer {
                    // else if cat=spacer then state:=skip_blanks
                    state!($globals) = skip_blanks;
                } else {
                    // else state:=mid_line;
                    state!($globals) = mid_line;
                }
                // if (cat=letter)and(k<=limit) then
                if cat==letter && k <= limit!($globals) {
                    // @<Scan ahead in the buffer until finding a nonletter;
                    // if an expanded code is encountered, reduce it
                    // and |goto start_cs|; otherwise if a multiletter control
                    // sequence is found, adjust |cur_cs| and |loc|, and
                    // |goto found|@>
                    crate::section_0356::Scan_ahead_in_the_buffer_until_finding_a_nonletter__if_an_expanded_code_is_encountered_reduce_it_and_goto_start_cs__otherwise_if_a_multiletter_control_sequence_is_found_adjust_cur_cs_and_loc_and_goto_found!
                        ($globals, k, cat, 'start_cs, 'found);
                } else {
                    // else @<If an expanded code is present, reduce it and |goto start_cs|@>;
                    crate::section_0355::If_an_expanded_code_is_present_reduce_it_and_goto_start_cs!
                        ($globals, k, cat, 'start_cs);
                }
                // cur_cs:=single_base+buffer[loc]; incr(loc);
                #[cfg(feature = "unicode_support")]
                if $globals.buffer[loc!($globals)] > ASCII_code::from(255) {
                    todo!("buffer item > 255, is {}", $globals.buffer[loc!($globals)].0);
                }
                $globals.cur_cs = single_base as halfword +
                    $globals.buffer[loc!($globals)].numeric_value() as halfword;
                incr!(loc!($globals));
            }
            |'start_cs|
            }
            // end;
        }
        }
        // found: cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
        'found <-
        }
        crate::trace_expr_verbose!("cur_cs = {}", $globals.cur_cs);
        $globals.cur_cmd = eq_type!($globals, $globals.cur_cs as u32);
        crate::trace_expr_verbose!("cur_cmd = {}", $globals.cur_cmd);
        $globals.cur_chr = chr_code_type::new(equiv!($globals, $globals.cur_cs as u32) as _);
        crate::trace_expr_verbose!("cur_chr = {:?}", $globals.cur_chr);
        // if cur_cmd>=outer_call then check_outer_validity;
        if $globals.cur_cmd >= outer_call {
            check_outer_validity($globals);
            // end
        }
        use crate::section_0016::incr;
        use crate::section_0036::loc;
        use crate::section_0113::halfword;
        use crate::section_0113::quarterword;
        use crate::section_0221::equiv;
        use crate::section_0222::null_cs;
        use crate::section_0222::single_base;
        use crate::section_0297::chr_code_type;
        use crate::section_0018::ASCII_code;
        use crate::section_0210::outer_call;
        use crate::section_0221::eq_type;
        use crate::section_0230::cat_code;
        use crate::section_0302::limit;
        use crate::section_0336::check_outer_validity;
        use crate::section_0207::spacer;
        use crate::section_0207::letter;
        use crate::section_0302::state;
        use crate::section_0303::skip_blanks;
        use crate::section_0303::mid_line;
        use crate::pascal::integer;
    }}
}
