//! @ @<Set init...@>=
//! for k:=font_base to font_max do font_used[k]:=false;
//!
//! @ \TeX\ always knows at least one font, namely the null font. It has no
//! characters, and its seven parameters are all equal to zero.
//!
//! @<Initialize table...@>=
//! font_ptr:=null_font; fmem_ptr:=7;
//! font_name[null_font]:="nullfont"; font_area[null_font]:="";
//! hyphen_char[null_font]:="-"; skew_char[null_font]:=-1;
//! bchar_label[null_font]:=non_address;
//! font_bchar[null_font]:=non_char; font_false_bchar[null_font]:=non_char;
//! font_bc[null_font]:=1; font_ec[null_font]:=0;
//! font_size[null_font]:=0; font_dsize[null_font]:=0;
//! char_base[null_font]:=0; width_base[null_font]:=0;
//! height_base[null_font]:=0; depth_base[null_font]:=0;
//! italic_base[null_font]:=0; lig_kern_base[null_font]:=0;
//! kern_base[null_font]:=0; exten_base[null_font]:=0;
//! font_glue[null_font]:=null; font_params[null_font]:=7;
//! param_base[null_font]:=-1;
//! for k:=0 to 6 do font_info[k].sc:=0;
//!
//! @ @<Put each...@>=
//! primitive("nullfont",set_font,null_font);
//! @!@:null_font_}{\.{\\nullfont} primitive@>
//! text(frozen_null_font):="nullfont"; eqtb[frozen_null_font]:=eqtb[cur_val];
//!
