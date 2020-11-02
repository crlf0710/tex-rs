//! @ The ASCII code is ``standard'' only to a certain extent, since many
//! computer installations have found it advantageous to have ready access
//! to more than 94 printing characters. Appendix~C of {\sl The \TeX book\/}
//! gives a complete specification of the intended correspondence between
//! characters and \TeX's internal representation.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! If \TeX\ is being used
//! on a garden-variety \PASCAL\ for which only standard ASCII
//! codes will appear in the input and output files, it doesn't really matter
//! what codes are specified in |xchr[0..@'37]|, but the safest policy is to
//! blank everything out by using the code shown below.
//!
//! However, other settings of |xchr| will make \TeX\ more friendly on
//! computers that have an extended character set, so that users can type things
//! like `\.^^Z' instead of `\.{\\ne}'. People with extended character sets can
//! assign codes arbitrarily, giving an |xchr| equivalent to whatever
//! characters the users of \TeX\ are allowed to have in their input files.
//! It is best to make the codes correspond to the intended interpretations as
//! shown in Appendix~C whenever possible; but this is not necessary. For
//! example, in countries with an alphabet of more than 26 letters, it is
//! usually best to map the additional letters into codes less than~@'40.
//! To get the most ``permissive'' character set, change |' '| on the
//! right of these assignment statements to |chr(i)|.
//! @^character set dependencies@>
//! @^system dependencies@>
//!
//! @<Set init...@>=
//! for i:=0 to @'37 do xchr[i]:=' ';
//! for i:=@'177 to @'377 do xchr[i]:=' ';
//!
