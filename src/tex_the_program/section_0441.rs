//! ` `

// @<Get the next non-blank non-sign token...@>=
pub(crate) macro Get_the_next_non_blank_non_sign_token__set_negative_appropriately($globals:expr, $negative:expr) {{
    crate::trace_span!("Get the next non-blank non-sign token...");
    // negative:=false;
    $negative = false;
    // repeat @<Get the next non-blank non-call token@>;
    loop {
        crate::section_0406::Get_the_next_non_blank_non_call_token!($globals);
        // if cur_tok=other_token+"-" then
        if $globals.cur_tok.get() == other_token + b'-' as cur_tok_repr {
            // begin negative := not negative; cur_tok:=other_token+"+";
            $negative = !$negative;
            $globals.cur_tok = cur_tok_type::new(other_token + b'+' as cur_tok_repr);
            // end;
        }
        // until cur_tok<>other_token+"+"
        if $globals.cur_tok.get() != other_token + b'+' as cur_tok_repr {
            break;
        }
    }
    use crate::section_0289::other_token;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0297::cur_tok_type;
}}
