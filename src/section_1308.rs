//! @ Sections of a \.{WEB} program that are ``commented out'' still contribute
//! strings to the string pool; therefore \.{INITEX} and \TeX\ will have
//! the same strings. (And it is, of course, a good thing that they do.)
//! @.WEB@>
//! @^string pool@>
//!
//! @<Undump constants for consistency check@>=
//! x:=fmt_file^.int;
//! if x<>@$ then goto bad_fmt; {check that strings are the same}
//! undump_int(x);
//! if x<>mem_bot then goto bad_fmt;
//! undump_int(x);
//! if x<>mem_top then goto bad_fmt;
//! undump_int(x);
//! if x<>eqtb_size then goto bad_fmt;
//! undump_int(x);
//! if x<>hash_prime then goto bad_fmt;
//! undump_int(x);
//! if x<>hyph_size then goto bad_fmt
//!
