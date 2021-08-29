//! @ The counterpart of |eq_define| for the remaining (fullword) positions in
//! |eqtb| is called |eq_word_define|. Since |xeq_level[p]>=level_one| for all
//! |p|, a `|restore_zero|' will never be used in this case.

// @p procedure eq_word_define(@!p:pointer;@!w:integer);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn eq_word_define(globals: &mut TeXGlobals, p:pointer, w:integer) {
    // begin if xeq_level[p]<>cur_level then
    if globals.xeq_level[p] != globals.cur_level {
        // begin eq_save(p,xeq_level[p]); xeq_level[p]:=cur_level;
        eq_save(globals, p, globals.xeq_level[p]);
        globals.xeq_level[p] = globals.cur_level;
        // end;
    }
    // eqtb[p].int:=w;
    globals.eqtb[p][MEMORY_WORD_INT] = w;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0113::MEMORY_WORD_INT;
use crate::section_0115::pointer;
use crate::section_0276::eq_save;