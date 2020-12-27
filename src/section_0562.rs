//! ` `
// @<Read and check...@>=
macro_rules! Read_and_check_the_font_data__abort_if_the_TFM_file_is_malformed__if_there_s_no_room_for_this_font__say_so_and_goto_done__otherwise_incr_font_ptr_and_goto_done {
    ($globals:expr, $nom:expr, $aire:expr, $file_opened:expr, $lbl_bad_tfm:lifetime) => {{
        // @<Open |tfm_file| for input@>;
        Open_tfm_file_for_input!($globals, $nom, $aire, $file_opened, $lbl_bad_tfm);
        // @<Read the {\.{TFM}} size fields@>;
        Read_the_TFM_size_fields!($globals);
        // @<Use size fields to allocate font information@>;
        // @<Read the {\.{TFM}} header@>;
        // @<Read character data@>;
        // @<Read box dimensions@>;
        // @<Read ligature/kern program@>;
        // @<Read extensible character recipes@>;
        // @<Read font parameters@>;
        // @<Make final adjustments and |goto done|@>
    }}
}
