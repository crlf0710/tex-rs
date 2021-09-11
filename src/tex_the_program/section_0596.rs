//! @ Initially the buffer is all in one piece; we will output half of it only
//! after it first fills up.
//
// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0596($globals:expr) {{
    let globals = &mut *$globals;
    // half_buf:=dvi_buf_size div 2; dvi_limit:=dvi_buf_size; dvi_ptr:=0;
    globals.half_buf = (dvi_buf_size / 2).into();
    globals.dvi_limit = dvi_buf_size.into();
    globals.dvi_ptr = 0.into();
    // dvi_offset:=0; dvi_gone:=0;
    globals.dvi_offset = 0;
    globals.dvi_gone = 0;

    use crate::section_0011::dvi_buf_size;
}}
