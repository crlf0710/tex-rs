//! @ Besides the arrays just enumerated, we have directory arrays that make it
//! easy to get at the individual entries in |font_info|. For example, the
//! |char_info| data for character |c| in font |f| will be in
//! |font_info[char_base[f]+c].qqqq|; and if |w| is the |width_index|
//! part of this word (the |b0| field), the width of the character is
//! |font_info[width_base[f]+w].sc|. (These formulas assume that
//! |min_quarterword| has already been added to |c| and to |w|, since \TeX\
//! stores its quarterwords that way.)
//
// @<Glob...@>=
// @!char_base:array[internal_font_number] of integer;
//   {base addresses for |char_info|}
/// base addresses for `char_info`
#[globals_struct_field(TeXGlobals)]
pub(crate) static char_base: internal_font_array<integer> = internal_font_array::default();
// @!width_base:array[internal_font_number] of integer;
//   {base addresses for widths}
/// base addresses for widths
#[globals_struct_field(TeXGlobals)]
pub(crate) static width_base: internal_font_array<integer> = internal_font_array::default();
// @!height_base:array[internal_font_number] of integer;
//   {base addresses for heights}
/// base addresses for heights
#[globals_struct_field(TeXGlobals)]
pub(crate) static height_base: internal_font_array<integer> = internal_font_array::default();
// @!depth_base:array[internal_font_number] of integer;
//   {base addresses for depths}
/// base addresses for depths
#[globals_struct_field(TeXGlobals)]
pub(crate) static depth_base: internal_font_array<integer> = internal_font_array::default();
// @!italic_base:array[internal_font_number] of integer;
//   {base addresses for italic corrections}
// @!lig_kern_base:array[internal_font_number] of integer;
//   {base addresses for ligature/kerning programs}
// @!kern_base:array[internal_font_number] of integer;
//   {base addresses for kerns}
// @!exten_base:array[internal_font_number] of integer;
//   {base addresses for extensible recipes}
// @!param_base:array[internal_font_number] of integer;
//   {base addresses for font parameters}
/// base addresses for font parameters
#[globals_struct_field(TeXGlobals)]
pub(crate) static param_base: internal_font_array<integer> = internal_font_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0549::internal_font_array;

use globals_struct::{globals_struct_field, globals_struct_use};
