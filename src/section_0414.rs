//! ` `

// @<Fetch a character code from some table@>=
macro_rules! Fetch_a_character_code_from_some_table {
    ($globals:expr, $m:expr) => {{
        // begin scan_char_num;
        scan_char_num($globals)?;
        // if m=math_code_base then scanned_result(ho(math_code(cur_val)))(int_val)
        if $m.get() as integer == math_code_base as integer {
            todo!("a");
        }
        // else if m<math_code_base then scanned_result(equiv(m+cur_val))(int_val)
        else if ($m.get() as integer) < math_code_base as integer {
            scanned_result!(
                $globals,
                equiv!($globals, $m.get() as integer + $globals.cur_val) as _,
                cur_val_level_kind::int_val
            );
        }
        // else scanned_result(eqtb[m+cur_val].int)(int_val);
        else {
            scanned_result!(
                $globals,
                $globals.eqtb[($m.get() as integer + $globals.cur_val) as pointer][MEMORY_WORD_INT],
                cur_val_level_kind::int_val
            );
        }
        // end
        use crate::pascal::integer;
        use crate::section_0230::math_code_base;
        use crate::section_0434::scan_char_num;
    }};
}
