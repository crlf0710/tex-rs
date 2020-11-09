//! @ The procedure usually ``learns'' the character code used for macro
//! parameters by seeing one in a |match| command before it runs into any
//! |out_param| commands.

// @<Display the token ...@>=
macro_rules! Display_the_token_m_c {
    ($globals:expr, $m:expr, $c:expr) => {{
        // case m of
        // left_brace,right_brace,math_shift,tab_mark,sup_mark,sub_mark,spacer,
        //   letter,other_char: print(c);
        if $m == left_brace || $m == right_brace || $m == math_shift || $m == tab_mark ||
            $m == sup_mark || $m == sub_mark || $m == spacer || $m == letter || $m == other_char {
            print($globals, $c.get() as _);
        }
        // mac_param: begin print(c); print(c);
        //   end;
        // out_param: begin print(match_chr);
        //   if c<=9 then print_char(c+"0")
        //   else  begin print_char("!"); return;
        //     end;
        //   end;
        // match: begin match_chr:=c; print(c); incr(n); print_char(n);
        //   if n>"9" then return;
        //   end;
        // end_match: print("->");
        // @.->@>
        // othercases print_esc("BAD.")
        else {
            print_esc($globals, strpool_str!("BAD."));
            // @.BAD@>
        }
        // endcases
        use crate::section_0059::print;
        use crate::section_0207::*;
        
    }}
}
