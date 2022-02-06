//! @ An active character that is an |outer_call| is allowed here.
//
// @<Treat |cur_chr|...@>=
pub(crate) macro Treat_cur_chr_as_an_active_character($globals:expr) {{
    // begin cur_cs:=cur_chr+active_base;
    $globals.cur_cs = crate::section_0230::index_offset_with_ASCII_code(
        crate::section_0222::active_base,
        $globals.cur_chr.into(),
    ) as _;
    // cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
    $globals.cur_cmd = eq_type!($globals, $globals.cur_cs);
    $globals.cur_chr = chr_code_type::new(equiv!($globals, $globals.cur_cs) as _);
    // x_token; back_input;
    x_token($globals)?;
    back_input($globals);
    // end
    use crate::pascal::integer;
    use crate::section_0221::eq_type;
    use crate::section_0221::equiv;
    use crate::section_0297::chr_code_type;
    use crate::section_0325::back_input;
    use crate::section_0381::x_token;
}}
