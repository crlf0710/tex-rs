//! @ When the user gives a new identifier to a font that was previously loaded,
//! the new name becomes the font identifier of record. Font names `\.{xyz}' and
//! `\.{XYZ}' are considered to be different.
//
// @<If this font has already been loaded...@>=
macro_rules! If_this_font_has_already_been_loaded_set_f_to_the_internal_font_number_and_goto_common_ending {
    ($globals: expr, $s:expr) => {{
        /// string not yet referenced
        let flushable_string: str_number;
        // flushable_string:=str_ptr-1;
        flushable_string = $globals.str_ptr - 1;
        // for f:=font_base+1 to font_ptr do
        for f in font_base as u16 + 1 ..= $globals.font_ptr.get() {
            let f: internal_font_number = f.into();
            todo!();
            // if str_eq_str(font_name[f],cur_name)and str_eq_str(font_area[f],cur_area) then
            //   begin if cur_name=flushable_string then
            //     begin flush_string; cur_name:=font_name[f];
            //     end;
            //   if s>0 then
            //     begin if s=font_size[f] then goto common_ending;
            //     end
            //   else if font_size[f]=xn_over_d(font_dsize[f],-s,1000) then
            //     goto common_ending;
            //   end
        }
        use crate::section_0012::font_base;
    }}
}