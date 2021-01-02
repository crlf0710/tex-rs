//! ` `

// @<Fetch a font integer@>=
macro_rules! Fetch_a_font_integer {
    ($globals:expr, $m:expr) => {{
        // begin scan_font_ident;
        scan_font_ident($globals)?;
        // if m=0 then scanned_result(hyphen_char[cur_val])(int_val)
        if $m.get() == 0 {
            scanned_result!(
                $globals,
                $globals.hyphen_char[internal_font_number::new($globals.cur_val as _)],
                cur_val_level_kind::int_val
            );
        }
        // else scanned_result(skew_char[cur_val])(int_val);
        else {
            scanned_result!(
                $globals,
                $globals.skew_char[internal_font_number::new($globals.cur_val as _)],
                cur_val_level_kind::int_val
            );
        }
        // end
        use crate::section_0548::internal_font_number;
        use crate::section_0577::scan_font_ident;
    }};
}
