//! ` `

// @<Append a new letter or a hyphen level@>=
macro_rules! Append_a_new_letter_or_a_hyphen_level {
    ($globals:expr, $digit_sensed:expr, $k:expr) => {{
        // if digit_sensed or(cur_chr<"0")or(cur_chr>"9") then
        if $digit_sensed
            || $globals.cur_chr.get() < b'0' as chr_code_repr
            || $globals.cur_chr.get() > b'9' as chr_code_repr
        {
            // begin if cur_chr="." then cur_chr:=0 {edge-of-word delimiter}
            if $globals.cur_chr.get() == b'.' as chr_code_repr {
                /// edge-of-word delimiter
                const _: () = ();
                $globals.cur_chr = chr_code_type::new(0);
            }
            // else  begin cur_chr:=lc_code(cur_chr);
            else {
                $globals.cur_chr =
                    chr_code_type::new(lc_code!($globals, $globals.cur_chr.into()) as _);
                // if cur_chr=0 then
                if $globals.cur_chr.get() == 0 as chr_code_repr {
                    // begin print_err("Nonletter");
                    print_err!($globals, strpool_str!("Nonletter"));
                    // @.Nonletter@>
                    // help1("(See Appendix H.)"); error;
                    help1!($globals, strpool_str!("(See Appendix H.)"));
                    error($globals)?;
                    // end;
                }
                // end;
            }
            // if k<63 then
            if $k < 63 {
                // begin incr(k); hc[k]:=cur_chr; hyf[k]:=0; digit_sensed:=false;
                incr!($k);
                $globals.hc[$k.get() as usize] =
                    ASCII_code::from($globals.cur_chr.get() as integer);
                $globals.hyf[$k.get() as usize] = 0.into();
                $digit_sensed = false;
                // end;
            }
        // end
        }
        // else if k<63 then
        else if $k < 63 {
            // begin hyf[k]:=cur_chr-"0"; digit_sensed:=true;
            $globals.hyf[$k.get() as usize] =
                u8_from_0_to_n::new(($globals.cur_chr.get() - b'0' as chr_code_repr) as _);
            $digit_sensed = true;
            // end
        }

        use crate::pascal::integer;
        use crate::section_0018::ASCII_code;
        use crate::section_0297::chr_code_repr;
        use crate::section_0297::chr_code_type;
    }};
}
