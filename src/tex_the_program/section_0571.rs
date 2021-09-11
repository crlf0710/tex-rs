//! @ A |fix_word| whose four bytes are $(a,b,c,d)$ from left to right represents
//! the number
//! $$x=\left\{\vcenter{\halign{$#$,\hfil\qquad&if $#$\hfil\cr
//! b\cdot2^{-4}+c\cdot2^{-12}+d\cdot2^{-20}&a=0;\cr
//! -16+b\cdot2^{-4}+c\cdot2^{-12}+d\cdot2^{-20}&a=255.\cr}}\right.$$
//! (No other choices of |a| are allowed, since the magnitude of a number in
//! design-size units must be less than 16.)  We want to multiply this
//! quantity by the integer~|z|, which is known to be less than $2^{27}$.
//! If $|z|<2^{23}$, the individual multiplications $b\cdot z$,
//! $c\cdot z$, $d\cdot z$ cannot overflow; otherwise we will divide |z| by 2,
//! 4, 8, or 16, to obtain a multiplier less than $2^{23}$, and we can
//! compensate for this later. If |z| has thereby been replaced by
//! $|z|^\prime=|z|/2^e$, let $\beta=2^{4-e}$; we shall compute
//! $$\lfloor(b+c\cdot2^{-8}+d\cdot2^{-16})\,z^\prime/\beta\rfloor$$
//! if $a=0$, or the same quantity minus $\alpha=2^{4+e}z^\prime$ if $a=255$.
//! This calculation must be done exactly, in order to guarantee portability
//! of \TeX\ between computers.
//
// @d store_scaled(#)==begin fget; a:=fbyte; fget; b:=fbyte;
pub(crate) macro store_scaled($globals:expr, $val:expr, $z:expr, $alpha:expr, $beta:expr, $lbl_bad_tfm:lifetime) {{
    /// byte variables
    let (a, b, c, d): (eight_bits, eight_bits, eight_bits, eight_bits);
    /// accumulators
    let sw: scaled;

    fget!($globals);
    a = fbyte!($globals);
    fget!($globals);
    b = fbyte!($globals);
    // fget; c:=fbyte; fget; d:=fbyte;@/
    fget!($globals);
    c = fbyte!($globals);
    fget!($globals);
    d = fbyte!($globals);
    // sw:=(((((d*z)div@'400)+(c*z))div@'400)+(b*z))div beta;
    sw = scaled::new_from_inner(
        (((d as integer * $z) / 0o400 + (c as integer * $z)) / 0o400 + (b as integer * $z))
            / $beta as integer,
    );
    // if a=0 then #:=sw@+else if a=255 then #:=sw-alpha@+else abort;
    if a == 0 {
        $val = sw;
    } else if a == 255 {
        $val = sw - scaled::new_from_inner($alpha);
    } else {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // end
    use crate::pascal::integer;
    use crate::section_0025::eight_bits;
    use crate::section_0101::scaled;
    use crate::section_0564::fbyte;
    use crate::section_0564::fget;
}}
//
// @<Read box dimensions@>=
pub(crate) macro Read_box_dimensions($globals:expr, $f:expr, $z:expr, $alpha:expr, $beta:expr, $lbl_bad_tfm:lifetime) {{
    // begin @<Replace |z| by $|z|^\prime$ and compute $\alpha,\beta$@>;
    crate::section_0572::Replace_z_by_z_prime_and_compute_alpha_beta!($globals, $z, $alpha, $beta);
    // for k:=width_base[f] to lig_kern_base[f]-1 do
    for k in $globals.width_base[$f]..=$globals.lig_kern_base[$f] - 1 {
        let k = k as pointer;
        // store_scaled(font_info[k].sc);
        store_scaled!(
            $globals,
            $globals.font_info[k][MEMORY_WORD_SC],
            $z.inner(),
            $alpha,
            $beta,
            $lbl_bad_tfm
        );
    }
    // if font_info[width_base[f]].sc<>0 then abort; {\\{width}[0] must be zero}
    /// `width`[0] must be zero
    if $globals.font_info[$globals.width_base[$f] as pointer][MEMORY_WORD_SC] != scaled::zero() {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // if font_info[height_base[f]].sc<>0 then abort; {\\{height}[0] must be zero}
    /// `height`[0] must be zero
    if $globals.font_info[$globals.height_base[$f] as pointer][MEMORY_WORD_SC] != scaled::zero() {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // if font_info[depth_base[f]].sc<>0 then abort; {\\{depth}[0] must be zero}
    if $globals.font_info[$globals.depth_base[$f] as pointer][MEMORY_WORD_SC] != scaled::zero() {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // if font_info[italic_base[f]].sc<>0 then abort; {\\{italic}[0] must be zero}
    if $globals.font_info[$globals.italic_base[$f] as pointer][MEMORY_WORD_SC] != scaled::zero() {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // end
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0115::pointer;
}}
