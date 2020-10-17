//! @ The last part of the postamble, following the |post_post| byte that
//! signifies the end of the font definitions, contains |q|, a pointer to the
//! |post| command that started the postamble.  An identification byte, |i|,
//! comes next; this currently equals~2, as in the preamble.
//!
//! The |i| byte is followed by four or more bytes that are all equal to
//! the decimal number 223 (i.e., @'337 in octal). \TeX\ puts out four to seven of
//! these trailing bytes, until the total length of the file is a multiple of
//! four bytes, since this works out best on machines that pack four bytes per
//! word; but any number of 223's is allowed, as long as there are at least four
//! of them. In effect, 223 is a sort of signature that is added at the very end.
//! @^Fuchs, David Raymond@>
//!
//! This curious way to finish off a \.{DVI} file makes it feasible for
//! \.{DVI}-reading programs to find the postamble first, on most computers,
//! even though \TeX\ wants to write the postamble last. Most operating
//! systems permit random access to individual words or bytes of a file, so
//! the \.{DVI} reader can start at the end and skip backwards over the 223's
//! until finding the identification byte. Then it can back up four bytes, read
//! |q|, and move to byte |q| of the file. This byte should, of course,
//! contain the value 248 (|post|); now the postamble can be read, so the
//! \.{DVI} reader can discover all the information needed for typesetting the
//! pages. Note that it is also possible to skip through the \.{DVI} file at
//! reasonably high speed to locate a particular page, if that proves
//! desirable. This saves a lot of time, since \.{DVI} files used in production
//! jobs tend to be large.
//!
//! Unfortunately, however, standard \PASCAL\ does not include the ability to
//! @^system dependencies@>
//! access a random position in a file, or even to determine the length of a file.
//! Almost all systems nowadays provide the necessary capabilities, so \.{DVI}
//! format has been designed to work most efficiently with modern operating systems.
//! But if \.{DVI} files have to be processed under the restrictions of standard
//! \PASCAL, one can simply read them from front to back, since the necessary
//! header information is present in the preamble and in the font definitions.
//! (The |l| and |u| and |s| and |t| parameters, which appear only in the
//! postamble, are ``frills'' that are handy but not absolutely necessary.)
//!
