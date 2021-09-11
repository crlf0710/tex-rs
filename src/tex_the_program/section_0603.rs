//! ` `
// @<Output the font name whose internal number is |f|@>=
pub(crate) macro Output_the_font_name_whose_internal_number_is_f($globals:expr, $f:expr) {{
    // for k:=str_start[font_area[f]] to str_start[font_area[f]+1]-1 do
    for k in $globals.str_start[$globals.font_area[$f]].get()
        ..=$globals.str_start[$globals.font_area[$f] + 1].get() - 1
    {
        // dvi_out(so(str_pool[k]));
        dvi_out!($globals, $globals.str_pool[k].numeric_value());
    }
    // for k:=str_start[font_name[f]] to str_start[font_name[f]+1]-1 do
    for k in $globals.str_start[$globals.font_name[$f]].get()
        ..=$globals.str_start[$globals.font_name[$f] + 1].get() - 1
    {
        // dvi_out(so(str_pool[k]))
        dvi_out!($globals, $globals.str_pool[k].numeric_value());
    }
    use crate::section_0598::dvi_out;
}}
