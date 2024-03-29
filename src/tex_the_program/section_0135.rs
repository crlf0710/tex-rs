//! @ An |hlist_node| stands for a box that was made from a horizontal list.
//! Each |hlist_node| is seven words long, and contains the following fields
//! (in addition to the mandatory |type| and |link|, which we shall not
//! mention explicitly when discussing the other node types): The |height| and
//! |width| and |depth| are scaled integers denoting the dimensions of the
//! box.  There is also a |shift_amount| field, a scaled integer indicating
//! how much this box should be lowered (if it appears in a horizontal list),
//! or how much it should be moved to the right (if it appears in a vertical
//! list). There is a |list_ptr| field, which points to the beginning of the
//! list from which this box was fabricated; if |list_ptr| is |null|, the box
//! is empty. Finally, there are three fields that represent the setting of
//! the glue:  |glue_set(p)| is a word of type |glue_ratio| that represents
//! the proportionality constant for glue setting; |glue_sign(p)| is
//! |stretching| or |shrinking| or |normal| depending on whether or not the
//! glue should stretch or shrink or remain rigid; and |glue_order(p)|
//! specifies the order of infinity to which glue setting applies (|normal|,
//! |fil|, |fill|, or |filll|). The |subtype| field is not used.
//
// @d hlist_node=0 {|type| of hlist nodes}
/// `type` of hlist nodes
pub(crate) const hlist_node: quarterword = 0;
// @d box_node_size=7 {number of words to allocate for a box node}
/// number of words to allocate for a box node
pub(crate) const box_node_size: quarterword = 7;
// @d width_offset=1 {position of |width| field in a box node}
/// position of `width` field in a box node
pub(crate) const width_offset: quarterword = 1;
// @d depth_offset=2 {position of |depth| field in a box node}
/// position of `depth` field in a box node
pub(crate) const depth_offset: quarterword = 2;
// @d height_offset=3 {position of |height| field in a box node}
/// position of `height` field in a box node
pub(crate) const height_offset: quarterword = 3;
// @d width(#) == mem[#+width_offset].sc {width of the box, in sp}
/// width of the box, in sp
pub(crate) macro width($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + crate::section_0135::width_offset as crate::section_0115::pointer]
        [crate::section_0101::MEMORY_WORD_SC]
}
// @d depth(#) == mem[#+depth_offset].sc {depth of the box, in sp}
/// depth of the box, in sp
pub(crate) macro depth($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + crate::section_0135::depth_offset as crate::section_0115::pointer]
        [crate::section_0101::MEMORY_WORD_SC]
}
// @d height(#) == mem[#+height_offset].sc {height of the box, in sp}
/// height of the box, in sp
pub(crate) macro height($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + crate::section_0135::height_offset as crate::section_0115::pointer]
        [crate::section_0101::MEMORY_WORD_SC]
}
// @d shift_amount(#) == mem[#+4].sc {repositioning distance, in sp}
/// repositioning distance, in sp
pub(crate) macro shift_amount($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + 4][crate::section_0101::MEMORY_WORD_SC]
}
// @d list_offset=5 {position of |list_ptr| field in a box node}
/// position of `list_ptr` field in a box node
pub(crate) const list_offset: quarterword = 5;
// @d list_ptr(#) == link(#+list_offset) {beginning of the list inside the box}
/// beginning of the list inside the box
pub(crate) macro list_ptr($globals:expr, $ptr:expr) {
    crate::section_0118::link!(
        $globals,
        $ptr + crate::section_0135::list_offset as crate::section_0115::pointer
    )
}
// @d glue_order(#) == subtype(#+list_offset) {applicable order of infinity}
/// applicable order of infinity
pub(crate) macro glue_order($globals:expr, $ptr:expr) {
    crate::section_0133::subtype!(
        $globals,
        $ptr + crate::section_0135::list_offset as crate::section_0115::pointer
    )
}
// @d glue_sign(#) == type(#+list_offset) {stretching or shrinking}
/// stretching or shrinking
pub(crate) macro glue_sign($globals:expr, $ptr:expr) {
    crate::section_0133::r#type!(
        $globals,
        $ptr + crate::section_0135::list_offset as crate::section_0115::pointer
    )
}
// @d normal=0 {the most common case when several cases are named}
// @d stretching = 1 {glue setting applies to the stretch components}
// @d shrinking = 2 {glue setting applies to the shrink components}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum glue_sign {
    /// the most common case when several cases are named
    normal = 0,
    /// glue setting applies to the stretch components
    stretching = 1,
    /// glue setting applies to the shrink components
    shrinking = 2,
}

impl From<u8> for glue_sign {
    fn from(val: u8) -> Self {
        if val == glue_sign::normal as u8 {
            glue_sign::normal
        } else if val == glue_sign::stretching as u8 {
            glue_sign::stretching
        } else if val == glue_sign::shrinking as u8 {
            glue_sign::shrinking
        } else {
            unreachable!()
        }
    }
}

// @d glue_offset = 6 {position of |glue_set| in a box node}
/// position of `glue_set` in a box node
pub(crate) const glue_offset: quarterword = 6;
// @d glue_set(#) == mem[#+glue_offset].gr
//   {a word of type |glue_ratio| for glue setting}
/// a word of type `glue_ratio` for glue setting
pub(crate) macro glue_set($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + crate::section_0135::glue_offset as crate::section_0115::pointer]
        [crate::section_0113::MEMORY_WORD_GR]
}

#[derive(Copy, Clone)]
pub(crate) enum let_kind {
    /// `normal` value for `let`
    normal = 0,
    /// `futurelet` value for `futurelet`
    futurelet = 1,
}

use crate::section_0113::quarterword;
