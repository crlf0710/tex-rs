//! ` `

// @<If the next character is a parameter number...@>=

pub(crate) macro If_the_next_character_is_a_parameter_number__make_cur_tok_a_match_token__but_if_it_is_a_left_brace__store_left_brace_end_match__set_hash_brace_and_goto_done($globals:expr, $t:expr, $hash_brace:expr, $lbl_done:lifetime, $lbl_continue:lifetime, $p:expr, $q:expr) {{
    // begin s:=match_token+cur_chr; get_token;
    let s = match_token + $globals.cur_chr.get();
    get_token($globals)?;
    // if cur_tok<left_brace_limit then
    if $globals.cur_tok.get() < left_brace_limit {
        // begin hash_brace:=cur_tok;
        $hash_brace = $globals.cur_tok.get();
        // store_new_token(cur_tok); store_new_token(end_match_token);
        store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
        store_new_token!($globals, end_match_token, $p, $q);
        // goto done;
        crate::goto_forward_label!($lbl_done);
        // end;
    }
    // if t=zero_token+9 then
    if $t == (zero_token + 9) as cur_tok_repr {
        todo!();
        //   begin print_err("You already have nine parameters");
        // @.You already have nine...@>
        //   help2("I'm going to ignore the # sign you just used,")@/
        //     ("as well as the token that followed it."); error; goto continue;
        //   end
    }
    // else  begin incr(t);
    else {
        incr!($t);
        // if cur_tok<>t then
        crate::trace_expr_verbose!("cur_tok = {}", $globals.cur_tok.get());
        crate::trace_expr_verbose!("t = {}", $t);
        if $globals.cur_tok.get() != $t as cur_tok_repr {
            todo!("numbering");
            // begin print_err("Parameters must be numbered consecutively");
            // @.Parameters...consecutively@>
            // help2("I've inserted the digit you should have used after the #.")@/
            //   ("Type `1' to delete what you did use."); back_error;
            // end;
        }
        // cur_tok:=s;
        $globals.cur_tok = cur_tok_type::new(s);
        // end;
    }
    // end
    use crate::section_0016::incr;
    use crate::section_0289::end_match_token;
    use crate::section_0289::left_brace_limit;
    use crate::section_0289::match_token;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0297::cur_tok_type;
    use crate::section_0365::get_token;
    use crate::section_0371::store_new_token;
    use crate::section_0445::zero_token;
}}
