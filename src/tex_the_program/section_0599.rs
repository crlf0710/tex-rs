//! @ Here is how we clean out the buffer when \TeX\ is all through; |dvi_ptr|
//! will be a multiple of~4.
//
// @<Empty the last bytes out of |dvi_buf|@>=
pub(crate) macro Empty_the_last_bytes_out_of_dvi_buf($globals:expr) {{
    // if dvi_limit=half_buf then write_dvi(half_buf,dvi_buf_size-1);
    if $globals.dvi_limit == $globals.half_buf {
        write_dvi($globals, $globals.half_buf, (dvi_buf_size - 1).into());
    }
    // if dvi_ptr>0 then write_dvi(0,dvi_ptr-1)
    if $globals.dvi_ptr > 0 {
        write_dvi($globals, 0.into(), $globals.dvi_ptr - 1);
    }
    use crate::section_0011::dvi_buf_size;
    use crate::section_0597::write_dvi;
}}
