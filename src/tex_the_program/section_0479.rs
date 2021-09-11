//! ` `

// @<Look for parameter number...@>=
pub(crate) macro Look_for_parameter_number_or_sharpsharp($globals:expr, $xpand:expr, $t:expr) {{
    // begin s:=cur_tok;
    let s = $globals.cur_tok;
    // if xpand then get_x_token else get_token;
    if $xpand {
        get_x_token($globals)?;
    } else {
        get_token($globals)?;
    }
    // if cur_cmd<>mac_param then
    if $globals.cur_cmd != mac_param {
        //   if (cur_tok<=zero_token)or(cur_tok>t) then
        if $globals.cur_tok <= zero_token || $globals.cur_tok > $t {
            todo!();
            //     begin print_err("Illegal parameter number in definition of ");
            // @.Illegal parameter number...@>
            //     sprint_cs(warning_index);
            //     help3("You meant to type ## instead of #, right?")@/
            //     ("Or maybe a } was forgotten somewhere earlier, and things")@/
            //     ("are all screwed up? I'm going to assume that you meant ##.");
            //     back_error; cur_tok:=s;
            //     end
        }
        //   else cur_tok:=out_param_token-"0"+cur_chr;
        else {
            $globals.cur_tok = cur_tok_type::new(
                out_param_token - b'0' as cur_tok_repr + $globals.cur_chr.get() as cur_tok_repr,
            );
        }
    }
    // end
    use crate::section_0207::*;
    use crate::section_0289::out_param_token;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0297::cur_tok_type;
    use crate::section_0365::get_token;
    use crate::section_0380::get_x_token;
    use crate::section_0445::zero_token;
}}
