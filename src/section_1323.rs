//! @ @<Undump the array info for internal font number |k|@>=
//! begin undump_qqqq(font_check[k]);@/
//! undump_int(font_size[k]);
//! undump_int(font_dsize[k]);
//! undump(min_halfword)(max_halfword)(font_params[k]);@/
//! undump_int(hyphen_char[k]);
//! undump_int(skew_char[k]);@/
//! undump(0)(str_ptr)(font_name[k]);
//! undump(0)(str_ptr)(font_area[k]);@/
//! undump(0)(255)(font_bc[k]);
//! undump(0)(255)(font_ec[k]);@/
//! undump_int(char_base[k]);
//! undump_int(width_base[k]);
//! undump_int(height_base[k]);@/
//! undump_int(depth_base[k]);
//! undump_int(italic_base[k]);
//! undump_int(lig_kern_base[k]);@/
//! undump_int(kern_base[k]);
//! undump_int(exten_base[k]);
//! undump_int(param_base[k]);@/
//! undump(min_halfword)(lo_mem_max)(font_glue[k]);@/
//! undump(0)(fmem_ptr-1)(bchar_label[k]);
//! undump(min_quarterword)(non_char)(font_bchar[k]);
//! undump(min_quarterword)(non_char)(font_false_bchar[k]);
//! end
//!
