//! @ Input files that can't be found in the user's area may appear in a standard
//! system area called |TEX_area|. Font metric files whose areas are not given
//! explicitly are assumed to appear in a standard system area called
//! |TEX_font_area|.  These system area names will, of course, vary from place
//! to place.
//! @^system dependencies@>
//
// @d TEX_area=="TeXinputs:"
pub(crate) macro TEX_area() {
    crate::strpool_str!("TeXinputs:")
}
// @.TeXinputs@>
// @d TEX_font_area=="TeXfonts:"
pub(crate) macro TEX_font_area() {
    crate::strpool_str!("TeXfonts:")
}
// @.TeXfonts@>
//
