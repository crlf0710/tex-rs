//! @ A |glue_node| represents glue in a list. However, it is really only
//! a pointer to a separate glue specification, since \TeX\ makes use of the
//! fact that many essentially identical nodes of glue are usually present.
//! If |p| points to a |glue_node|, |glue_ptr(p)| points to
//! another packet of words that specify the stretch and shrink components, etc.
//!
//! Glue nodes also serve to represent leaders; the |subtype| is used to
//! distinguish between ordinary glue (which is called |normal|) and the three
//! kinds of leaders (which are called |a_leaders|, |c_leaders|, and |x_leaders|).
//! The |leader_ptr| field points to a rule node or to a box node containing the
//! leaders; it is set to |null| in ordinary glue nodes.
//!
//! Many kinds of glue are computed from \TeX's ``skip'' parameters, and
//! it is helpful to know which parameter has led to a particular glue node.
//! Therefore the |subtype| is set to indicate the source of glue, whenever
//! it originated as a parameter. We will be defining symbolic names for the
//! parameter numbers later (e.g., |line_skip_code=0|, |baseline_skip_code=1|,
//! etc.); it suffices for now to say that the |subtype| of parametric glue
//! will be the same as the parameter number, plus~one.
//!
//! In math formulas there are two more possibilities for the |subtype| in a
//! glue node: |mu_glue| denotes an \.{\\mskip} (where the units are scaled \.{mu}
//! instead of scaled \.{pt}); and |cond_math_glue| denotes the `\.{\\nonscript}'
//! feature that cancels the glue node immediately following if it appears
//! in a subscript.

// @d glue_node=10 {|type| of node that points to a glue specification}
/// `type` of node that points to a glue specification
pub(crate) const glue_node: quarterword = 10;
// @d cond_math_glue=98 {special |subtype| to suppress glue in the next node}
// @d mu_glue=99 {|subtype| for math glue}
// @d a_leaders=100 {|subtype| for aligned leaders}
// @d c_leaders=101 {|subtype| for centered leaders}
// @d x_leaders=102 {|subtype| for expanded leaders}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub(crate) enum glue_node_subtype {
    normal = 0,
    /// special `subtype` to suppress glue in the next node
    cond_math_glue = 98,
    /// `subtype` for math glue
    mu_glue = 99,
    /// `subtype` for aligned leaders
    a_leaders = 100,
    /// `subtype` for centered leaders
    c_leaders = 101,
    /// `subtype` for expanded leaders
    x_leaders = 102,
}
// @d glue_ptr==llink {pointer to a glue specification}
/// pointer to a glue specification
macro_rules! glue_ptr {
    ($globals:expr, $ptr:expr) => {
        llink!($globals, $ptr)
    };
}
// @d leader_ptr==rlink {pointer to box or rule node for leaders}
/// pointer to box or rule node for leaders
macro_rules! leader_ptr {
    ($globals:expr, $ptr:expr) => {
        rlink!($globals, $ptr)
    };
}

use crate::section_0113::quarterword;
