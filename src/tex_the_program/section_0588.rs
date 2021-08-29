//! @ Font definitions for a given font number |k| contain further parameters
//! $$\hbox{|c[4]| |s[4]| |d[4]| |a[1]| |l[1]| |n[a+l]|.}$$
//! The four-byte value |c| is the check sum that \TeX\ found in the \.{TFM}
//! file for this font; |c| should match the check sum of the font found by
//! programs that read this \.{DVI} file.
//! @^check sum@>
//!
//! Parameter |s| contains a fixed-point scale factor that is applied to
//! the character widths in font |k|; font dimensions in \.{TFM} files and
//! other font files are relative to this quantity, which is called the
//! ``at size'' elsewhere in this documentation. The value of |s| is
//! always positive and less than $2^{27}$. It is given in the same units
//! as the other \.{DVI} dimensions, i.e., in sp when \TeX82 has made the
//! file.  Parameter |d| is similar to |s|; it is the ``design size,'' and
//! (like~|s|) it is given in \.{DVI} units. Thus, font |k| is to be used
//! at $|mag|\cdot s/1000d$ times its normal size.
//!
//! The remaining part of a font definition gives the external name of the font,
//! which is an ASCII string of length |a+l|. The number |a| is the length
//! of the ``area'' or directory, and |l| is the length of the font name itself;
//! the standard local system font area is supposed to be used when |a=0|.
//! The |n| field contains the area in its first |a| bytes.
//!
//! Font definitions must appear before the first use of a particular font number.
//! Once font |k| is defined, it must not be defined again; however, we
//! shall see below that font definitions appear in the postamble as well as
//! in the pages, so in this sense each font number is defined exactly twice,
//! if at all. Like |nop| commands, font definitions can
//! appear before the first |bop|, or between an |eop| and a |bop|.
//!
