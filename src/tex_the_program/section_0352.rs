//! @ Notice that a code like \.{\^\^8} becomes \.x if not followed by a hex digit.
//!
// @d is_hex(#)==(((#>="0")and(#<="9"))or((#>="a")and(#<="f")))
pub(crate) macro is_hex($val:expr) {
    ($val >= b'0' as _ && $val <= b'9' as _) || ($val >= b'a' as _ && $val <= b'f' as _)
}

// @d hex_to_cur_chr==
pub(crate) macro hex_to_cur_chr($globals:expr, $c:expr, $cc:expr) {{
    // if c<="9" then cur_chr:=c-"0" @+else cur_chr:=c-"a"+10;
    if $c <= b'9' as _ {
        $globals.cur_chr = chr_code_type::new($c - b'0' as chr_code_repr);
    } else {
        $globals.cur_chr = chr_code_type::new($c - b'a' as chr_code_repr + 10);
    }
    // if cc<="9" then cur_chr:=16*cur_chr+cc-"0"
    if $cc <= b'9' as _ {
        $globals.cur_chr =
            chr_code_type::new(16 * $globals.cur_chr.get() + $cc - b'0' as chr_code_repr)
    }
    // else cur_chr:=16*cur_chr+cc-"a"+10
    else {
        $globals.cur_chr =
            chr_code_type::new(16 * $globals.cur_chr.get() + $cc - b'a' as chr_code_repr + 10)
    }
    use crate::section_0297::chr_code_repr;
    use crate::section_0297::chr_code_type;
}}

// @<If this |sup_mark| starts an expanded character...@>=
pub(crate) macro If_this_sup_mark_starts_an_expanded_character_like___A__or__df__then_goto_reswitch__otherwise_set_state__mid_line($globals:expr, $lbl_reswitch:lifetime) {{
    crate::trace_span!("If this `sup_mark` starts an expanded character...");
    // begin if cur_chr=buffer[loc] then if loc<limit then
    crate::trace_expr_verbose!(
        "buffer[loc] = {}",
        $globals.buffer[loc!($globals)].numeric_value()
    );
    if $globals.cur_chr.get() == $globals.buffer[loc!($globals)].numeric_value() as chr_code_repr {
        if loc!($globals) < limit!($globals) {
            // begin c:=buffer[loc+1]; @+if c<@'200 then {yes we have an expanded char}
            let c = $globals.buffer[loc!($globals) + 1].numeric_value();
            crate::trace_expr_verbose!("c = {}", c);
            if c < 0o200 {
                /// yes we have an expanded char
                const _: () = ();
                // begin loc:=loc+2;
                loc!($globals) = loc!($globals) + 2;
                // if is_hex(c) then if loc<=limit then
                if is_hex!(c) {
                    if loc!($globals) <= limit!($globals) {
                        // begin cc:=buffer[loc]; @+if is_hex(cc) then
                        let cc = $globals.buffer[loc!($globals)].numeric_value();
                        crate::trace_expr_verbose!("cc = {}", cc);
                        if is_hex!(cc) {
                            // begin incr(loc); hex_to_cur_chr; goto reswitch;
                            incr!(loc!($globals));
                            hex_to_cur_chr!($globals, c, cc);
                            crate::goto_backward_label!($lbl_reswitch);
                            // end;
                        }
                        // end;
                    }
                }
                // if c<@'100 then cur_chr:=c+@'100 @+else cur_chr:=c-@'100;
                if c < 0o100 {
                    $globals.cur_chr = chr_code_type::new(c + 0o100);
                } else {
                    $globals.cur_chr = chr_code_type::new(c - 0o100);
                }
                // goto reswitch;
                crate::goto_backward_label!($lbl_reswitch);
                // end;
            }
            // end;
        }
        // state:=mid_line;
        state!($globals) = mid_line;
        // end
    }
    use crate::section_0016::incr;
    use crate::section_0036::loc;
    use crate::section_0297::chr_code_repr;
    use crate::section_0297::chr_code_type;
    use crate::section_0302::limit;
    use crate::section_0302::state;
    use crate::section_0303::*;
}}
