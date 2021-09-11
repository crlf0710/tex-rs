//! ` `
// @<Finish the extensions@>=
pub(crate) macro Finish_the_extensions($globals:expr) {{
    // for k:=0 to 15 do if write_open[k] then a_close(write_file[k])
    for k in 0..=15 {
        if $globals.write_open[k] {
            a_close(&mut $globals.write_file[k])
        }
    }
    use crate::section_0028::a_close;
}}
