//! @ \TeX\ always knows at least one font, namely the null font. It has no
//! characters, and its seven parameters are all equal to zero.
//
// @<Initialize table...@>=
#[distributed_slice(INIT_TBLENTRY)]
#[allow(unused_variables)]
pub(crate) fn initialize_table_entries_done_by_initex_only_0552(globals: &mut TeXGlobals) {
    // font_ptr:=null_font; fmem_ptr:=7;
    globals.font_ptr = null_font;
    globals.fmem_ptr = 7.into();
    // font_name[null_font]:="nullfont"; font_area[null_font]:="";
    globals.font_name[null_font] = crate::strpool_str!("nullfont");
    globals.font_area[null_font] = crate::strpool_str!("");
    // hyphen_char[null_font]:="-"; skew_char[null_font]:=-1;
    globals.hyphen_char[null_font] = b'-' as _;
    globals.skew_char[null_font] = -1;
    // bchar_label[null_font]:=non_address;
    globals.bchar_label[null_font] = non_address;
    // font_bchar[null_font]:=non_char; font_false_bchar[null_font]:=non_char;
    globals.font_bchar[null_font] = non_char;
    globals.font_false_bchar[null_font] = non_char;
    // font_bc[null_font]:=1; font_ec[null_font]:=0;
    globals.font_bc[null_font] = 1.into();
    globals.font_ec[null_font] = 0.into();
    // font_size[null_font]:=0; font_dsize[null_font]:=0;
    globals.font_size[null_font] = scaled::zero();
    globals.font_dsize[null_font] = scaled::zero();
    // char_base[null_font]:=0; width_base[null_font]:=0;
    globals.char_base[null_font] = 0;
    globals.width_base[null_font] = 0;
    // height_base[null_font]:=0; depth_base[null_font]:=0;
    globals.height_base[null_font] = 0;
    globals.depth_base[null_font] = 0;
    // italic_base[null_font]:=0; lig_kern_base[null_font]:=0;
    globals.italic_base[null_font] = 0;
    globals.lig_kern_base[null_font] = 0;
    // kern_base[null_font]:=0; exten_base[null_font]:=0;
    globals.kern_base[null_font] = 0;
    globals.exten_base[null_font] = 0;
    // font_glue[null_font]:=null; font_params[null_font]:=7;
    globals.font_glue[null_font] = null;
    globals.font_params[null_font] = 7.into();
    // param_base[null_font]:=-1;
    globals.param_base[null_font] = -1;
    // for k:=0 to 6 do font_info[k].sc:=0;
    for k in 0..=6 {
        globals.font_info[k][MEMORY_WORD_SC] = scaled::zero();
    }
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use crate::section_0101::scaled;
use crate::section_0101::MEMORY_WORD_SC;
use crate::section_0115::null;
use crate::section_0232::null_font;
use crate::section_0548::internal_font_number;
use crate::section_0549::non_address;
use crate::section_0549::non_char;

use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
