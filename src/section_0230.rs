//! @ Region 4 of |eqtb| contains the local quantities defined here. The
//! bulk of this region is taken up by five tables that are indexed by eight-bit
//! characters; these tables are important to both the syntactic and semantic
//! portions of \TeX. There are also a bunch of special things like font and
//! token parameters, as well as the tables of \.{\\toks} and \.{\\box}
//! registers.

// @d par_shape_loc=local_base {specifies paragraph shape}
// @d output_routine_loc=local_base+1 {points to token list for \.{\\output}}
// @d every_par_loc=local_base+2 {points to token list for \.{\\everypar}}
// @d every_math_loc=local_base+3 {points to token list for \.{\\everymath}}
// @d every_display_loc=local_base+4 {points to token list for \.{\\everydisplay}}
// @d every_hbox_loc=local_base+5 {points to token list for \.{\\everyhbox}}
// @d every_vbox_loc=local_base+6 {points to token list for \.{\\everyvbox}}
// @d every_job_loc=local_base+7 {points to token list for \.{\\everyjob}}
// @d every_cr_loc=local_base+8 {points to token list for \.{\\everycr}}
// @d err_help_loc=local_base+9 {points to token list for \.{\\errhelp}}
// @d toks_base=local_base+10 {table of 256 token list registers}
/// table of 256 token list registers
pub(crate) type toks_base_TYPENUM = typenum::op!(local_base_TYPENUM + U10);
pub(crate) const toks_base: word = toks_base_TYPENUM::U32;
// @d box_base=toks_base+256 {table of 256 box registers}
/// table of 256 box registers
pub(crate) type box_base_TYPENUM = typenum::op!(toks_base_TYPENUM + U256);
pub(crate) const box_base: word = box_base_TYPENUM::U32;
// @d cur_font_loc=box_base+256 {internal font number outside math mode}
/// internal font number outside math mode
pub(crate) type cur_font_loc_TYPENUM = typenum::op!(box_base_TYPENUM + U256);
pub(crate) const cur_font_loc: word = cur_font_loc_TYPENUM::U32;
// @d math_font_base=cur_font_loc+1 {table of 48 math font numbers}
/// table of 48 math font numbers
pub(crate) type math_font_base_TYPENUM = typenum::op!(cur_font_loc_TYPENUM + U1);
pub(crate) const math_font_base: word = math_font_base_TYPENUM::U32;
// @d cat_code_base=math_font_base+48
//   {table of 256 command codes (the ``catcodes'')}
/// table of 256 command codes (the "catcodes")
pub(crate) type cat_code_base_TYPENUM = typenum::op!(math_font_base_TYPENUM + U48);
pub(crate) const cat_code_base: word = cat_code_base_TYPENUM::U32;
// @d lc_code_base=cat_code_base+256 {table of 256 lowercase mappings}
/// table of 256 lowercase mappings
pub(crate) type lc_code_base_TYPENUM = typenum::op!(cat_code_base_TYPENUM + U256);
pub(crate) const lc_code_base: word = lc_code_base_TYPENUM::U32;
// @d uc_code_base=lc_code_base+256 {table of 256 uppercase mappings}
/// table of 256 uppercase mappings
pub(crate) type uc_code_base_TYPENUM = typenum::op!(lc_code_base_TYPENUM + U256);
pub(crate) const uc_code_base: word = uc_code_base_TYPENUM::U32;
// @d sf_code_base=uc_code_base+256 {table of 256 spacefactor mappings}
/// table of 256 spacefactor mappings
pub(crate) type sf_code_base_TYPENUM = typenum::op!(uc_code_base_TYPENUM + U256);
pub(crate) const sf_code_base: word = sf_code_base_TYPENUM::U32;
// @d math_code_base=sf_code_base+256 {table of 256 math mode mappings}
/// table of 256 math mode mappings
pub(crate) type math_code_base_TYPENUM = typenum::op!(sf_code_base_TYPENUM + U256);
pub(crate) const math_code_base: word = math_code_base_TYPENUM::U32;
// @d int_base=math_code_base+256 {beginning of region 5}
/// beginning of region 5
pub(crate) type int_base_TYPENUM = typenum::op!(math_code_base_TYPENUM + U256);
pub(crate) const int_base: word = int_base_TYPENUM::U32;
// @#
// @d par_shape_ptr==equiv(par_shape_loc)
// @d output_routine==equiv(output_routine_loc)
// @d every_par==equiv(every_par_loc)
// @d every_math==equiv(every_math_loc)
// @d every_display==equiv(every_display_loc)
// @d every_hbox==equiv(every_hbox_loc)
// @d every_vbox==equiv(every_vbox_loc)
// @d every_job==equiv(every_job_loc)
// @d every_cr==equiv(every_cr_loc)
// @d err_help==equiv(err_help_loc)
// @d toks(#)==equiv(toks_base+#)
// @d box(#)==equiv(box_base+#)
// @d cur_font==equiv(cur_font_loc)
// @d fam_fnt(#)==equiv(math_font_base+#)
// @d cat_code(#)==equiv(cat_code_base+#)
macro_rules! cat_code {
    ($globals:expr, $val:expr) => {{
        use crate::section_0230::{cat_code_base, index_offset_with_ASCII_code};
        equiv!($globals, index_offset_with_ASCII_code(cat_code_base, $val))
    }};
}
// @d lc_code(#)==equiv(lc_code_base+#)
// @d uc_code(#)==equiv(uc_code_base+#)
// @d sf_code(#)==equiv(sf_code_base+#)
// @d math_code(#)==equiv(math_code_base+#)
//   {Note: |math_code(c)| is the true math code plus |min_halfword|}
//
// @<Put each...@>=
// primitive("output",assign_toks,output_routine_loc);
// @!@:output_}{\.{\\output} primitive@>
// primitive("everypar",assign_toks,every_par_loc);
// @!@:every_par_}{\.{\\everypar} primitive@>
// primitive("everymath",assign_toks,every_math_loc);
// @!@:every_math_}{\.{\\everymath} primitive@>
// primitive("everydisplay",assign_toks,every_display_loc);
// @!@:every_display_}{\.{\\everydisplay} primitive@>
// primitive("everyhbox",assign_toks,every_hbox_loc);
// @!@:every_hbox_}{\.{\\everyhbox} primitive@>
// primitive("everyvbox",assign_toks,every_vbox_loc);
// @!@:every_vbox_}{\.{\\everyvbox} primitive@>
// primitive("everyjob",assign_toks,every_job_loc);
// @!@:every_job_}{\.{\\everyjob} primitive@>
// primitive("everycr",assign_toks,every_cr_loc);
// @!@:every_cr_}{\.{\\everycr} primitive@>
// primitive("errhelp",assign_toks,err_help_loc);
// @!@:err_help_}{\.{\\errhelp} primitive@>

use crate::pascal::word;
use crate::section_0224::local_base_TYPENUM;

#[allow(non_snake_case)]
pub(crate) fn index_offset_with_ASCII_code(initial: word, offset: ASCII_code) -> word {
    #[cfg(not(feature = "unicode_support"))]
    {
        initial + offset.0
    }
    #[cfg(feature = "unicode_support")]
    {
        if offset.0 < 256 {
            initial + offset.0
        } else {
            unimplemented!();
        }
    }
}

use crate::section_0018::ASCII_code;
use typenum::Unsigned;
use typenum::{U1, U10, U256, U48};
