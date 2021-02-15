//! @ Here is how we clean out the buffer when \TeX\ is all through; |dvi_ptr|
//! will be a multiple of~4.
//!
//! @<Empty the last bytes out of |dvi_buf|@>=
//! if dvi_limit=half_buf then write_dvi(half_buf,dvi_buf_size-1);
//! if dvi_ptr>0 then write_dvi(0,dvi_ptr-1)
