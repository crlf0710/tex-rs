//! @ Note: A malformed \.{TFM} file might be shorter than it claims to be;
//! thus |eof(tfm_file)| might be true when |read_font_info| refers to
//! |tfm_file^| or when it says |get(tfm_file)|. If such circumstances
//! cause system error messages, you will have to defeat them somehow,
//! for example by defining |fget| to be `\ignorespaces|begin get(tfm_file);|
//! |if eof(tfm_file) then abort; end|\unskip'.
//! @^system dependencies@>
//
// @d fget==get(tfm_file)
pub(crate) macro fget($globals:expr) {
    crate::io_support::get(&mut $globals.tfm_file)
}
// @d fbyte==tfm_file^
pub(crate) macro fbyte($globals:expr) {
    crate::io_support::buffer_variable(&mut $globals.tfm_file)
}
// @d read_sixteen(#)==begin #:=fbyte;
pub(crate) macro read_sixteen($globals:expr, $val:expr, $lbl_bad_tfm:lifetime) {
    $val = fbyte!($globals) as u16;
    // if #>127 then abort;
    if $val > 127 {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // fget; #:=#*@'400+fbyte;
    fget!($globals);
    $val = $val * 0o400 + fbyte!($globals) as u16;
    // end
}

// @d store_four_quarters(#)==begin fget; a:=fbyte; qw.b0:=qi(a);
pub(crate) macro store_four_quarters($globals:expr, $val:expr) {{
    /// byte variables
    let (a, b, c, d): (eight_bits, eight_bits, eight_bits, eight_bits);
    /// accumulators
    let mut qw: four_quarters = four_quarters::default();

    fget!($globals);
    a = fbyte!($globals);
    qw[FOUR_QUARTERS_B0] = qi!(a);
    // fget; b:=fbyte; qw.b1:=qi(b);
    fget!($globals);
    b = fbyte!($globals);
    qw[FOUR_QUARTERS_B1] = qi!(b);
    // fget; c:=fbyte; qw.b2:=qi(c);
    fget!($globals);
    c = fbyte!($globals);
    qw[FOUR_QUARTERS_B2] = qi!(c);
    // fget; d:=fbyte; qw.b3:=qi(d);
    fget!($globals);
    d = fbyte!($globals);
    qw[FOUR_QUARTERS_B3] = qi!(d);
    // #:=qw;
    $val = qw;
    // end

    use crate::section_0025::eight_bits;
    use crate::section_0112::qi;
    use crate::section_0113::four_quarters;
    use crate::section_0113::FOUR_QUARTERS_B0;
    use crate::section_0113::FOUR_QUARTERS_B1;
    use crate::section_0113::FOUR_QUARTERS_B2;
    use crate::section_0113::FOUR_QUARTERS_B3;
}}
