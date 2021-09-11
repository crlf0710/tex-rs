//! ` `
// @<Undump a couple more things and the closing check word@>=
pub(crate) macro Undump_a_couple_more_things_and_the_closing_check_word($globals:expr, $lbl_bad_fmt:lifetime) {{
    // undump(batch_mode)(error_stop_mode)(interaction);
    undump!(
        $globals,
        batch_mode,
        error_stop_mode,
        $globals.interaction,
        u8_from_m_to_n::new,
        $lbl_bad_fmt
    );
    // undump(0)(str_ptr)(format_ident);
    undump!(
        $globals,
        0,
        $globals.str_ptr.get(),
        $globals.format_ident,
        str_number::new,
        $lbl_bad_fmt
    );
    let x;
    // undump_int(x);
    undump_int!($globals, x);
    // if (x<>69069)or eof(fmt_file) then goto bad_fmt
    if x != 69069 || eof(&mut $globals.fmt_file) {
        crate::goto_forward_label!($lbl_bad_fmt);
    }
    use crate::pascal::eof;
    use crate::pascal::u8_from_m_to_n;
    use crate::section_0038::str_number;
    use crate::section_0073::batch_mode;
    use crate::section_0073::error_stop_mode;
    use crate::section_1306::undump;
    use crate::section_1306::undump_int;
}}
