//! @ The global variable |null_character| is set up to be a word of
//! |char_info| for a character that doesn't exist. Such a word provides a
//! convenient way to deal with erroneous situations.
//!
//! @<Glob...@>=
//! @!null_character:four_quarters; {nonexistent character information}
//!
//! @ @<Set init...@>=
//! null_character.b0:=min_quarterword; null_character.b1:=min_quarterword;
//! null_character.b2:=min_quarterword; null_character.b3:=min_quarterword;
//!
//! @ Here are some macros that help process ligatures and kerns.
//! We write |char_kern(f)(j)| to find the amount of kerning specified by
//! kerning command~|j| in font~|f|. If |j| is the |char_info| for a character
//! with a ligature/kern program, the first instruction of that program is either
//! |i=font_info[lig_kern_start(f)(j)]| or |font_info[lig_kern_restart(f)(i)]|,
//! depending on whether or not |skip_byte(i)<=stop_flag|.
//!
//! The constant |kern_base_offset| should be simplified, for \PASCAL\ compilers
//! that do not do local optimization.
//! @^system dependencies@>
//!
//! @d char_kern_end(#)==256*op_byte(#)+rem_byte(#)].sc
//! @d char_kern(#)==font_info[kern_base[#]+char_kern_end
//! @d kern_base_offset==256*(128+min_quarterword)
//! @d lig_kern_start(#)==lig_kern_base[#]+rem_byte {beginning of lig/kern program}
//! @d lig_kern_restart_end(#)==256*op_byte(#)+rem_byte(#)+32768-kern_base_offset
//! @d lig_kern_restart(#)==lig_kern_base[#]+lig_kern_restart_end
//!
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
//! @ @<The x-height for |cur_font|@>=x_height(cur_font)
//!
