//! @ The inter-element spacing in math formulas depends on an $8\times8$ table that
//! \TeX\ preloads as a 64-digit string. The elements of this string have the
//! following significance:
//! $$\vbox{\halign{#\hfil\cr
//! \.0 means no space;\cr
//! \.1 means a conditional thin space (\.{\\nonscript\\mskip\\thinmuskip});\cr
//! \.2 means a thin space (\.{\\mskip\\thinmuskip});\cr
//! \.3 means a conditional medium space
//!   (\.{\\nonscript\\mskip\\medmuskip});\cr
//! \.4 means a conditional thick space
//!   (\.{\\nonscript\\mskip\\thickmuskip});\cr
//! \.* means an impossible case.\cr}}$$
//! This is all pretty cryptic, but {\sl The \TeX book\/} explains what is
//! supposed to happen, and the string makes it happen.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! A global variable |magic_offset| is computed so that if |a| and |b| are
//! in the range |ord_noad..inner_noad|, then |str_pool[a*8+b+magic_offset]|
//! is the digit for spacing between noad types |a| and |b|.
//!
//! If \PASCAL\ had provided a good way to preload constant arrays, this part of
//! the program would not have been so strange.
//! @:PASCAL}{\PASCAL@>
//
// @d math_spacing=@;@/
// @t\hskip-35pt@>
// "0234000122*4000133**3**344*0400400*000000234000111*1111112341011"
// @t$ \hskip-35pt$@>
pub(crate) const MATH_SPACING: [u8; 64 + 1] =
    *b" 0234000122*4000133**3**344*0400400*000000234000111*1111112341011";
//
// @<Glob...@>=
// @!magic_offset:integer; {used to find inter-element spacing}
//
