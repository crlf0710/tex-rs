//! @ You might think that we have introduced enough node types by now. Well,
//! almost, but there is one more: An |unset_node| has nearly the same format
//! as an |hlist_node| or |vlist_node|; it is used for entries in \.{\\halign}
//! or \.{\\valign} that are not yet in their final form, since the box
//! dimensions are their ``natural'' sizes before any glue adjustment has been
//! made. The |glue_set| word is not present; instead, we have a |glue_stretch|
//! field, which contains the total stretch of order |glue_order| that is
//! present in the hlist or vlist being boxed.
//! Similarly, the |shift_amount| field is replaced by a |glue_shrink| field,
//! containing the total shrink of order |glue_sign| that is present.
//! The |subtype| field is called |span_count|; an unset box typically
//! contains the data for |qo(span_count)+1| columns.
//! Unset nodes will be changed to box nodes when alignment is completed.
//
// @d unset_node=13 {|type| for an unset node}
/// `type` for an unset node
pub(crate) const unset_node: quarterword = 13;
// @d glue_stretch(#)==mem[#+glue_offset].sc {total stretch in an unset node}
// @d glue_shrink==shift_amount {total shrink in an unset node}
// @d span_count==subtype {indicates the number of spanned columns}
//

use crate::section_0113::quarterword;
