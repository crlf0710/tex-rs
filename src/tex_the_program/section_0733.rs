//! ` `

// @<Cases for noads that can follow a |bin_noad|@>=
pub(crate) macro Cases_for_noads_that_can_follow_a_bin_noad($globals:expr, $q:expr, $type_q:expr, $delta:expr, $lbl_check_dimensions:lifetime, $lbl_done_with_noad:lifetime) {{
    // left_noad: goto done_with_noad;
    let processed = if $type_q == left_noad {
        crate::goto_forward_label!($lbl_done_with_noad);
        true
    }
    // fraction_noad: begin make_fraction(q); goto check_dimensions;
    else if $type_q == fraction_noad {
        make_fraction($globals, $q)?;
        crate::goto_forward_label!($lbl_check_dimensions);
        // end;
        true
    }
    // op_noad: begin delta:=make_op(q);
    else if $type_q == op_noad {
        $delta = make_op($globals, $q)?;
        // if subtype(q)=limits then goto check_dimensions;
        if subtype!($globals, $q) == op_noad_subtype::limits as _ {
            crate::goto_forward_label!($lbl_check_dimensions);
        }
        // end;
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
        make_over($globals, $q)?;
        true
    }
    // under_noad: make_under(q);
    else if $type_q == under_noad {
        make_under($globals, $q)?;
        true
    }
    // accent_noad: make_math_accent(q);
    else if $type_q == accent_noad {
        make_math_accent($globals, $q)?;
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
    use crate::section_0133::subtype;
    use crate::section_0682::*;
    use crate::section_0683::*;
    use crate::section_0687::*;
    use crate::section_0734::make_over;
    use crate::section_0735::make_under;
    use crate::section_0736::make_vcenter;
    use crate::section_0737::make_radical;
    use crate::section_0738::make_math_accent;
    use crate::section_0743::make_fraction;
    use crate::section_0749::make_op;
    use crate::section_0752::make_ord;
    processed
}}
