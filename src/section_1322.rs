//! ` `
// @<Dump the array info for internal font number |k|@>=
macro_rules! Dump_the_array_info_for_internal_font_number_k {
    ($globals:expr, $k:expr) => {{
        // begin dump_qqqq(font_check[k]);
        dump_qqqq!($globals, $globals.font_check[$k]);
        // dump_int(font_size[k]);
        dump_int!($globals, $globals.font_size[$k].inner());
        // dump_int(font_dsize[k]);
        dump_int!($globals, $globals.font_dsize[$k].inner());
        // dump_int(font_params[k]);@/
        dump_int!($globals, $globals.font_params[$k].get() as _);
        // dump_int(hyphen_char[k]);
        dump_int!($globals, $globals.hyphen_char[$k]);
        // dump_int(skew_char[k]);@/
        dump_int!($globals, $globals.skew_char[$k]);
        // dump_int(font_name[k]);
        dump_int!($globals, $globals.font_name[$k].get() as _);
        // dump_int(font_area[k]);@/
        dump_int!($globals, $globals.font_area[$k].get() as _);
        // dump_int(font_bc[k]);
        dump_int!($globals, $globals.font_bc[$k].numeric_value() as _);
        // dump_int(font_ec[k]);@/
        dump_int!($globals, $globals.font_ec[$k].numeric_value() as _);
        // dump_int(char_base[k]);
        dump_int!($globals, $globals.char_base[$k]);
        // dump_int(width_base[k]);
        dump_int!($globals, $globals.width_base[$k]);
        // dump_int(height_base[k]);@/
        dump_int!($globals, $globals.height_base[$k]);
        // dump_int(depth_base[k]);
        dump_int!($globals, $globals.depth_base[$k]);
        // dump_int(italic_base[k]);
        dump_int!($globals, $globals.italic_base[$k]);
        // dump_int(lig_kern_base[k]);@/
        dump_int!($globals, $globals.lig_kern_base[$k]);
        // dump_int(kern_base[k]);
        dump_int!($globals, $globals.kern_base[$k]);
        // dump_int(exten_base[k]);
        dump_int!($globals, $globals.exten_base[$k]);
        // dump_int(param_base[k]);@/
        dump_int!($globals, $globals.param_base[$k]);
        // dump_int(font_glue[k]);@/
        dump_int!($globals, $globals.font_glue[$k] as _);
        // dump_int(bchar_label[k]);
        dump_int!($globals, $globals.bchar_label[$k].get() as _);
        // dump_int(font_bchar[k]);
        dump_int!($globals, $globals.font_bchar[$k] as _);
        // dump_int(font_false_bchar[k]);@/
        dump_int!($globals, $globals.font_false_bchar[$k] as _);
        // print_nl("\font"); print_esc(font_id_text(k)); print_char("=");
        print_nl($globals, strpool_str!("\\font"));
        print_esc($globals, str_number::new(font_id_text!($globals, $k) as _));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'='),
        );
        // print_file_name(font_name[k],font_area[k],"");
        print_file_name(
            $globals,
            $globals.font_name[$k].get() as _,
            $globals.font_area[$k].get() as _,
            strpool_str!("").get() as _,
        );
        // if font_size[k]<>font_dsize[k] then
        if $globals.font_size[$k] != $globals.font_dsize[$k] {
            // begin print(" at "); print_scaled(font_size[k]); print("pt");
            print($globals, strpool_str!(" at ").get() as _);
            print_scaled($globals, $globals.font_size[$k]);
            print($globals, strpool_str!("pt").get() as _);
            // end;
        }
        // end
        use crate::section_0038::str_number;
        use crate::section_0062::print_nl;
        use crate::section_0063::print_esc;
        use crate::section_0103::print_scaled;
        use crate::section_0518::print_file_name;
    }};
}
