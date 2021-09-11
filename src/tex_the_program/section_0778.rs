//! ` `
// @<Append the current tabskip glue to the preamble list@>=
pub(crate) macro Append_the_current_tabskip_glue_to_the_preamble_list($globals:expr) {{
    // link(cur_align):=new_param_glue(tab_skip_code);
    link!($globals, $globals.cur_align) = new_param_glue($globals, tab_skip_code.into())?;
    // cur_align:=link(cur_align)
    $globals.cur_align = link!($globals, $globals.cur_align);
    use crate::section_0118::link;
    use crate::section_0152::new_param_glue;
    use crate::section_0224::tab_skip_code;
}}
