//! ` `

// @<Compute result of |register| or |advance|...@>=
pub(crate) macro Compute_result_of_register_or_advance_put_it_in_cur_val($globals:expr, $l:expr, $p:expr, $q:expr) {{
    crate::trace_span!("Compute result of `register` or `advance`...");
    // if p<glue_val then
    if $p < glue_val {
        // begin if p=int_val then scan_int@+else scan_normal_dimen;
        if $p == int_val {
            scan_int($globals)?;
        } else {
            scan_normal_dimen!($globals)?;
        }
        // if q=advance then cur_val:=cur_val+eqtb[l].int;
        if $q == advance as pointer {
            $globals.cur_val += $globals.eqtb[$l][MEMORY_WORD_INT];
        }
        // end
    }
    // else  begin scan_glue(p);
    else {
        scan_glue($globals, small_number::new($p as _))?;
        // if q=advance then @<Compute the sum of two glue specs@>;
        if $q == advance as pointer {
            crate::section_1239::Compute_the_sum_of_two_glue_specs!($globals, $l);
        }
        // end
    }
    use crate::section_0101::small_number;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0115::pointer;
    use crate::section_0209::*;
    use crate::section_0410::cur_val_level_kind::*;
    use crate::section_0440::scan_int;
    use crate::section_0448::scan_normal_dimen;
    use crate::section_0461::scan_glue;
}}
