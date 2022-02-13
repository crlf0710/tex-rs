//! @ When the case of a |chr_code| changes, we don't change the |cmd|.
//! We also change active characters, using the fact that
//! |cs_token_flag+active_base| is a multiple of~256.
//! @^data structure assumptions@>
//
// @<Change the case of the token in |p|, if a change is appropriate@>=
pub(crate) macro Change_the_case_of_the_token_in_p_if_a_change_is_appropriate($globals:expr, $p:expr, $b:expr) {
    // t:=info(p);
    let t = info_tok!($globals, $p);
    #[cfg(not(feature = "unicode_support"))]
    {
        todo!("not yet implemented in {}", file!());
        // if t<cs_token_flag+single_base then
        //   begin c:=t mod 256;
        //   if equiv(b+c)<>0 then info(p):=t-c+equiv(b+c);
        //   end
    }
    #[cfg(feature = "unicode_support")]
    {
        let c;
        if let Some(t_cs) = t.get_cs() {
            if t_cs < single_base as pointer {
                c = Some((t_cs % 256) as u8);
            } else {
                c = None;
            }
        } else if let Some((_, chr)) = t.get_cmd_and_chr() {
            if chr.get() < 256 {
                c = Some((chr.get() % 256) as u8)
            } else {
                todo!("not yet implemented in {}", file!());
            }
        } else {
            c = None;
        }
        if let Some(c) = c {
            if equiv!($globals, $b + c as pointer) != 0 {
                info_tok_assign!(
                    $globals,
                    $p,
                    cur_tok_type::new(
                        t.get() - c as cur_tok_repr
                            + equiv!($globals, $b + c as pointer) as cur_tok_repr
                    )
                );
            }
        }

        use crate::section_0222::single_base;
        use crate::section_0297::cur_tok_repr;
        use crate::section_0297::cur_tok_type;
    }
    use crate::section_0115::pointer;
    use crate::section_0118::info_tok;
    use crate::section_0118::info_tok_assign;
    use crate::section_0221::equiv;
}
