//! ` `
// @<Declare act...@>=
// procedure append_kern;
#[allow(unused_variables)]
pub(crate) fn append_kern(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var s:quarterword; {|subtype| of the kern node}
    /// `subtype` of the kern node
    let s: quarterword;
    // begin s:=cur_chr; scan_dimen(s=mu_glue,false,false);
    s = globals.cur_chr.get() as _;
    scan_dimen(
        globals,
        s == kern_node_subtype::mu_glue as quarterword,
        false,
        false,
    )?;
    // tail_append(new_kern(cur_val)); subtype(tail):=s;
    tail_append!(
        globals,
        new_kern(globals, scaled::new_from_inner(globals.cur_val))?
    );
    subtype!(globals, tail!(globals)) = s;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0113::quarterword;
use crate::section_0133::subtype;
use crate::section_0155::kern_node_subtype;
use crate::section_0156::new_kern;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0448::scan_dimen;
