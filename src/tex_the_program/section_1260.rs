//! @ When the user gives a new identifier to a font that was previously loaded,
//! the new name becomes the font identifier of record. Font names `\.{xyz}' and
//! `\.{XYZ}' are considered to be different.
//
// @<If this font has already been loaded...@>=
pub(crate) macro If_this_font_has_already_been_loaded_set_f_to_the_internal_font_number_and_goto_common_ending($globals: expr, $f:expr, $s:expr, $lbl_common_ending:lifetime) {{
    /// string not yet referenced
    let flushable_string: str_number;
    // flushable_string:=str_ptr-1;
    flushable_string = $globals.str_ptr - 1;
    // for f:=font_base+1 to font_ptr do
    for f in font_base as u16 + 1..=$globals.font_ptr.get() {
        $f = f.into();
        // if str_eq_str(font_name[f],cur_name)and str_eq_str(font_area[f],cur_area) then
        if str_eq_str($globals, $globals.font_name[f], $globals.cur_name)
            && str_eq_str($globals, $globals.font_area[f], $globals.cur_area)
        {
            // begin if cur_name=flushable_string then
            if $globals.cur_name == flushable_string {
                // begin flush_string; cur_name:=font_name[f];
                flush_string($globals);
                $globals.cur_name = $globals.font_name[f];
                // end;
            }
            // if s>0 then
            if $s > scaled::zero() {
                // begin if s=font_size[f] then goto common_ending;
                if $s == $globals.font_size[f] {
                    crate::goto_forward_label!($lbl_common_ending);
                }
            // end
            }
            // else if font_size[f]=xn_over_d(font_dsize[f],-s,1000) then
            else if {
                let font_size_f = $globals.font_size[f];
                let font_dsize_f = $globals.font_dsize[f];
                font_size_f == xn_over_d($globals, font_dsize_f, -$s.inner(), 1000)
            } {
                // goto common_ending;
                crate::goto_forward_label!($lbl_common_ending);
            }
            // end
        }
    }
    use crate::section_0012::font_base;
    use crate::section_0038::str_number;
    use crate::section_0044::flush_string;
    use crate::section_0046::str_eq_str;
    use crate::section_0101::scaled;
    use crate::section_0107::xn_over_d;
}}
