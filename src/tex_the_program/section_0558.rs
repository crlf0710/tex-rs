//! @ Font parameters are referred to as |slant(f)|, |space(f)|, etc.
//
// @d param_end(#)==param_base[#]].sc
// @d param(#)==font_info[#+param_end
macro_rules! param {
    ($globals:expr, $p:expr, $v:expr) => {
        $globals.font_info[($v as crate::pascal::integer + $globals.param_base[$p as pointer])
            as crate::section_0548::font_index_repr][crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d slant==param(slant_code) {slant to the right, per unit distance upward}
// @d space==param(space_code) {normal space between words}
// @d space_stretch==param(space_stretch_code) {stretch between words}
// @d space_shrink==param(space_shrink_code) {shrink between words}
// @d x_height==param(x_height_code) {one ex}
// @d quad==param(quad_code) {one em}
// @d extra_space==param(extra_space_code) {additional space at end of sentence}
/// additional space at end of sentence
macro_rules! extra_space {
    ($globals:expr, $p:expr) => {
        param!($globals, $p, crate::section_0547::extra_space_code)
    };
}

// @<The em width for |cur_font|@>=quad(cur_font)
//
