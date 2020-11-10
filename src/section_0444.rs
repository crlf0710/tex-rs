//! ` `

// @<Scan a numeric constant@>=
macro_rules! Scan_a_numeric_constant {
    ($globals:expr) => {{
        // begin radix:=10; m:=214748364;
        $globals.radix = 10.into();
        let mut m = 214748364;
        let mut d;
        // if cur_tok=octal_token then
        if $globals.cur_tok == octal_token {
            // begin radix:=8; m:=@'2000000000; get_x_token;
            $globals.radix = 8.into();
            m = 0o2000000000;
            get_x_token($globals)?;
            // end
        }
        // else if cur_tok=hex_token then
        else if $globals.cur_tok == hex_token {
            // begin radix:=16; m:=@'1000000000; get_x_token;
            $globals.radix = 16.into();
            m = 0o1000000000;
            get_x_token($globals)?;
            // end;
        }
        // vacuous:=true; cur_val:=0;@/
        let mut vacuous = true;
        $globals.cur_val = 0;
        // @<Accumulate the constant until |cur_tok| is not a suitable digit@>;
        Accumulate_the_constant_until_cur_tok_is_not_a_suitable_digit!($globals, d, m, vacuous);
        // if vacuous then @<Express astonishment that no number was here@>
        if vacuous {
            Express_astonishment_that_no_number_was_here!($globals);
        }
        // else if cur_cmd<>spacer then back_input;
        else if $globals.cur_cmd != spacer {
            back_input($globals);
        }
        // end
        use crate::section_0207::spacer;
        use crate::section_0325::back_input;
        use crate::section_0380::get_x_token;
        use crate::section_0438::octal_token;
        use crate::section_0438::hex_token;
    }}
}