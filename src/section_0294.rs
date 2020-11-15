//! @ The procedure usually ``learns'' the character code used for macro
//! parameters by seeing one in a |match| command before it runs into any
//! |out_param| commands.

// @<Display the token ...@>=
macro_rules! Display_the_token_m_c {
    ($globals:expr, $m:expr, $c:expr, $n:expr, $match_chr:expr) => {{
        // case m of
        // left_brace,right_brace,math_shift,tab_mark,sup_mark,sub_mark,spacer,
        //   letter,other_char: print(c);
        if $m == left_brace || $m == right_brace || $m == math_shift || $m == tab_mark ||
            $m == sup_mark || $m == sub_mark || $m == spacer || $m == letter || $m == other_char {
            print($globals, $c.get() as _);
        }
        // mac_param: begin print(c); print(c);
        else if $m == mac_param {
            print($globals, $c.get() as _);
            print($globals, $c.get() as _);
            // end;
        }
        // out_param: begin print(match_chr);
        else if $m == out_param {
            print($globals, $match_chr.numeric_value() as _);
            // if c<=9 then print_char(c+"0")
            if $c.get() <= 9 {
                print_char(make_globals_io_string_view!($globals),
                    ASCII_code_literal!($c.get() as u8 + b'0'));
            }
            // else  begin print_char("!"); return;
            else {
                print_char(make_globals_io_string_view!($globals),
                    ASCII_code_literal!(b'!'));
                return;
                // end;
            }
            // end;
        }
        // match: begin match_chr:=c; print(c); incr(n); print_char(n);
        else if $m == r#match {
            $match_chr = $c.into();
            print($globals, $c.get() as _);
            $n = ASCII_code::from(($n.numeric_value() + 1) as integer);
            print_char(make_globals_io_string_view!($globals), $n);
            // if n>"9" then return;
            if $n > ASCII_code_literal!(b'9') {
                return;
            }
            // end;
        }
        // end_match: print("->");
        else if $m == end_match {
            print($globals, strpool_str!("->").get() as _);
        }
        // @.->@>
        // othercases print_esc("BAD.")
        else {
            print_esc($globals, strpool_str!("BAD."));
            // @.BAD@>
        }
        // endcases
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0058::print_char;
        use crate::section_0059::print;
        use crate::section_0207::*;
        
    }}
}
