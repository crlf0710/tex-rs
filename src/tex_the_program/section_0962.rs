//! ` `

// @<Append a new letter or a hyphen level@>=
pub(crate) macro Append_a_new_letter_or_a_hyphen_level($globals:expr, $digit_sensed:expr, $k:expr) {{
    crate::trace_span_verbose!("Append a new letter or a hyphen level");
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
            $globals.cur_chr = chr_code_type::new(lc_code!($globals, $globals.cur_chr.into()) as _);
            // if cur_chr=0 then
            if $globals.cur_chr.get() == 0 as chr_code_repr {
                // begin print_err("Nonletter");
                print_err!($globals, crate::strpool_str!("Nonletter"));
                // @.Nonletter@>
                // help1("(See Appendix H.)"); error;
                help1!($globals, crate::strpool_str!("(See Appendix H.)"));
                error($globals)?;
                // end;
            }
            // end;
        }
        // if k<63 then
        if $k < 63 {
            // begin incr(k); hc[k]:=cur_chr; hyf[k]:=0; digit_sensed:=false;
            incr!($k);
            $globals.hc[$k.get() as usize] = $globals.cur_chr.get() as _;
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
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0082::error;
    use crate::section_0230::lc_code;
    use crate::section_0297::chr_code_repr;
    use crate::section_0297::chr_code_type;
}}
