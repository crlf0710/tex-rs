//! @ A space is ignored after an alphabetic character constant, so that
//! such constants behave like numeric ones.
//
// @<Scan an alphabetic character code into |cur_val|@>=
macro_rules! Scan_an_alphabetic_character_code_into_cur_val {
    ($globals:expr) => {{
        // begin get_token; {suppress macro expansion}
        /// suppress macro expansion
        get_token($globals)?;
        // if cur_tok<cs_token_flag then
        //   begin cur_val:=cur_chr;
        //   if cur_cmd<=right_brace then
        //     if cur_cmd=right_brace then incr(align_state)
        //     else decr(align_state);
        //   end
        // else if cur_tok<cs_token_flag+single_base then
        //   cur_val:=cur_tok-cs_token_flag-active_base
        // else cur_val:=cur_tok-cs_token_flag-single_base;
        // if cur_val>255 then
        //   begin print_err("Improper alphabetic constant");
        // @.Improper alphabetic constant@>
        //   help2("A one-character control sequence belongs after a ` mark.")@/
        //     ("So I'm essentially inserting \0 here.");
        //   cur_val:="0"; back_error;
        //   end
        // else @<Scan an optional space@>;
        // end
        use crate::section_0365::get_token;
    }}
}
