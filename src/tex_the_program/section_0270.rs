//! @ The global variable |cur_group| keeps track of what sort of group we are
//! currently in. Another global variable, |cur_boundary|, points to the
//! topmost |level_boundary| word.  And |cur_level| is the current depth of
//! nesting. The routines are designed to preserve the condition that no entry
//! in the |save_stack| or in |eqtb| ever has a level greater than |cur_level|.
//!
