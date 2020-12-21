//! @ The |unsave| routine goes the other way, taking items off of |save_stack|.
//! This routine takes care of restoration when a level ends; everything
//! belonging to the topmost group is cleared off of the save stack.
//
// @p@t\4@>@<Declare the procedure called |restore_trace|@>@;@/
// procedure@?back_input; forward; @t\2@>
// procedure unsave; {pops the top level off the save stack}
/// pops the top level off the save stack
pub(crate) fn unsave(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done;
    // var p:pointer; {position to be restored}
    // @!l:quarterword; {saved level, if in fullword regions of |eqtb|}
    // @!t:halfword; {saved value of |cur_tok|}
    // begin if cur_level>level_one then
    if globals.cur_level > level_one {
        // begin decr(cur_level);
        decr!(globals.cur_level);
        // @<Clear off top level from |save_stack|@>;
        Clear_off_top_level_from_save_stack!(globals);
        // end
    }
    // else confusion("curlevel"); {|unsave| is not used when |cur_group=bottom_level|}
    else {
        /// `unsave` is not used when `cur_group=bottom_level`
        confusion(globals, strpool_str!("curlevel"))?;
    }
    // @:this can't happen curlevel}{\quad curlevel@>
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0221::level_one;