//! ` `

// @<Compute result of |register| or |advance|...@>=
macro_rules! Compute_result_of_register_or_advance_put_it_in_cur_val {
    ($globals:expr, $p:expr, $q:expr) => {{
        trace_span!("Compute result of `register` or `advance`...");
        // if p<glue_val then
        if $p < glue_val {
            // begin if p=int_val then scan_int@+else scan_normal_dimen;
            if $p == int_val {
                scan_int($globals)?;
            } else {
                todo!("not int_val");
            }
            // if q=advance then cur_val:=cur_val+eqtb[l].int;
            if $q == advance as _ {
                todo!("advance");
            }
            // end
        }
        // else  begin scan_glue(p);
        else {
            // if q=advance then @<Compute the sum of two glue specs@>;
            // end
        }
    }}
}
