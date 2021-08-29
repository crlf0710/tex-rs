//! @ A |math_node|, which occurs only in horizontal lists, appears before and
//! after mathematical formulas. The |subtype| field is |before| before the
//! formula and |after| after it. There is a |width| field, which represents
//! the amount of surrounding space inserted by \.{\\mathsurround}.
//
// @d math_node=9 {|type| of a math node}
/// `type` of a math node
pub(crate) const math_node: quarterword = 9;
// @d before=0 {|subtype| for math node that introduces a formula}
// @d after=1 {|subtype| for math node that winds up a formula}
//
// @p function new_math(@!w:scaled;@!s:small_number):pointer;
// var p:pointer; {the new node}
// begin p:=get_node(small_node_size); type(p):=math_node;
// subtype(p):=s; width(p):=w; new_math:=p;
// end;
//

use crate::section_0113::quarterword;
