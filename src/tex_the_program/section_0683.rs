//! @ A |radical_noad| is five words long; the fifth word is the |left_delimiter|
//! field, which usually represents a square root sign.
//!
//! A |fraction_noad| is six words long; it has a |right_delimiter| field
//! as well as a |left_delimiter|.
//!
//! Delimiter fields are of type |four_quarters|, and they have four subfields
//! called |small_fam|, |small_char|, |large_fam|, |large_char|. These subfields
//! represent variable-size delimiters by giving the ``small'' and ``large''
//! starting characters, as explained in Chapter~17 of {\sl The \TeX book}.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! A |fraction_noad| is actually quite different from all other noads. Not
//! only does it have six words, it has |thickness|, |denominator|, and
//! |numerator| fields instead of |nucleus|, |subscr|, and |supscr|. The
//! |thickness| is a scaled value that tells how thick to make a fraction
//! rule; however, the special value |default_code| is used to stand for the
//! |default_rule_thickness| of the current size. The |numerator| and
//! |denominator| point to mlists that define a fraction; we always have
//! $$\hbox{|math_type(numerator)=math_type(denominator)=sub_mlist|}.$$ The
//! |left_delimiter| and |right_delimiter| fields specify delimiters that will
//! be placed at the left and right of the fraction. In this way, a
//! |fraction_noad| is able to represent all of \TeX's operators \.{\\over},
//! \.{\\atop}, \.{\\above}, \.{\\overwithdelims}, \.{\\atopwithdelims}, and
//!  \.{\\abovewithdelims}.
//
// @d left_delimiter(#)==#+4 {first delimiter field of a noad}
/// first delimiter field of a noad
pub(crate) macro left_delimiter($v:expr) {
    $v + 4
}
// @d right_delimiter(#)==#+5 {second delimiter field of a fraction noad}
/// second delimiter field of a fraction noad
pub(crate) macro right_delimiter($v:expr) {
    $v + 5
}
// @d radical_noad=inner_noad+1 {|type| of a noad for square roots}
/// `type` of a noad for square roots
pub(crate) const radical_noad: quarterword = inner_noad + 1;
// @d radical_noad_size=5 {number of |mem| words in a radical noad}
/// number of `mem` words in a radical noad
pub(crate) const radical_noad_size: quarterword = 5;
// @d fraction_noad=radical_noad+1 {|type| of a noad for generalized fractions}
/// `type` of a noad for generalized fractions
pub(crate) const fraction_noad: quarterword = radical_noad + 1;
// @d fraction_noad_size=6 {number of |mem| words in a fraction noad}
/// number of `mem` words in a fraction noad
pub(crate) const fraction_noad_size: quarterword = 6;
// @d small_fam(#)==mem[#].qqqq.b0 {|fam| for ``small'' delimiter}
// @d small_char(#)==mem[#].qqqq.b1 {|character| for ``small'' delimiter}

#[cfg(not(feature = "unicode_support"))]
/// `fam` for "small" delimiter
pub(crate) macro small_fam($globals:expr, $f:expr) {
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B0]
}
#[cfg(feature = "unicode_support")]
/// `fam` for "small" delimiter
pub(crate) macro small_fam($globals:expr, $f:expr) {
    crate::unicode_support::fontchar_value(
        $globals,
        $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH],
    )
    .font
}

#[cfg(not(feature = "unicode_support"))]
/// `character` for "small" delimiter
pub(crate) macro small_char($globals:expr, $f:expr) {
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B1]
}
#[cfg(feature = "unicode_support")]
/// `character` for "small" delimiter
pub(crate) macro small_char($globals:expr, $f:expr) {
    crate::unicode_support::fontchar_value(
        $globals,
        $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH],
    )
    .character
}

#[cfg(not(feature = "unicode_support"))]
pub(crate) macro assign_small_fam_and_char($globals:expr, $f:expr, $fam:expr, $char:expr) {{
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B0] = $fam;
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B1] = $char;
}}

#[cfg(feature = "unicode_support")]
pub(crate) macro assign_small_fam_and_char($globals:expr, $f:expr, $fam:expr, $char:expr) {{
    $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH] =
        crate::unicode_support::register_fontchar_value(
            $globals,
            crate::section_0134::font_and_character {
                font: $fam,
                character: $char,
            },
        );
}}

// @d large_fam(#)==mem[#].qqqq.b2 {|fam| for ``large'' delimiter}
// @d large_char(#)==mem[#].qqqq.b3 {|character| for ``large'' delimiter}

#[cfg(not(feature = "unicode_support"))]
/// `fam` for "large" delimiter
pub(crate) macro large_fam($globals:expr, $f:expr) {
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B2]
}
#[cfg(feature = "unicode_support")]
/// `fam` for "large" delimiter
pub(crate) macro large_fam($globals:expr, $f:expr) {
    crate::unicode_support::fontchar_value(
        $globals,
        $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_RH],
    )
    .font
}

#[cfg(not(feature = "unicode_support"))]
/// `character` for "large" delimiter
pub(crate) macro large_char($globals:expr, $f:expr) {
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B3]
}
#[cfg(feature = "unicode_support")]
/// `character` for "large" delimiter
pub(crate) macro large_char($globals:expr, $f:expr) {
    crate::unicode_support::fontchar_value(
        $globals,
        $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_RH],
    )
    .character
}

#[cfg(not(feature = "unicode_support"))]
pub(crate) macro assign_large_fam_and_char($globals:expr, $f:expr, $fam:expr, $char:expr) {{
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B2] = $fam;
    $globals.mem[$f][MEMORY_WORD_QQQQ][FOUR_QUARTERS_B3] = $char;
}}

#[cfg(feature = "unicode_support")]
pub(crate) macro assign_large_fam_and_char($globals:expr, $f:expr, $fam:expr, $char:expr) {{
    $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_RH] =
        crate::unicode_support::register_fontchar_value(
            $globals,
            crate::section_0134::font_and_character {
                font: $fam,
                character: $char,
            },
        );
}}
// @d thickness==width {|thickness| field in a fraction noad}
// @d default_code==@'10000000000 {denotes |default_rule_thickness|}
// @d numerator==supscr {|numerator| field in a fraction noad}
// @d denominator==subscr {|denominator| field in a fraction noad}
//
use crate::section_0113::quarterword;
use crate::section_0682::inner_noad;
