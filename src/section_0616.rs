//! @ The actual distances by which we want to move might be computed as the
//! sum of several separate movements. For example, there might be several
//! glue nodes in succession, or we might want to move right by the width of
//! some box plus some amount of glue. More importantly, the baselineskip
//! distances are computed in terms of glue together with the depth and
//! height of adjacent boxes, and we want the \.{DVI} file to lump these
//! three quantities together into a single motion.
//!
//! Therefore, \TeX\ maintains two pairs of global variables: |dvi_h| and |dvi_v|
//! are the |h| and |v| coordinates corresponding to the commands actually
//! output to the \.{DVI} file, while |cur_h| and |cur_v| are the coordinates
//! corresponding to the current state of the output routines. Coordinate
//! changes will accumulate in |cur_h| and |cur_v| without being reflected
//! in the output, until such a change becomes necessary or desirable; we
//! can call the |movement| procedure whenever we want to make |dvi_h=cur_h|
//! or |dvi_v=cur_v|.
//!
//! The current font reflected in the \.{DVI} output is called |dvi_f|;
//! there is no need for a `\\{cur\_f}' variable.
//!
//! The depth of nesting of |hlist_out| and |vlist_out| is called |cur_s|;
//! this is essentially the depth of |push| commands in the \.{DVI} output.
//
// @d synch_h==if cur_h<>dvi_h then
//     begin movement(cur_h-dvi_h,right1); dvi_h:=cur_h;
//     end
// @d synch_v==if cur_v<>dvi_v then
//     begin movement(cur_v-dvi_v,down1); dvi_v:=cur_v;
//     end
//
// @<Glob...@>=
// @!dvi_h,@!dvi_v:scaled; {a \.{DVI} reader program thinks we are here}
/// a `DVI` reader program thinks we are here
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_h: scaled = scaled::zero();
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_v: scaled = scaled::zero();
// @!cur_h,@!cur_v:scaled; {\TeX\ thinks we are here}
/// `TeX` thinks we are here
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_h: scaled = scaled::zero();
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_v: scaled = scaled::zero();
// @!dvi_f:internal_font_number; {the current font}
/// the current font
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_f: internal_font_number = internal_font_number::default();
// @!cur_s:integer; {current depth of output box nesting, initially $-1$}
/// current depth of output box nesting, initially `-1`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_s: integer = -1;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
