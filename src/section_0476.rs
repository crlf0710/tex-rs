//! ` `

// @<If the next character is a parameter number...@>=

macro_rules! If_the_next_character_is_a_parameter_number__make_cur_tok_a_match_token__but_if_it_is_a_left_brace__store_left_brace_end_match__set_hash_brace_and_goto_done {
    ($globals:expr, $t:expr, $hash_brace:expr, $lbl_done:lifetime, $p:expr, $q:expr) => {{
        // begin s:=match_token+cur_chr; get_token;
        let s = match_token + $globals.cur_chr.get();
        get_token($globals)?;
        // if cur_cmd=left_brace then
        if $globals.cur_cmd == left_brace {
            // begin hash_brace:=cur_tok;
            $hash_brace = $globals.cur_tok.get();
            // store_new_token(cur_tok); store_new_token(end_match_token);
            store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
            store_new_token!($globals, end_match_token, $p, $q);
            // goto done;
            goto_forward_label!($lbl_done);
            // end;
        }
        // if t=zero_token+9 then
        if $t == (zero_token + 9) as cur_tok_type_repr {
            todo!();
            //   begin print_err("You already have nine parameters");
            // @.You already have nine...@>
            //   help1("I'm going to ignore the # sign you just used."); error;
            //   end
        }
        // else  begin incr(t);
        else {
            incr!($t);
            // if cur_tok<>t then
            trace_expr!("cur_tok = {}", $globals.cur_tok.get());
            trace_expr!("t = {}", $t);
            if $globals.cur_tok.get() != $t as _ {
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
        use crate::section_0207::left_brace;
        use crate::section_0289::match_token;
        use crate::section_0297::cur_tok_type;
    }}
}