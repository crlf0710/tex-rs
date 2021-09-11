//! @ The |eq_define| and |eq_word_define| routines take care of local definitions.
//! @^global definitions@>
//! Global definitions are done in almost the same way, but there is no need
//! to save old values, and the new value is associated with |level_one|.
//
// @p procedure geq_define(@!p:pointer;@!t:quarterword;@!e:halfword);
//   {global |eq_define|}
/// global `eq_define`
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn geq_define(
    globals: &mut TeXGlobals,
    p: pointer,
    t: quarterword,
    e: halfword,
) -> TeXResult<()> {
    // begin eq_destroy(eqtb[p]);
    eq_destroy(globals, globals.eqtb[p])?;
    // eq_level(p):=level_one; eq_type(p):=t; equiv(p):=e;
    eq_level!(globals, p) = level_one;
    eq_type!(globals, p) = t;
    equiv!(globals, p) = e;
    // end;
    crate::ok_nojump!()
}

// @#
// procedure geq_word_define(@!p:pointer;@!w:integer); {global |eq_word_define|}
/// global `eq_word_define`
#[allow(unused_variables)]
pub(crate) fn geq_word_define(globals: &mut TeXGlobals, p: pointer, w: integer) {
    // begin eqtb[p].int:=w; xeq_level[p]:=level_one;
    globals.eqtb[p][MEMORY_WORD_INT] = w;
    globals.xeq_level[p] = level_one;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0113::MEMORY_WORD_INT;
use crate::section_0115::pointer;
use crate::section_0221::eq_level;
use crate::section_0221::eq_type;
use crate::section_0221::equiv;
use crate::section_0221::level_one;
use crate::section_0275::eq_destroy;
