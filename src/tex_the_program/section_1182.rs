//! ` `

// @<Use code |c|...@>=
pub(crate) macro Use_code_c_to_distinguish_between_generalized_fractions($globals:expr, $c:expr) {{
    // if c>=delimited_code then
    if $c >= delimited_code as _ {
        // begin scan_delimiter(left_delimiter(incompleat_noad),false);
        scan_delimiter(
            $globals,
            left_delimiter!(incompleat_noad!($globals)) as _,
            false,
        )?;
        // scan_delimiter(right_delimiter(incompleat_noad),false);
        scan_delimiter(
            $globals,
            right_delimiter!(incompleat_noad!($globals)) as _,
            false,
        )?;
        // end;
    }
    // case c mod delimited_code of
    let c_mod_delimited_code = $c % (delimited_code as chr_code_repr);
    // above_code: begin scan_normal_dimen;
    if c_mod_delimited_code == above_code as _ {
        scan_normal_dimen!($globals)?;
        // thickness(incompleat_noad):=cur_val;
        thickness!($globals, incompleat_noad!($globals) as pointer) =
            scaled::new_from_inner($globals.cur_val);
        // end;
    }
    // over_code: thickness(incompleat_noad):=default_code;
    else if c_mod_delimited_code == over_code as _ {
        thickness!($globals, incompleat_noad!($globals) as pointer) = default_code;
    }
    // atop_code: thickness(incompleat_noad):=0;
    else if c_mod_delimited_code == atop_code as _ {
        thickness!($globals, incompleat_noad!($globals) as pointer) = scaled::zero();
    }
    // end {there are no other cases}
    else {
        /// there are no other cases
        unreachable!();
    }
    use crate::section_0101::scaled;
    use crate::section_0115::pointer;
    use crate::section_0213::incompleat_noad;
    use crate::section_0297::chr_code_repr;
    use crate::section_0448::scan_normal_dimen;
    use crate::section_0683::default_code;
    use crate::section_0683::left_delimiter;
    use crate::section_0683::right_delimiter;
    use crate::section_0683::thickness;
    use crate::section_1160::scan_delimiter;
    use crate::section_1178::above_code;
    use crate::section_1178::atop_code;
    use crate::section_1178::delimited_code;
    use crate::section_1178::over_code;
}}
