//! ` `

// @<Look for parameter number...@>=
macro_rules! Look_for_parameter_number_or_sharpsharp {
    ($globals:expr, $xpand:expr) => {
        // begin s:=cur_tok;
        let s = $globals.cur_tok;
        // if xpand then get_x_token else get_token;
        if $xpand {
            get_x_token($globals)?;
        } else {
            get_token($globals)?;
        }
        // if cur_cmd<>mac_param then
        //   if (cur_tok<=zero_token)or(cur_tok>t) then
        //     begin print_err("Illegal parameter number in definition of ");
        // @.Illegal parameter number...@>
        //     sprint_cs(warning_index);
        //     help3("You meant to type ## instead of #, right?")@/
        //     ("Or maybe a } was forgotten somewhere earlier, and things")@/
        //     ("are all screwed up? I'm going to assume that you meant ##.");
        //     back_error; cur_tok:=s;
        //     end
        //   else cur_tok:=out_param_token-"0"+cur_chr;
        // end
    }
}