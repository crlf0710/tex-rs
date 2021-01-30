//! @* \[19] Saving and restoring equivalents.
//! The nested structure provided by `$\.{\char'173}\ldots\.{\char'175}$' groups
//! in \TeX\ means that |eqtb| entries valid in outer groups should be saved
//! and restored later if they are overridden inside the braces. When a new |eqtb|
//! value is being assigned, the program therefore checks to see if the previous
//! entry belongs to an outer level. In such a case, the old value is placed
//! on the |save_stack| just before the new value enters |eqtb|. At the
//! end of a grouping level, i.e., when the right brace is sensed, the
//! |save_stack| is used to restore the outer values, and the inner ones are
//! destroyed.
//!
//! Entries on the |save_stack| are of type |memory_word|. The top item on
//! this stack is |save_stack[p]|, where |p=save_ptr-1|; it contains three
//! fields called |save_type|, |save_level|, and |save_index|, and it is
//! interpreted in one of four ways:
//!
//! \yskip\hangg 1) If |save_type(p)=restore_old_value|, then
//! |save_index(p)| is a location in |eqtb| whose current value should
//! be destroyed at the end of the current group and replaced by |save_stack[p-1]|.
//! Furthermore if |save_index(p)>=int_base|, then |save_level(p)|
//! should replace the corresponding entry in |xeq_level|.
//!
//! \yskip\hangg 2) If |save_type(p)=restore_zero|, then |save_index(p)|
//! is a location in |eqtb| whose current value should be destroyed at the end
//! of the current group, when it should be
//! replaced by the value of |eqtb[undefined_control_sequence]|.
//!
//! \yskip\hangg 3) If |save_type(p)=insert_token|, then |save_index(p)|
//! is a token that should be inserted into \TeX's input when the current
//! group ends.
//!
//! \yskip\hangg 4) If |save_type(p)=level_boundary|, then |save_level(p)|
//! is a code explaining what kind of group we were previously in, and
//! |save_index(p)| points to the level boundary word at the bottom of
//! the entries for that group.
//
// @d save_type(#)==save_stack[#].hh.b0 {classifies a |save_stack| entry}
/// classifies a `save_stack` entry
macro_rules! save_type {
    ($globals:expr, $ptr:expr) => {
        $globals.save_stack[$ptr][crate::section_0113::MEMORY_WORD_HH_B0]
    }
}
// @d save_level(#)==save_stack[#].hh.b1
//   {saved level for regions 5 and 6, or group code}
/// saved level for regions 5 and 6, or group code
macro_rules! save_level {
    ($globals:expr, $ptr:expr) => {
        $globals.save_stack[$ptr][crate::section_0113::MEMORY_WORD_HH_B1]
    }
}
// @d save_index(#)==save_stack[#].hh.rh
//   {|eqtb| location or token or |save_stack| location}
/// `eqtb` location or token or `save_stack` location
macro_rules! save_index {
    ($globals:expr, $ptr:expr) => {
        $globals.save_stack[$ptr][crate::section_0113::MEMORY_WORD_HH_RH]
    }
}
// @d restore_old_value=0 {|save_type| when a value should be restored later}
/// `save_type` when a value should be restored later
pub(crate) const restore_old_value: quarterword = 0;
// @d restore_zero=1 {|save_type| when an undefined entry should be restored}
/// `save_type` when an undefined entry should be restored
pub(crate) const restore_zero: quarterword = 1;
// @d insert_token=2 {|save_type| when a token is being saved for later use}
/// `save_type` when a token is being saved for later use
pub(crate) const insert_token: quarterword = 2;
// @d level_boundary=3 {|save_type| corresponding to beginning of group}
/// `save_type` corresponding to beginning of group
pub(crate) const level_boundary: quarterword = 3;

use crate::section_0113::quarterword;
