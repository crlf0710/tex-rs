//! @ Each noad is four or more words long. The first word contains the |type|
//! and |subtype| and |link| fields that are already so familiar to us; the
//! second, third, and fourth words are called the noad's |nucleus|, |subscr|,
//! and |supscr| fields.
//!
//! Consider, for example, the simple formula `\.{\$x\^2\$}', which would be
//! parsed into an mlist containing a single element called an |ord_noad|.
//! The |nucleus| of this noad is a representation of `\.x', the |subscr| is
//! empty, and the |supscr| is a representation of `\.2'.
//!
//! The |nucleus|, |subscr|, and |supscr| fields are further broken into
//! subfields. If |p| points to a noad, and if |q| is one of its principal
//! fields (e.g., |q=subscr(p)|), there are several possibilities for the
//! subfields, depending on the |math_type| of |q|.
//!
//! \yskip\hang|math_type(q)=math_char| means that |fam(q)| refers to one of
//! the sixteen font families, and |character(q)| is the number of a character
//! within a font of that family, as in a character node.
//!
//! \yskip\hang|math_type(q)=math_text_char| is similar, but the character is
//! unsubscripted and unsuperscripted and it is followed immediately by another
//! character from the same font. (This |math_type| setting appears only
//! briefly during the processing; it is used to suppress unwanted italic
//! corrections.)
//!
//! \yskip\hang|math_type(q)=empty| indicates a field with no value (the
//! corresponding attribute of noad |p| is not present).
//!
//! \yskip\hang|math_type(q)=sub_box| means that |info(q)| points to a box
//! node (either an |hlist_node| or a |vlist_node|) that should be used as the
//! value of the field.  The |shift_amount| in the subsidiary box node is the
//! amount by which that box will be shifted downward.
//!
//! \yskip\hang|math_type(q)=sub_mlist| means that |info(q)| points to
//! an mlist; the mlist must be converted to an hlist in order to obtain
//! the value of this field.
//!
//! \yskip\noindent In the latter case, we might have |info(q)=null|. This
//! is not the same as |math_type(q)=empty|; for example, `\.{\$P\_\{\}\$}'
//! and `\.{\$P\$}' produce different results (the former will not have the
//! ``italic correction'' added to the width of |P|, but the ``script skip''
//! will be added).
//!
//! The definitions of subfields given here are evidently wasteful of space,
//! since a halfword is being used for the |math_type| although only three
//! bits would be needed. However, there are hardly ever many noads present at
//! once, since they are soon converted to nodes that take up even more space,
//! so we can afford to represent them in whatever way simplifies the
//! programming.
//
// @d noad_size=4 {number of words in a normal noad}
/// number of words in a normal noad
pub(crate) const noad_size: quarterword = 4;
// @d nucleus(#)==#+1 {the |nucleus| field of a noad}
/// the `nucleus` field of a noad
pub(crate) macro nucleus($p:expr) {
    $p + 1
}
// @d supscr(#)==#+2 {the |supscr| field of a noad}
/// the `supscr` field of a noad
pub(crate) macro supscr($p:expr) {
    $p + 2
}
// @d subscr(#)==#+3 {the |subscr| field of a noad}
/// the `subscr` field of a noad
pub(crate) macro subscr($p:expr) {
    $p + 3
}
// @d math_type==link {a |halfword| in |mem|}
/// a `halfword` in `mem`
pub(crate) macro math_type($globals:expr, $p:expr) {
    crate::section_0118::link!($globals, $p)
}

// @d fam==font {a |quarterword| in |mem|}
/// a `quarterword` in `mem`
pub(crate) macro fam($globals:expr, $f:expr) {
    crate::section_0134::font!($globals, $f)
}

pub(crate) macro assign_fam_and_character($globals:expr, $f:expr, $fam:expr, $character:expr) {
    crate::section_0134::assign_font_and_character!($globals, $f, $fam, $character)
}

// @d math_char=1 {|math_type| when the attribute is simple}
// @d sub_box=2 {|math_type| when the attribute is a box}
// @d sub_mlist=3 {|math_type| when the attribute is a formula}
// @d math_text_char=4 {|math_type| when italic correction is dubious}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum math_type_kind {
    /// symbolic name for a null constant
    empty = 0,
    /// `math_type` when the attribute is simple
    math_char = 1,
    /// `math_type` when the attribute is a box
    sub_box = 2,
    /// `math_type` when the attribute is a formula
    sub_mlist = 3,
    /// `math_type` when italic correction is dubious
    math_text_char = 4,
}

use crate::section_0113::quarterword;
