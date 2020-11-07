//! ` `

// @<Scan a numeric constant@>=
macro_rules! Scan_a_numeric_constant {
    ($globals:expr) => {{
        // begin radix:=10; m:=214748364;
        $globals.radix = 10.into();
        let m = 214748364;
        // if cur_tok=octal_token then
        //   begin radix:=8; m:=@'2000000000; get_x_token;
        //   end
        // else if cur_tok=hex_token then
        //   begin radix:=16; m:=@'1000000000; get_x_token;
        //   end;
        // vacuous:=true; cur_val:=0;@/
        // @<Accumulate the constant until |cur_tok| is not a suitable digit@>;
        // if vacuous then @<Express astonishment that no number was here@>
        // else if cur_cmd<>spacer then back_input;
        // end
    }}
}