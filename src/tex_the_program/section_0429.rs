//! @ When a |glue_val| changes to a |dimen_val|, we use the width component
//! of the glue; there is no need to decrease the reference count, since it
//! has not yet been increased.  When a |dimen_val| changes to an |int_val|,
//! we use scaled points so that the value doesn't actually change. And when a
//! |mu_val| changes to a |glue_val|, the value doesn't change either.
//
// @<Convert \(c)|cur_val| to a lower level@>=
pub(crate) macro Convert_cur_val_to_a_lower_level($globals:expr) {{
    // begin if cur_val_level=glue_val then cur_val:=width(cur_val)
    if $globals.cur_val_level == cur_val_level_kind::glue_val {
        $globals.cur_val = width!($globals, $globals.cur_val as pointer).inner();
    }
    // else if cur_val_level=mu_val then mu_error;
    else if $globals.cur_val_level == cur_val_level_kind::mu_val {
        mu_error($globals)?;
    }
    // decr(cur_val_level);
    $globals.cur_val_level = ($globals.cur_val_level as u8 - 1).into();
    // end
    use crate::section_0115::pointer;
    use crate::section_0135::width;
    use crate::section_0408::mu_error;
    use crate::section_0410::cur_val_level_kind;
}}
