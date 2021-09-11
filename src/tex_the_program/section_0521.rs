//! ` `
// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0521($globals:expr) {{
    let globals = &mut *$globals;
    // TEX_format_default:='TeXformats:plain.fmt';
    for (idx, ch) in " TeXformats:plain.fmt".chars().enumerate().skip(1) {
        globals.TEX_format_default[idx as u16] = xchr(ASCII_code::from(ch as u32 as integer));
    }
    // @.TeXformats@>
    // @.plain@>
    // @^system dependencies@>
    use crate::pascal::integer;
    use crate::section_0018::ASCII_code;
    use crate::section_0020::xchr;
}}
