//! ` `

// @d infinity==@'17777777777 {the largest positive value that \TeX\ knows}
// @d zero_token=other_token+"0" {zero, the smallest digit}
/// zero, the smallest digit
pub(crate) const zero_token: cur_tok_repr = other_token + b'0' as cur_tok_repr;
// @d A_token=letter_token+"A" {the smallest special hex digit}
/// the smallest special hex digit
pub(crate) const A_token: cur_tok_repr = letter_token + b'A' as cur_tok_repr;
// @d other_A_token=other_token+"A" {special hex digit of type |other_char|}
/// special hex digit of type `other_char`
pub(crate) const other_A_token: cur_tok_repr = other_token + b'A' as cur_tok_repr;

// @<Accumulate the constant...@>=
pub(crate) macro Accumulate_the_constant_until_cur_tok_is_not_a_suitable_digit($globals:expr, $d:expr, $m:expr, $vacuous:expr) {{
    crate::trace_span_verbose!("Accumulate the constant...");
    crate::region_forward_label! {
    |'done|
    {
        // loop@+  begin if (cur_tok<zero_token+radix)and(cur_tok>=zero_token)and
        //     (cur_tok<=zero_token+9) then d:=cur_tok-zero_token
        loop {
            if $globals.cur_tok < zero_token + $globals.radix.get() as cur_tok_repr &&
                $globals.cur_tok >= zero_token &&
                $globals.cur_tok <= zero_token + 9 {
                $d = $globals.cur_tok.get() as integer - zero_token as integer;
            }
            //   else if radix=16 then
            else if $globals.radix == 16 {
                // if (cur_tok<=A_token+5)and(cur_tok>=A_token) then d:=cur_tok-A_token+10
                if $globals.cur_tok <= A_token + 5 && $globals.cur_tok >= A_token {
                    $d = $globals.cur_tok.get() as integer - A_token as integer + 10;
                }
                // else if (cur_tok<=other_A_token+5)and(cur_tok>=other_A_token) then
                //   d:=cur_tok-other_A_token+10
                else if $globals.cur_tok <= other_A_token + 5 && $globals.cur_tok >= other_A_token {
                    $d = $globals.cur_tok.get() as integer - other_A_token as integer + 10;
                }
                // else goto done
                else {
                    crate::goto_forward_label!('done);
                }
            }
            // else goto done;
            else {
                crate::goto_forward_label!('done);
            }
            // vacuous:=false;
            $vacuous = false;
            //   if (cur_val>=m)and((cur_val>m)or(d>7)or(radix<>10)) then
            if $globals.cur_val >= $m && ($globals.cur_val > $m || $d > 7 || $globals.radix != 10) {
                todo!("not yet implemented in {}", file!());
                //     begin if OK_so_far then
                //       begin print_err("Number too big");
                // @.Number too big@>
                //       help2("I can only go up to 2147483647='17777777777=""7FFFFFFF,")@/
                //         ("so I'm using that number instead of yours.");
                //       error; cur_val:=infinity; OK_so_far:=false;
                //       end;
                //     end
            }
            // else cur_val:=cur_val*radix+d;
            else {
                $globals.cur_val = $globals.cur_val * $globals.radix.get() as integer + $d;
            }
            // get_x_token;
            get_x_token($globals)?;
            // end;
        }
    }
    // done:
    'done <-
    }
    use crate::pascal::integer;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0380::get_x_token;
    use crate::section_0445::other_A_token;
    use crate::section_0445::zero_token;
    use crate::section_0445::A_token;
}}

use crate::section_0289::letter_token;
use crate::section_0289::other_token;
use crate::section_0297::cur_tok_repr;
