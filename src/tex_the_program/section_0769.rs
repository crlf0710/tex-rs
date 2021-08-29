//! @ We have mentioned that alignrecords contain no |height| or |depth| fields.
//! Their |glue_sign| and |glue_order| are pre-empted as well, since it
//! is necessary to store information about what to do when a template ends.
//! This information is called the |extra_info| field.
//
// @d u_part(#)==mem[#+height_offset].int {pointer to \<u_j> token list}
/// pointer to `<u_j>` token list
macro_rules! u_part {
    ($globals:expr, $val:expr) => {
        $globals.mem[$val as u32 + crate::section_0135::height_offset as u32]
            [crate::section_0113::MEMORY_WORD_INT]
    };
}
// @d v_part(#)==mem[#+depth_offset].int {pointer to \<v_j> token list}
/// pointer to `<v_j>` token list
macro_rules! v_part {
    ($globals:expr, $val:expr) => {
        $globals.mem[$val as u32 + crate::section_0135::depth_offset as u32]
            [crate::section_0113::MEMORY_WORD_INT]
    };
}
// @d extra_info(#)==info(#+list_offset) {info to remember during template}
/// info to remember during template
macro_rules! extra_info {
    ($globals:expr, $val:expr) => {
        info_inner!(
            $globals,
            $val + crate::section_0135::list_offset as crate::section_0115::pointer
        )
    };
}
