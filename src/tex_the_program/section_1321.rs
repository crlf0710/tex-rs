//! ` `
// @<Undump the font information@>=
macro_rules! Undump_the_font_information {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        // undump_size(7)(font_mem_size)('font mem size')(fmem_ptr);
        undump_size!(
            $globals,
            7,
            font_mem_size,
            "font mem size",
            $globals.fmem_ptr,
            font_index::new,
            $lbl_bad_fmt
        );
        // for k:=0 to fmem_ptr-1 do undump_wd(font_info[k]);
        for k in 0..=($globals.fmem_ptr.get() - 1) {
            undump_wd!($globals, $globals.font_info[k]);
        }
        // undump_size(font_base)(font_max)('font max')(font_ptr);
        undump_size!(
            $globals,
            font_base,
            font_max,
            "font max",
            $globals.font_ptr,
            internal_font_number::new,
            $lbl_bad_fmt
        );
        // for k:=null_font to font_ptr do
        for k in null_font.get()..=$globals.font_ptr.get() {
            // @<Undump the array info for internal font number |k|@>
            Undump_the_array_info_for_internal_font_number_k!($globals, k, $lbl_bad_fmt);
        }
        use crate::section_0011::font_mem_size;
        use crate::section_0011::font_max;
        use crate::section_0012::font_base;
        use crate::section_0232::null_font;
        use crate::section_0548::font_index;
        use crate::section_0548::internal_font_number;
    }};
}
