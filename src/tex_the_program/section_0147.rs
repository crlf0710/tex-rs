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
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum math_node_subtype {
    /// `subtype` for math node that introduces a formula
    before = 0,
    /// `subtype` for math node that winds up a formula
    after = 1,
}

// @p function new_math(@!w:scaled;@!s:small_number):pointer;
pub(crate) fn new_math(
    globals: &mut TeXGlobals,
    w: scaled,
    s: math_node_subtype,
) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p;
    // begin p:=get_node(small_node_size); type(p):=math_node;
    p = get_node(globals, small_node_size as _)?;
    r#type!(globals, p) = math_node;
    // subtype(p):=s; width(p):=w; new_math:=p;
    subtype!(globals, p) = s as _;
    width!(globals, p) = w;
    crate::ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0135::width;
use crate::section_0141::small_node_size;
