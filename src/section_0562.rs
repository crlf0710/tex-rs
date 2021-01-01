//! ` `
// @<Read and check...@>=
macro_rules! Read_and_check_the_font_data__abort_if_the_TFM_file_is_malformed__if_there_s_no_room_for_this_font__say_so_and_goto_done__otherwise_incr_font_ptr_and_goto_done {
    ($globals:expr, $s:expr, $nom:expr, $aire:expr, $file_opened:expr, $lbl_bad_tfm:lifetime) => {{
        /// sizes of subfiles
        #[rustfmt::skip]
        let (mut lf, mut lh, mut bc, mut ec, mut nw, mut nh, mut nd, mut ni, mut nl, mut nk, mut ne, mut np):
            (halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword);

        /// the new font's number
        let f: internal_font_number;

        /// the design size or the "at" size
        let mut z: scaled;

        /// auxiliary quantities used in fixed-point multiplication
        let (mut alpha, beta): (integer, u8);

        /// left boundary start location, or infinity
        let mut bch_label: integer;

        /// right boundary character, or `non_char`
        let mut bchar: ASCII_code_or_non_char;

        // @<Open |tfm_file| for input@>;
        Open_tfm_file_for_input!($globals, $nom, $aire, $file_opened, $lbl_bad_tfm);
        // @<Read the {\.{TFM}} size fields@>;
        #[rustfmt::skip]
        Read_the_TFM_size_fields!(
            $globals, lf, lh, bc, ec, nw, nh, nd, ni, nl, nk, ne, np, $lbl_bad_tfm
        );
        // @<Use size fields to allocate font information@>;
        Use_size_fields_to_allocate_font_information!(
            $globals, f, lf, lh, bc, ec, nw, nh, nd, ni, nl, nk, ne, np
        );
        // @<Read the {\.{TFM}} header@>;
        Read_the_TFM_header!($globals, f, $s, z, lh, $lbl_bad_tfm);
        // @<Read character data@>;
        Read_character_data!($globals, f, nw, nh, nd, ni, nl, ne, $lbl_bad_tfm);
        // @<Read box dimensions@>;
        Read_box_dimensions!($globals, f, z, alpha, beta, $lbl_bad_tfm);
        // @<Read ligature/kern program@>;
        Read_ligature_kern_program!($globals, f, z, bch_label, bchar, alpha, beta, bc, ec, nl, nk, $lbl_bad_tfm);
        // @<Read extensible character recipes@>;
        Read_extensible_character_recipes!($globals, f, bc, ec, $lbl_bad_tfm);
        // @<Read font parameters@>;
        // @<Make final adjustments and |goto done|@>
        todo!("finish tfm loading");

        use crate::pascal::integer;
        use crate::section_0113::halfword;
        use crate::section_0907::ASCII_code_or_non_char;
    }};
}
