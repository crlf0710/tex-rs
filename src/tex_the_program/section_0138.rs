//! @ A |rule_node| stands for a solid black rectangle; it has |width|,
//! |depth|, and |height| fields just as in an |hlist_node|. However, if
//! any of these dimensions is $-2^{30}$, the actual value will be determined
//! by running the rule up to the boundary of the innermost enclosing box.
//! This is called a ``running dimension.'' The |width| is never running in
//! an hlist; the |height| and |depth| are never running in a~vlist.
//
// @d rule_node=2 {|type| of rule nodes}
/// `type` of rule nodes
pub(crate) const rule_node: quarterword = 2;
// @d rule_node_size=4 {number of words to allocate for a rule node}
/// number of words to allocate for a rule node
pub(crate) const rule_node_size: quarterword = 4;
// @d null_flag==-@'10000000000 {$-2^{30}$, signifies a missing item}
/// `-2^{30}`, signifies a missing item
pub(crate) const null_flag: scaled = scaled::new_from_inner(-0o10000000000);
// @d is_running(#) == (#=null_flag) {tests for a running dimension}
/// tests for a running dimension
pub(crate) macro is_running($v:expr) {
    $v == crate::section_0138::null_flag
}

use crate::pascal::integer;
use crate::section_0101::scaled;
use crate::section_0113::quarterword;
