//! ` `
// @<Read and check...@>=
macro_rules! Read_and_check_the_font_data__abort_if_the_TFM_file_is_malformed__if_there_s_no_room_for_this_font__say_so_and_goto_done__otherwise_incr_font_ptr_and_goto_done {
    ($globals:expr, $s:expr, $nom:expr, $aire:expr, $file_opened:expr, $lbl_bad_tfm:lifetime) => {{
        /// sizes of subfiles
        let (mut lf, mut lh, mut bc, mut ec, mut nw, mut nh, mut nd, mut ni, mut nl, mut nk, mut ne, mut np): 
            (halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword, halfword);

        /// the new font's number
        let f: internal_font_number;

        /// the design size or the "at" size
        let mut z: scaled;

        // @<Open |tfm_file| for input@>;
        Open_tfm_file_for_input!($globals, $nom, $aire, $file_opened, $lbl_bad_tfm);
        // @<Read the {\.{TFM}} size fields@>;
        Read_the_TFM_size_fields!($globals, lf, lh, bc, ec, nw, nh, nd, ni, nl, nk, ne, np, $lbl_bad_tfm);
        // @<Use size fields to allocate font information@>;
        Use_size_fields_to_allocate_font_information!($globals, f, lf, lh, bc, ec, nw, nh, nd, ni, nl, nk, ne, np);
        // @<Read the {\.{TFM}} header@>;
        Read_the_TFM_header!($globals, f, $s, z, lh, $lbl_bad_tfm);
        // @<Read character data@>;
        Read_character_data!($globals, f, nw, nh, nd, ni, nl, ne, $lbl_bad_tfm);
        // @<Read box dimensions@>;
        Read_box_dimensions!($globals, f, z, $lbl_bad_tfm);
        // @<Read ligature/kern program@>;
        // @<Read extensible character recipes@>;
        // @<Read font parameters@>;
        // @<Make final adjustments and |goto done|@>
        todo!("finish tfm loading");
        use crate::section_0113::halfword;
    }}
}
