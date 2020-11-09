//! @ A space is ignored after an alphabetic character constant, so that
//! such constants behave like numeric ones.
//
// @<Scan an alphabetic character code into |cur_val|@>=
macro_rules! Scan_an_alphabetic_character_code_into_cur_val {
    ($globals:expr) => {{
        trace_span!("Scan an alphabetic character code...");
        // begin get_token; {suppress macro expansion}
        /// suppress macro expansion
        get_token($globals)?;
        // if cur_tok<cs_token_flag then
        match $globals.cur_tok.get_cs() {
            None => {
                // begin cur_val:=cur_chr;
                $globals.cur_val = $globals.cur_chr.get() as _;
                // if cur_cmd<=right_brace then
                if $globals.cur_cmd <= right_brace {
                    // if cur_cmd=right_brace then incr(align_state)
                    if $globals.cur_cmd == right_brace {
                        incr!($globals.align_state);
                    }
                    // else decr(align_state);
                    else {
                        decr!($globals.align_state);
                    }
                }
                // end
            },
            Some(cs) => {
                // else if cur_tok<cs_token_flag+single_base then
                if cs < single_base as _ {
                    // cur_val:=cur_tok-cs_token_flag-active_base
                    $globals.cur_val = (cs - active_base as pointer) as _;
                }
                // else cur_val:=cur_tok-cs_token_flag-single_base;
                else {
                    $globals.cur_val = (cs - single_base as pointer) as _;
                }
            }
        }
        // if cur_val>255 then
        //   begin print_err("Improper alphabetic constant");
        // @.Improper alphabetic constant@>
        //   help2("A one-character control sequence belongs after a ` mark.")@/
        //     ("So I'm essentially inserting \0 here.");
        //   cur_val:="0"; back_error;
        //   end
        // else @<Scan an optional space@>;
        // end
        trace_expr!("cur_val={}", $globals.cur_val);
        use crate::section_0115::pointer;
        use crate::section_0207::right_brace;
        use crate::section_0222::single_base;
        use crate::section_0222::active_base;
        use crate::section_0365::get_token;
    }}
}
