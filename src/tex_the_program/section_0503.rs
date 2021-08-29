//! @ Here we use the fact that |"<"|, |"="|, and |">"| are consecutive ASCII
//! codes.
//! @^ASCII code@>
//
// @<Test relation between integers or dimensions@>=
macro_rules! Test_relation_between_integers_or_dimensions {
    ($globals:expr, $this_if:expr, $b:expr) => {{
        /// relation to be evaluated
        let r: u8;
        /// to be tested against the second operand
        let n: integer;
        // begin if this_if=if_int_code then scan_int@+else scan_normal_dimen;
        if $this_if == if_int_code {
            scan_int($globals)?;
        } else {
            scan_normal_dimen!($globals)?;
        }
        // n:=cur_val; @<Get the next non-blank non-call...@>;
        n = $globals.cur_val;
        Get_the_next_non_blank_non_call_token!($globals);
        // if (cur_tok>=other_token+"<")and(cur_tok<=other_token+">") then
        if $globals.cur_tok >= other_token + b'<' as cur_tok_repr &&
            $globals.cur_tok <= other_token + b'>' as cur_tok_repr {
            // r:=cur_tok-other_token
            r = ($globals.cur_tok.get() - other_token) as u8;
        }
        // else  begin print_err("Missing = inserted for ");
        else {
            print_err!($globals, strpool_str!("Missing = inserted for "));
            // @.Missing = inserted@>
            // print_cmd_chr(if_test,this_if);
            print_cmd_chr($globals, if_test, chr_code_type::new($this_if.get() as _));
            // help1("I was expecting to see `<', `=', or `>'. Didn't.");
            help1!($globals, strpool_str!("I was expecting to see `<', `=', or `>'. Didn't."));
            // back_error; r:="=";
            back_error($globals)?;
            r = b'=';
            // end;
        }
        // if this_if=if_int_code then scan_int@+else scan_normal_dimen;
        if $this_if == if_int_code {
            scan_int($globals)?;
        } else {
            scan_normal_dimen!($globals)?;
        }
        // case r of
        // "<": b:=(n<cur_val);
        if r == b'<' {
            $b = n < $globals.cur_val;
        }
        // "=": b:=(n=cur_val);
        else if r == b'=' {
            $b = n == $globals.cur_val;
        }
        // ">": b:=(n>cur_val);
        else if r == b'>' {
            $b = n > $globals.cur_val;
        }
        // end;
        else {
            unreachable!();
        }
        // end
        use crate::pascal::integer;
        use crate::section_0210::if_test;
        use crate::section_0289::other_token;
        use crate::section_0297::cur_tok_repr;
        use crate::section_0297::chr_code_type;
        use crate::section_0298::print_cmd_chr;
        use crate::section_0327::back_error;
        use crate::section_0440::scan_int;
    }}
}