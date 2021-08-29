//! @ A |@!char_node|, which represents a single character, is the most important
//! kind of node because it accounts for the vast majority of all boxes.
//! Special precautions are therefore taken to ensure that a |char_node| does
//! not take up much memory space. Every such node is one word long, and in fact
//! it is identifiable by this property, since other kinds of nodes have at least
//! two words, and they appear in |mem| locations less than |hi_mem_min|.
//! This makes it possible to omit the |type| field in a |char_node|, leaving
//! us room for two bytes that identify a |font| and a |character| within
//! that font.
//!
//! Note that the format of a |char_node| allows for up to 256 different
//! fonts and up to 256 characters per font; but most implementations will
//! probably limit the total number of fonts to fewer than 75 per job,
//! and most fonts will stick to characters whose codes are
//! less than 128 (since higher codes
//! are more difficult to access on most keyboards).
//!
//! Extensions of \TeX\ intended for oriental languages will need even more
//! than $256\times256$ possible characters, when we consider different sizes
//! @^oriental characters@>@^Chinese characters@>@^Japanese characters@>
//! and styles of type.  It is suggested that Chinese and Japanese fonts be
//! handled by representing such characters in two consecutive |char_node|
//! entries: The first of these has |font=font_base|, and its |link| points
//! to the second;
//! the second identifies the font and the character dimensions.
//! The saving feature about oriental characters is that most of them have
//! the same box dimensions. The |character| field of the first |char_node|
//! is a ``\\{charext}'' that distinguishes between graphic symbols whose
//! dimensions are identical for typesetting purposes. (See the \MF\ manual.)
//! Such an extension of \TeX\ would not be difficult; further details are
//! left to the reader.
//!
//! In order to make sure that the |character| code fits in a quarterword,
//! \TeX\ adds the quantity |min_quarterword| to the actual code.
//!
//! Character nodes appear only in horizontal lists, never in vertical lists.
//
// @d is_char_node(#) == (#>=hi_mem_min)
//   {does the argument point to a |char_node|?}
/// does the argument point to a `char_node`?
macro_rules! is_char_node {
    ($globals:expr, $ptr:expr) => {
        $ptr >= $globals.hi_mem_min
    };
}

// @d font == type {the font code in a |char_node|}
/// the font code in a `char_node`
#[cfg(not(feature = "unicode_support"))]
macro_rules! font {
    ($globals:expr, $f:expr) => {
        r#type!($globals, $f)
    };
}

/// the font code in a `char_node`
#[cfg(feature = "unicode_support")]
macro_rules! font {
    ($globals:expr, $f:expr) => {
        crate::unicode_support::fontchar_value(
            $globals,
            $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH],
        )
        .font
    };
}

// @d character == subtype {the character code in a |char_node|}

/// the font code in a `char_node`
#[cfg(not(feature = "unicode_support"))]
macro_rules! character {
    ($globals:expr, $f:expr) => {
        subtype!($globals, $f)
    };
}

/// the font code in a `char_node`
#[cfg(feature = "unicode_support")]
macro_rules! character {
    ($globals:expr, $f:expr) => {
        crate::unicode_support::fontchar_value(
            $globals,
            $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH],
        )
        .character
    };
}

#[cfg(not(feature = "unicode_support"))]
macro_rules! assign_font_and_character {
    ($globals:expr, $f:expr, $font:expr, $character:expr) => {{
        r#type!($globals, $f) = $font;
        subtype!($globals, $f) = $character;
    }};
}

#[cfg(feature = "unicode_support")]
macro_rules! assign_font_and_character {
    ($globals:expr, $f:expr, $font:expr, $character:expr) => {{
        $globals.mem[$f][crate::section_0113::MEMORY_WORD_HH_LH] =
            crate::unicode_support::register_fontchar_value(
                $globals,
                crate::section_0134::font_and_character {
                    font: $font,
                    character: $character,
                },
            );
    }};
}

#[cfg(feature = "unicode_support")]
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct font_and_character {
    pub(crate) font: internal_font_number,
    pub(crate) character: ASCII_code,
}

#[cfg(feature = "unicode_support")]
impl Default for font_and_character {
    fn default() -> Self {
        font_and_character {
            font: internal_font_number::new(font_base as _),
            character: ASCII_code_literal!(b' '),
        }
    }
}

use crate::section_0012::font_base;
use crate::section_0018::ASCII_code;
use crate::section_0548::internal_font_number;
