//! ` `

// @<Fetch an internal dimension and |goto attach_sign|...@>=
pub(crate) macro Fetch_an_internal_dimension_and_goto_attach_sign__or_fetch_an_internal_integer($globals:expr, $mu:expr, $lbl_attach_sign:lifetime) {{
    // if mu then
    if $mu {
        // begin scan_something_internal(mu_val,false);
        scan_something_internal(
            $globals,
            small_number::new(cur_val_level_kind::mu_val as _),
            false,
        )?;
        // @<Coerce glue to a dimension@>;
        crate::section_0451::Coerce_glue_to_a_dimension!($globals);
        // if cur_val_level=mu_val then goto attach_sign;
        if $globals.cur_val_level == cur_val_level_kind::mu_val {
            crate::goto_forward_label!($lbl_attach_sign);
        }
        // if cur_val_level<>int_val then mu_error;
        if $globals.cur_val_level != cur_val_level_kind::int_val {
            mu_error($globals)?;
        }
        // end
    }
    // else  begin scan_something_internal(dimen_val,false);
    else {
        scan_something_internal(
            $globals,
            small_number::new(cur_val_level_kind::dimen_val as _),
            false,
        )?;
        // if cur_val_level=dimen_val then goto attach_sign;
        if $globals.cur_val_level == cur_val_level_kind::dimen_val {
            crate::goto_forward_label!($lbl_attach_sign);
        }
        // end
    }
    use crate::section_0101::small_number;
    use crate::section_0408::mu_error;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0413::scan_something_internal;
}}
