//! ` `
// @<Undump the array info for internal font number |k|@>=
pub(crate) macro Undump_the_array_info_for_internal_font_number_k($globals:expr, $k:expr, $lbl_bad_fmt:lifetime) {{
    // begin undump_qqqq(font_check[k]);@/
    undump_qqqq!($globals, $globals.font_check[$k]);
    // undump_int(font_size[k]);
    undump_int!($globals, $globals.font_size[$k], scaled::new_from_inner);
    // undump_int(font_dsize[k]);
    undump_int!($globals, $globals.font_dsize[$k], scaled::new_from_inner);
    // undump(min_halfword)(max_halfword)(font_params[k]);@/
    undump!(
        $globals,
        min_halfword,
        max_halfword,
        $globals.font_params[$k],
        font_index::new,
        $lbl_bad_fmt
    );
    // undump_int(hyphen_char[k]);
    undump_int!($globals, $globals.hyphen_char[$k]);
    // undump_int(skew_char[k]);@/
    undump_int!($globals, $globals.skew_char[$k]);
    // undump(0)(str_ptr)(font_name[k]);
    undump!(
        $globals,
        0,
        $globals.str_ptr.get(),
        $globals.font_name[$k],
        str_number::new,
        $lbl_bad_fmt
    );
    // undump(0)(str_ptr)(font_area[k]);@/
    undump!(
        $globals,
        0,
        $globals.str_ptr.get(),
        $globals.font_area[$k],
        str_number::new,
        $lbl_bad_fmt
    );
    // undump(0)(255)(font_bc[k]);
    undump!(
        $globals,
        0,
        255,
        $globals.font_bc[$k],
        ASCII_code::from_integer,
        $lbl_bad_fmt
    );
    // undump(0)(255)(font_ec[k]);@/
    undump!(
        $globals,
        0,
        255,
        $globals.font_ec[$k],
        ASCII_code::from_integer,
        $lbl_bad_fmt
    );
    // undump_int(char_base[k]);
    undump_int!($globals, $globals.char_base[$k]);
    // undump_int(width_base[k]);
    undump_int!($globals, $globals.width_base[$k]);
    // undump_int(height_base[k]);@/
    undump_int!($globals, $globals.height_base[$k]);
    // undump_int(depth_base[k]);
    undump_int!($globals, $globals.depth_base[$k]);
    // undump_int(italic_base[k]);
    undump_int!($globals, $globals.italic_base[$k]);
    // undump_int(lig_kern_base[k]);@/
    undump_int!($globals, $globals.lig_kern_base[$k]);
    // undump_int(kern_base[k]);
    undump_int!($globals, $globals.kern_base[$k]);
    // undump_int(exten_base[k]);
    undump_int!($globals, $globals.exten_base[$k]);
    // undump_int(param_base[k]);@/
    undump_int!($globals, $globals.param_base[$k]);
    // undump(min_halfword)(lo_mem_max)(font_glue[k]);@/
    undump!(
        $globals,
        min_halfword,
        $globals.lo_mem_max,
        $globals.font_glue[$k],
        core::convert::identity,
        $lbl_bad_fmt
    );
    // undump(0)(fmem_ptr-1)(bchar_label[k]);
    undump!(
        $globals,
        0,
        $globals.fmem_ptr.get() - 1,
        $globals.bchar_label[$k],
        font_index::new,
        $lbl_bad_fmt
    );
    // undump(min_quarterword)(non_char)(font_bchar[k]);
    undump!(
        $globals,
        min_quarterword,
        non_char,
        $globals.font_bchar[$k],
        core::convert::identity,
        $lbl_bad_fmt
    );
    // undump(min_quarterword)(non_char)(font_false_bchar[k]);
    undump!(
        $globals,
        min_quarterword,
        non_char,
        $globals.font_false_bchar[$k],
        core::convert::identity,
        $lbl_bad_fmt
    );
    // end
    use crate::section_0018::ASCII_code;
    use crate::section_0038::str_number;
    use crate::section_0101::scaled;
    use crate::section_0110::max_halfword;
    use crate::section_0110::min_halfword;
    use crate::section_0110::min_quarterword;
    use crate::section_0548::font_index;
    use crate::section_0549::non_char;
    use crate::section_1306::undump;
    use crate::section_1306::undump_int;
    use crate::section_1306::undump_qqqq;
}}
