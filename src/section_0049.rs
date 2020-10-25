//! @ The first 128 strings will contain 95 standard ASCII characters, and the
//! other 33 characters will be printed in three-symbol form like `\.{\^\^A}'
//! unless a system-dependent change is made here. Installations that have
//! an extended character set, where for example |xchr[@'32]=@t\.{\'^^Z\'}@>|,
//! would like string @'32 to be the single character @'32 instead of the
//! three characters @'136, @'136, @'132 (\.{\^\^Z}). On the other hand,
//! even people with an extended character set will want to represent string
//! @'15 by \.{\^\^M}, since @'15 is |carriage_return|; the idea is to
//! produce visible strings instead of tabs or line-feeds or carriage-returns
//! or bell-rings or characters that are treated anomalously in text files.
//!
//! Unprintable characters of codes 128--255 are, similarly, rendered
//! \.{\^\^80}--\.{\^\^ff}.
//!
//! The boolean expression defined here should be |true| unless \TeX\
//! internal code number~|k| corresponds to a non-troublesome visible
//! symbol in the local character set.  An appropriate formula for the
//! extended character set recommended in {\sl The \TeX book\/} would, for
//! example, be `|k in [0,@'10..@'12,@'14,@'15,@'33,@'177..@'377]|'.
//! If character |k| cannot be printed, and |k<@'200|, then character |k+@'100| or
//! |k-@'100| must be printable; moreover, ASCII codes |[@'41..@'46,
//! @'60..@'71, @'136, @'141..@'146, @'160..@'171]| must be printable.
//! Thus, at least 81 printable characters are needed.
//! @:TeXbook}{\sl The \TeX book@>
//! @^character set dependencies@>
//! @^system dependencies@>
//
// @<Character |k| cannot be printed@>=
macro_rules! Character_k_cannot_be_printed {
    ($k:expr) => {
        // NOTE: This is used for 256 single byte characters
        $k < b' ' || $k > b'~'
    }
}
