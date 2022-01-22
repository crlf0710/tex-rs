//! ` `

// @<Cases for noads that can follow a |bin_noad|@>=
pub(crate) macro Cases_for_noads_that_can_follow_a_bin_noad($globals:expr, $q:expr, $type_q:expr) {{
    // left_noad: goto done_with_noad;
    let processed = if $type_q == left_noad {
        todo!("left_noad");
        true
    }
    // fraction_noad: begin make_fraction(q); goto check_dimensions;
    else if $type_q == fraction_noad {
        todo!("fraction_noad");
        // end;
        true
    }
    // op_noad: begin delta:=make_op(q);
    else if $type_q == op_noad {
        // if subtype(q)=limits then goto check_dimensions;
        // end;
        todo!("op_noad");
        true
    }
    // ord_noad: make_ord(q);
    else if $type_q == ord_noad {
        make_ord($globals, $q)?;
        true
    }
    // open_noad,inner_noad: do_nothing;
    else if $type_q == open_noad || $type_q == inner_noad {
        do_nothing!();
        true
    }
    // radical_noad: make_radical(q);
    else if $type_q == radical_noad {
        make_radical($globals, $q)?;
        true
    }
    // over_noad: make_over(q);
    else if $type_q == over_noad {
        todo!("over_noad");
        true
    }
    // under_noad: make_under(q);
    else if $type_q == under_noad {
        todo!("under_noad");
        true
    }
    // accent_noad: make_math_accent(q);
    else if $type_q == accent_noad {
        todo!("accent_noad");
        true
    }
    // vcenter_noad: make_vcenter(q);
    else if $type_q == vcenter_noad {
        make_vcenter($globals, $q)?;
        true
    } else {
        false
    };
    use crate::section_0016::do_nothing;
    use crate::section_0682::*;
    use crate::section_0683::*;
    use crate::section_0687::*;
    use crate::section_0736::make_vcenter;
    use crate::section_0737::make_radical;
    use crate::section_0752::make_ord;
    processed
}}
