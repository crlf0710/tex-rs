//! @ Whenever we reach the following piece of code, we will have
//! |cur_chr=buffer[k-1]| and |k<=limit+1| and |cat=cat_code(cur_chr)|. If an
//! expanded code like \.{\^\^A} or \.{\^\^df} appears in |buffer[(k-1)..(k+1)]|
//! or |buffer[(k-1)..(k+2)]|, we
//! will store the corresponding code in |buffer[k-1]| and shift the rest of
//! the buffer left two or three places.
//
// @<If an expanded...@>=
pub(crate) macro If_an_expanded_code_is_present_reduce_it_and_goto_start_cs($globals:expr, $k:expr, $cat:expr, $lbl_start_cs:lifetime) {{
    crate::trace_span_verbose!("If an expanded...");
    // begin if buffer[k]=cur_chr then @+if cat=sup_mark then @+if k<limit then
    if $globals.buffer[$k].numeric_value() == $globals.cur_chr.get()
        && $cat == sup_mark
        && $k < limit!($globals)
    {
        // begin c:=buffer[k+1]; @+if c<@'200 then {yes, one is indeed present}
        let c = $globals.buffer[$k + 1].numeric_value();
        if c < 0o200 {
            /// yes, one is indeed present
            const _: () = ();
            // begin d:=2;
            let mut d = 2;
            let mut cc = 0;
            // if is_hex(c) then @+if k+2<=limit then
            if is_hex!(c) && $k + 2 <= limit!($globals) {
                // begin cc:=buffer[k+2]; @+if is_hex(cc) then incr(d);
                cc = $globals.buffer[$k + 2].numeric_value();
                if is_hex!(cc) {
                    incr!(d);
                }
                // end;
            }
            // if d>2 then
            if d > 2 {
                // begin hex_to_cur_chr; buffer[k-1]:=cur_chr;
                hex_to_cur_chr!($globals, c, cc);
                $globals.buffer[$k - 1] = $globals.cur_chr.into();
                // end
            }
            // else if c<@'100 then buffer[k-1]:=c+@'100
            else if c < 0o100 {
                $globals.buffer[$k - 1] = ASCII_code::from(c as integer + 0o100);
            }
            // else buffer[k-1]:=c-@'100;
            else {
                $globals.buffer[$k - 1] = ASCII_code::from(c as integer - 0o100);
            }
            crate::trace_expr!(
                "buffer[k - 1] = {}",
                $globals.buffer[$k - 1].numeric_value()
            );
            crate::trace_expr!("d={}", d);
            // limit:=limit-d; first:=first-d;
            limit!($globals) -= d;
            $globals.first -= d;
            // while k<=limit do
            while $k <= limit!($globals) {
                // begin buffer[k]:=buffer[k+d]; incr(k);
                $globals.buffer[$k] = $globals.buffer[$k + d];
                incr!($k);
                // end;
            }
            // goto start_cs;
            crate::goto_backward_label!($lbl_start_cs);
            // end;
        }
        // end;
    }
    // end
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0207::*;
    use crate::section_0302::limit;
    use crate::section_0352::hex_to_cur_chr;
    use crate::section_0352::is_hex;
}}
