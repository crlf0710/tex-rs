//! @ Font parameters are referred to as |slant(f)|, |space(f)|, etc.
//!
//! @d param_end(#)==param_base[#]].sc
//! @d param(#)==font_info[#+param_end
//! @d slant==param(slant_code) {slant to the right, per unit distance upward}
//! @d space==param(space_code) {normal space between words}
//! @d space_stretch==param(space_stretch_code) {stretch between words}
//! @d space_shrink==param(space_shrink_code) {shrink between words}
//! @d x_height==param(x_height_code) {one ex}
//! @d quad==param(quad_code) {one em}
//! @d extra_space==param(extra_space_code) {additional space at end of sentence}
//!
//! @<The em width for |cur_font|@>=quad(cur_font)
//!
