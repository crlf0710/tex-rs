//! @ The procedure |eq_define| defines an |eqtb| entry having specified
//! |eq_type| and |equiv| fields, and saves the former value if appropriate.
//! This procedure is used only for entries in the first four regions of |eqtb|,
//! i.e., only for entries that have |eq_type| and |equiv| fields.
//! After calling this routine, it is safe to put four more entries on
//! |save_stack|, provided that there was room for four more entries before
//! the call, since |eq_save| makes the necessary test.
//
// @p procedure eq_define(@!p:pointer;@!t:quarterword;@!e:halfword);
//   {new data for |eqtb|}
/// new data for `eqtb`
#[allow(unused_variables)]
pub(crate) fn eq_define(globals: &mut TeXGlobals, p: pointer, t: quarterword, e: halfword) {
    // begin if eq_level(p)=cur_level then eq_destroy(eqtb[p])
    if eq_level!(globals, p) == globals.cur_level {
        eq_destroy(globals, globals.eqtb[p]);
    }
    // else if cur_level>level_one then eq_save(p,eq_level(p));
    else if globals.cur_level > level_one {
        eq_save(globals, p, eq_level!(globals, p));
    }
    // eq_level(p):=cur_level; eq_type(p):=t; equiv(p):=e;
    eq_level!(globals, p) = globals.cur_level;
    eq_type!(globals, p) = t;
    equiv!(globals, p) = e;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0221::level_one;
use crate::section_0275::eq_destroy;
use crate::section_0276::eq_save;


