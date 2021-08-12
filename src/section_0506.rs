//! @ An active character will be treated as category 13 following
//! \.{\\if\\noexpand} or following \.{\\ifcat\\noexpand}. We use the fact that
//! active characters have the smallest tokens, among all control sequences.
//
// @d get_x_token_or_active_char==@t@>@;
macro_rules! get_x_token_or_active_char {
    ($globals:expr) => {{
        // begin get_x_token;
        get_x_token($globals)?;
        // if cur_cmd=relax then if cur_chr=no_expand_flag then
        if $globals.cur_cmd == relax && $globals.cur_chr.get() == no_expand_flag {
            // begin cur_cmd:=active_char;
            $globals.cur_cmd = active_char;
            // cur_chr:=cur_tok-cs_token_flag-active_base;
            $globals.cur_chr =
                chr_code_type::new(($globals.cur_tok.get_cs().unwrap() as word - active_base) as _);
            // end;
        }
        // end
        use crate::pascal::word;
        use crate::section_0222::active_base;
        use crate::section_0358::no_expand_flag;
        use crate::section_0380::get_x_token;
    }};
}

// @<Test if two characters match@>=
macro_rules! Test_if_two_characters_match {
    ($globals:expr, $this_if:expr, $b:expr) => {{
        /// to be tested against the second operand
        let (m, n);

        // begin get_x_token_or_active_char;
        get_x_token_or_active_char!($globals);
        // if (cur_cmd>active_char)or(cur_chr>255) then {not a character}
        if $globals.cur_cmd > active_char
            || $globals.cur_chr.get() > crate::pascal::CHAR_MAX_REPR as chr_code_repr
        {
            /// not a character
            const _: () = ();
            // begin m:=relax; n:=256;
            m = relax;
            n = chr_code_type::new(crate::pascal::CHAR_MAX_REPR as chr_code_repr);
        // end
        }
        // else  begin m:=cur_cmd; n:=cur_chr;
        else {
            m = $globals.cur_cmd;
            n = $globals.cur_chr;
            // end;
        }
        // get_x_token_or_active_char;
        get_x_token_or_active_char!($globals);
        // if (cur_cmd>active_char)or(cur_chr>255) then
        if $globals.cur_cmd > active_char
            || $globals.cur_chr.get() > crate::pascal::CHAR_MAX_REPR as chr_code_repr
        {
            // begin cur_cmd:=relax; cur_chr:=256;
            $globals.cur_cmd = relax;
            $globals.cur_chr = chr_code_type::new(crate::pascal::CHAR_MAX_REPR as chr_code_repr);
            // end;
        }
        // if this_if=if_char_code then b:=(n=cur_chr)@+else b:=(m=cur_cmd);
        if $this_if == if_char_code {
            $b = n == $globals.cur_chr;
        } else {
            $b = m == $globals.cur_cmd;
        }
        // end
        use crate::section_0207::active_char;
        use crate::section_0207::relax;
        use crate::section_0297::chr_code_type;
    }};
}
