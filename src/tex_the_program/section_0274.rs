//! @ Procedure |new_save_level| is called when a group begins. The
//! argument is a group identification code like `|hbox_group|'. After
//! calling this routine, it is safe to put five more entries on |save_stack|.
//!
//! In some cases integer-valued items are placed onto the
//! |save_stack| just below a |level_boundary| word, because this is a
//! convenient place to keep information that is supposed to ``pop up'' just
//! when the group has finished.
//! For example, when `\.{\\hbox to 100pt}\grp' is being treated, the 100pt
//! dimension is stored on |save_stack| just before |new_save_level| is
//! called.
//!
//! We use the notation |saved(k)| to stand for an integer item that
//! appears in location |save_ptr+k| of the save stack.
//
// @d saved(#)==save_stack[save_ptr+#].int
pub(crate) macro saved {
    ($globals:expr, $ptr:expr) => {
        $globals.save_stack[$globals.save_ptr + $ptr][crate::section_0113::MEMORY_WORD_INT]
    },
    ($globals:expr, @neg $ptr:expr) => {
        $globals.save_stack[$globals.save_ptr - $ptr][crate::section_0113::MEMORY_WORD_INT]
    }
}
//
// @p procedure new_save_level(@!c:group_code); {begin a new level of grouping}
/// begin a new level of grouping
pub(crate) fn new_save_level(globals: &mut TeXGlobals, c: group_code) {
    // begin check_full_save_stack;
    check_full_save_stack!(globals);
    // save_type(save_ptr):=level_boundary; save_level(save_ptr):=cur_group;
    // save_index(save_ptr):=cur_boundary;
    save_type!(globals, globals.save_ptr) = level_boundary;
    save_level!(globals, globals.save_ptr) = globals.cur_group.get();
    save_index!(globals, globals.save_ptr) = globals.cur_boundary.get();
    // if cur_level=max_quarterword then overflow("grouping levels",
    // @:TeX capacity exceeded grouping levels}{\quad grouping levels@>
    //   max_quarterword-min_quarterword);
    //   {quit if |(cur_level+1)| is too big to be stored in |eqtb|}
    if globals.cur_level == max_quarterword {
        todo!("overflow");
    }
    // cur_boundary:=save_ptr; incr(cur_level); incr(save_ptr); cur_group:=c;
    globals.cur_boundary = globals.save_ptr;
    incr!(globals.cur_level);
    incr!(globals.save_ptr);
    globals.cur_group = c;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0110::max_quarterword;
use crate::section_0268::level_boundary;
use crate::section_0268::save_index;
use crate::section_0268::save_level;
use crate::section_0268::save_type;
use crate::section_0269::group_code;
use crate::section_0273::check_full_save_stack;
