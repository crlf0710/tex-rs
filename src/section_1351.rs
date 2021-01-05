//! ` `
// @<Implement \.{\\openout}@>=
macro_rules! Implement_openout {
    ($globals:expr) => {{
        // begin new_write_whatsit(open_node_size);
        new_write_whatsit($globals, open_node_size.into())?;
        // scan_optional_equals; scan_file_name;@/
        scan_optional_equals($globals)?;
        scan_file_name($globals)?;
        // open_name(tail):=cur_name; open_area(tail):=cur_area; open_ext(tail):=cur_ext;
        open_name!($globals, tail!($globals)) = $globals.cur_name.get() as _;
        open_area!($globals, tail!($globals)) = $globals.cur_area.get() as _;
        open_ext!($globals, tail!($globals)) = $globals.cur_ext.get() as _;
        // end
        use crate::section_0405::scan_optional_equals;
        use crate::section_0526::scan_file_name;
        use crate::section_1341::open_node_size;
        use crate::section_1350::new_write_whatsit;
    }}
}
