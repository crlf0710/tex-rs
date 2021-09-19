//! ` `

// @<The x-height for |cur_font|@>=x_height(cur_font)
pub(crate) macro The_x_height_for_cur_font($globals:expr) {
    crate::section_0558::x_height!(
        $globals,
        crate::section_0548::internal_font_number::from(crate::section_0230::cur_font!($globals))
    )
}
