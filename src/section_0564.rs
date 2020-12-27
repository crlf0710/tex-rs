//! @ Note: A malformed \.{TFM} file might be shorter than it claims to be;
//! thus |eof(tfm_file)| might be true when |read_font_info| refers to
//! |tfm_file^| or when it says |get(tfm_file)|. If such circumstances
//! cause system error messages, you will have to defeat them somehow,
//! for example by defining |fget| to be `\ignorespaces|begin get(tfm_file);|
//! |if eof(tfm_file) then abort; end|\unskip'.
//! @^system dependencies@>
//!
//! @d fget==get(tfm_file)
//! @d fbyte==tfm_file^
//! @d read_sixteen(#)==begin #:=fbyte;
//!   if #>127 then abort;
//!   fget; #:=#*@'400+fbyte;
//!   end
//! @d store_four_quarters(#)==begin fget; a:=fbyte; qw.b0:=qi(a);
//!   fget; b:=fbyte; qw.b1:=qi(b);
//!   fget; c:=fbyte; qw.b2:=qi(c);
//!   fget; d:=fbyte; qw.b3:=qi(d);
//!   #:=qw;
//!   end
//!
