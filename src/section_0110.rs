//! @* \[8] Packed data.
//!
//! In order to make efficient use of storage space, \TeX\ bases its major data
//! structures on a |memory_word|, which contains either a (signed) integer,
//! possibly scaled, or a (signed) |glue_ratio|, or a small number of
//! fields that are one half or one quarter of the size used for storing
//! integers.
//!
//! If |x| is a variable of type |memory_word|, it contains up to four
//! fields that can be referred to as follows:
//! $$\vbox{\halign{\hfil#&#\hfil&#\hfil\cr
//! |x|&.|int|&(an |integer|)\cr
//! |x|&.|sc|\qquad&(a |scaled| integer)\cr
//! |x|&.|gr|&(a |glue_ratio|)\cr
//! |x.hh.lh|, |x.hh|&.|rh|&(two halfword fields)\cr
//! |x.hh.b0|, |x.hh.b1|, |x.hh|&.|rh|&(two quarterword fields, one halfword
//!   field)\cr
//! |x.qqqq.b0|, |x.qqqq.b1|, |x.qqqq|&.|b2|, |x.qqqq.b3|\hskip-100pt
//!   &\qquad\qquad\qquad(four quarterword fields)\cr}}$$
//! This is somewhat cumbersome to write, and not very readable either, but
//! macros will be used to make the notation shorter and more transparent.
//! The \PASCAL\ code below gives a formal definition of |memory_word| and
//! its subsidiary types, using packed variant records. \TeX\ makes no
//! assumptions about the relative positions of the fields within a word.
//!
//! Since we are assuming 32-bit integers, a halfword must contain at least
//! 16 bits, and a quarterword must contain at least 8 bits.
//! @^system dependencies@>
//! But it doesn't hurt to have more bits; for example, with enough 36-bit
//! words you might be able to have |mem_max| as large as 262142, which is
//! eight times as much memory as anybody had during the first four years of
//! \TeX's existence.
//!
//! N.B.: Valuable memory space will be dreadfully wasted unless \TeX\ is compiled
//! by a \PASCAL\ that packs all of the |memory_word| variants into
//! the space of a single integer. This means, for example, that |glue_ratio|
//! words should be |short_real| instead of |real| on some computers. Some
//! \PASCAL\ compilers will pack an integer whose subrange is `|0..255|' into
//! an eight-bit field, but others insist on allocating space for an additional
//! sign bit; on such systems you can get 256 values into a quarterword only
//! if the subrange is `|-128..127|'.
//!
//! The present implementation tries to accommodate as many variations as possible,
//! so it makes few assumptions. If integers having the subrange
//! `|min_quarterword..max_quarterword|' can be packed into a quarterword,
//! and if integers having the subrange `|min_halfword..max_halfword|'
//! can be packed into a halfword, everything should work satisfactorily.
//!
//! It is usually most efficient to have |min_quarterword=min_halfword=0|,
//! so one should try to achieve this unless it causes a severe problem.
//! The values defined here are recommended for most 32-bit computers.
//!
//! @d min_quarterword=0 {smallest allowable value in a |quarterword|}
//! @d max_quarterword=255 {largest allowable value in a |quarterword|}
//! @d min_halfword==0 {smallest allowable value in a |halfword|}
//! @d max_halfword==65535 {largest allowable value in a |halfword|}
//!

/// smallest allowable value in a `quarterword`
pub const min_quarterword: quarterword = 0;

/// largest allowable value in a `quarterword`
pub const max_quarterword: quarterword = 255;

/// smallest allowable value in a `halfword`
pub const min_halfword: halfword = 0;

/// largest allowable value in a `halfword`
pub const max_halfword: halfword = 65535;

use crate::section_0113::{quarterword, halfword};
