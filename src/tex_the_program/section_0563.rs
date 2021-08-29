//! ` `

//! @<Open |tfm_file| for input@>=
macro_rules! Open_tfm_file_for_input {
    ($globals:expr, $nom:expr, $aire:expr, $file_opened:expr, $lbl_bad_tfm:lifetime) => {{
        // file_opened:=false;
        $file_opened = false;
        // if aire="" then pack_file_name(nom,TEX_font_area,".tfm")
        if $aire == strpool_str!("") {
            pack_file_name($globals, $nom, TEX_font_area!(), strpool_str!(".tfm"));
        }
        // else pack_file_name(nom,aire,".tfm");
        else {
            pack_file_name($globals, $nom, $aire, strpool_str!(".tfm"));
        }
        // if not b_open_in(tfm_file) then abort;
        if !b_open_in(make_globals_filename_view!($globals), &mut $globals.tfm_file) {
            goto_forward_label!($lbl_bad_tfm);
        }
        // file_opened:=true
        $file_opened = true;

        use crate::section_0004::TeXGlobalsFilenameView;
        use crate::section_0027::b_open_in;
        use crate::section_0519::pack_file_name;
    }}
}
