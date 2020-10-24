// @ @<Scan ahead in the buffer...@>=
macro_rules! Scan_ahead_in_the_buffer_until_finding_a_nonletter__if_an_expanded_code_is_encountered_reduce_it_and_goto_start_cs__otherwise_if_a_multiletter_control_sequence_is_found_adjust_cur_cs_and_loc_and_goto_found {
    ($globals:expr, $k:expr, $cat:expr, $lbl_start_cs:lifetime, $lbl_found:lifetime) => {
        // begin repeat cur_chr:=buffer[k]; cat:=cat_code(cur_chr); incr(k);
        loop {
            $globals.cur_chr = $globals.buffer[$k].into();
            $cat = cat_code!($globals, ASCII_code::from($globals.cur_chr)) as _;
            incr!($k);
            // until (cat<>letter)or(k>limit);
            if $cat != letter || $k > limit!($globals) {
                break;
            }
        }
        // @<If an expanded...@>;
        If_an_expanded_code_is_present_reduce_it_and_goto_start_cs!($globals);
        // if cat<>letter then decr(k);
        if $cat != letter {
            decr!($k);
        }
        // {now |k| points to first nonletter}
        /// now `k` points to first nonletter
        const _ : () = ();
        // if k>loc+1 then {multiletter control sequence has been scanned}
        /// multiletter control sequence has been scanned
        if $k > loc!($globals) + 1 {
            // begin cur_cs:=id_lookup(loc,k-loc); loc:=k; goto found;
            $globals.cur_cs = id_lookup($globals, loc!($globals) as _, ($k - loc!($globals)) as _);
            loc!($globals) = $k;
            goto_forward_label!($lbl_found);
            // end;
        }
        // end
        use crate::section_0259::id_lookup;
    }
}
