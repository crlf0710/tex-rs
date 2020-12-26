//! @ Insertions are represented by |ins_node| records, where the |subtype|
//! indicates the corresponding box number. For example, `\.{\\insert 250}'
//! leads to an |ins_node| whose |subtype| is |250+min_quarterword|.
//! The |height| field of an |ins_node| is slightly misnamed; it actually holds
//! the natural height plus depth of the vertical list being inserted.
//! The |depth| field holds the |split_max_depth| to be used in case this
//! insertion is split, and the |split_top_ptr| points to the corresponding
//! |split_top_skip|. The |float_cost| field holds the |floating_penalty| that
//! will be used if this insertion floats to a subsequent page after a
//! split insertion of the same class.  There is one more field, the
//! |ins_ptr|, which points to the beginning of the vlist for the insertion.
//
// @d ins_node=3 {|type| of insertion nodes}
/// `type` of insertion nodes
pub(crate) const ins_node: quarterword = 3;
// @d ins_node_size=5 {number of words to allocate for an insertion}
// @d float_cost(#)==mem[#+1].int {the |floating_penalty| to be used}
// @d ins_ptr(#)==info(#+4) {the vertical list to be inserted}
// @d split_top_ptr(#)==link(#+4) {the |split_top_skip| to be used}
//

use crate::section_0113::quarterword;
