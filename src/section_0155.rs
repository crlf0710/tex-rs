//! @ A |kern_node| has a |width| field to specify a (normally negative)
//! amount of spacing. This spacing correction appears in horizontal lists
//! between letters like A and V when the font designer said that it looks
//! better to move them closer together or further apart. A kern node can
//! also appear in a vertical list, when its `|width|' denotes additional
//! spacing in the vertical direction. The |subtype| is either |normal| (for
//! kerns inserted from font information or math mode calculations) or |explicit|
//! (for kerns inserted from \.{\\kern} and \.{\\/} commands) or |acc_kern|
//! (for kerns inserted from non-math accents) or |mu_glue| (for kerns
//! inserted from \.{\\mkern} specifications in math formulas).
//!
//! @d kern_node=11 {|type| of a kern node}
//! @d explicit=1 {|subtype| of kern nodes from \.{\\kern} and \.{\\/}}
//! @d acc_kern=2 {|subtype| of kern nodes from accents}
//!

