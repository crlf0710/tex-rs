//! @ When the following code is executed, we have |cur_tok=point_token|, but this
//! token has been backed up using |back_input|; we must first discard it.
//!
//! It turns out that a decimal point all by itself is equivalent to `\.{0.0}'.
//! Let's hope people don't use that fact.
//
// @<Scan decimal fraction@>=
pub(crate) macro Scan_decimal_fraction
    ($globals:expr, $f:expr) {{
        /// number of digits in a decimal fraction
        let mut k: small_number;
        /// top of decimal digit stack
        let (mut p, mut q): (pointer, pointer);
        // begin k:=0; p:=null; get_token; {|point_token| is being re-scanned}
        k = 0.into();
        p = null;
        /// `point_token` is being re-scanned
        get_token($globals)?;
        crate::region_forward_label!(
        |'done1|
        {
            // loop@+  begin get_x_token;
            loop {
                get_x_token($globals)?;
                // if (cur_tok>zero_token+9)or(cur_tok<zero_token) then goto done1;
                if $globals.cur_tok > zero_token + 9 || $globals.cur_tok < zero_token {
                    crate::goto_forward_label!('done1);
                }
                // if k<17 then {digits for |k>=17| cannot affect the result}
                if k < 17 {
                    /// digits for `k>=17` cannot affect the result
                    const _ : () = ();
                    // begin q:=get_avail; link(q):=p; info(q):=cur_tok-zero_token;
                    q = get_avail($globals);
                    link!($globals, q) = p;
                    info_inner!($globals, q) = ($globals.cur_tok.get() - zero_token) as _;
                    // p:=q; incr(k);
                    p = q;
                    incr!(k);
                    // end;
                }
                // end;
            }
        }
        'done1 <-
        );
        // done1: for kk:=k downto 1 do
        for kk in (1..=k.get()).into_iter().rev() {
            // begin dig[kk-1]:=info(p); q:=p; p:=link(p); free_avail(q);
            $globals.dig[(kk - 1) as usize] = (info_inner!($globals, p) as u8).into();
            q = p;
            p = link!($globals, p);
            free_avail!($globals, q);
            // end;
        }
        // f:=round_decimals(k);
        $f = round_decimals($globals, k).inner();
        // if cur_cmd<>spacer then back_input;
        if $globals.cur_cmd != spacer {
            back_input($globals);
        }
        // end
        use crate::pascal::u8_from_m_to_n;
        use crate::section_0016::incr;
        use crate::section_0121::free_avail;
        use crate::section_0101::small_number;
        use crate::section_0102::round_decimals;
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0120::get_avail;
        use crate::section_0207::spacer;
        use crate::section_0365::get_token;
        use crate::section_0380::get_x_token;
        use crate::section_0118::link;
        use crate::section_0445::zero_token;
        use crate::section_0118::info_inner;
        use crate::section_0325::back_input;
    }}
