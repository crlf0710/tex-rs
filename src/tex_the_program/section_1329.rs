//! ` `
// @<Close the format file@>=
pub(crate) macro Close_the_format_file($globals:expr) {{
    // w_close(fmt_file)
    w_close(&mut $globals.fmt_file);
    use crate::section_0028::w_close;
}}
