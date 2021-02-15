//! @ Initially the buffer is all in one piece; we will output half of it only
//! after it first fills up.
//!
//! @<Set init...@>=
//! half_buf:=dvi_buf_size div 2; dvi_limit:=dvi_buf_size; dvi_ptr:=0;
//! dvi_offset:=0; dvi_gone:=0;
//!
