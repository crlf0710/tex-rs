//! @ The inverse macros are slightly more complicated, since we need to check
//! the range of the values we are reading in. We say `|undump(a)(b)(x)|' to
//! read an integer value |x| that is supposed to be in the range |a<=x<=b|.
//! System error messages should be suppressed when undumping.
//! @^system dependencies@>
//!
//! @d undump_wd(#)==begin get(fmt_file); #:=fmt_file^;@+end
//! @d undump_int(#)==begin get(fmt_file); #:=fmt_file^.int;@+end
//! @d undump_hh(#)==begin get(fmt_file); #:=fmt_file^.hh;@+end
//! @d undump_qqqq(#)==begin get(fmt_file); #:=fmt_file^.qqqq;@+end
//! @d undump_end_end(#)==#:=x;@+end
//! @d undump_end(#)==(x>#) then goto bad_fmt@+else undump_end_end
//! @d undump(#)==begin undump_int(x); if (x<#) or undump_end
//! @d undump_size_end_end(#)==too_small(#)@+else undump_end_end
//! @d undump_size_end(#)==if x># then undump_size_end_end
//! @d undump_size(#)==begin undump_int(x);
//!   if x<# then goto bad_fmt; undump_size_end
//!
