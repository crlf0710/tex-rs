//! @ Each portion of a formula is classified as Ord, Op, Bin, Rel, Open,
//! Close, Punct, or Inner, for purposes of spacing and line breaking. An
//! |ord_noad|, |op_noad|, |bin_noad|, |rel_noad|, |open_noad|, |close_noad|,
//! |punct_noad|, or |inner_noad| is used to represent portions of the various
//! types. For example, an `\.=' sign in a formula leads to the creation of a
//! |rel_noad| whose |nucleus| field is a representation of an equals sign
//! (usually |fam=0|, |character=@'75|).  A formula preceded by \.{\\mathrel}
//! also results in a |rel_noad|.  When a |rel_noad| is followed by an
//! |op_noad|, say, and possibly separated by one or more ordinary nodes (not
//! noads), \TeX\ will insert a penalty node (with the current |rel_penalty|)
//! just after the formula that corresponds to the |rel_noad|, unless there
//! already was a penalty immediately following; and a ``thick space'' will be
//! inserted just before the formula that corresponds to the |op_noad|.
//!
//! A noad of type |ord_noad|, |op_noad|, \dots, |inner_noad| usually
//! has a |subtype=normal|. The only exception is that an |op_noad| might
//! have |subtype=limits| or |no_limits|, if the normal positioning of
//! limits has been overridden for this operator.
//
// @d ord_noad=unset_node+3 {|type| of a noad classified Ord}
/// `type` of a noad classified Ord
pub(crate) const ord_noad: quarterword = unset_node + 3;
// @d op_noad=ord_noad+1 {|type| of a noad classified Op}
// @d bin_noad=ord_noad+2 {|type| of a noad classified Bin}
// @d rel_noad=ord_noad+3 {|type| of a noad classified Rel}
// @d open_noad=ord_noad+4 {|type| of a noad classified Open}
// @d close_noad=ord_noad+5 {|type| of a noad classified Close}
// @d punct_noad=ord_noad+6 {|type| of a noad classified Punct}
// @d inner_noad=ord_noad+7 {|type| of a noad classified Inner}
/// `type` of a noad classified Inner
pub(crate) const inner_noad: quarterword = ord_noad + 7;
// @d limits=1 {|subtype| of |op_noad| whose scripts are to be above, below}
// @d no_limits=2 {|subtype| of |op_noad| whose scripts are to be normal}

use crate::section_0113::quarterword;
use crate::section_0159::unset_node;
