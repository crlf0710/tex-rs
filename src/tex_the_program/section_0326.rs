//! ` `

// @<Insert token |p| into \TeX's input@>=
pub(crate) macro Insert_token_p_into_TeX_s_input($globals:expr, $p:expr) {{
    /// saved value of `cur_tok`
    let t;
    // begin t:=cur_tok; cur_tok:=p; back_input; cur_tok:=t;
    t = $globals.cur_tok;
    #[cfg(not(feature = "unicode_support"))]
    {
        $globals.cur_tok = cur_tok_type::new($p as _);
    }
    #[cfg(feature = "unicode_support")]
    {
        $globals.cur_tok = crate::section_0297::cur_tok_type::new(
            crate::unicode_support::info_value($globals, $p),
        );
    }
    back_input($globals);
    $globals.cur_tok = t;
    // end
    use crate::section_0297::cur_tok_type;
    use crate::section_0325::back_input;
}}
