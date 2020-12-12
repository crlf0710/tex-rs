//! @ \TeX\ makes use of the fact that |hlist_node|, |vlist_node|,
//! |rule_node|, |ins_node|, |mark_node|, |adjust_node|, |ligature_node|,
//! |disc_node|, |whatsit_node|, and |math_node| are at the low end of the
//! type codes, by permitting a break at glue in a list if and only if the
//! |type| of the previous node is less than |math_node|. Furthermore, a
//! node is discarded after a break if its type is |math_node| or~more.
//!
//! @d precedes_break(#)==(type(#)<math_node)
//! @d non_discardable(#)==(type(#)<math_node)
//!
