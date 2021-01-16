//! ` `

// @<Declare action...@>=
// procedure package(@!c:small_number);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn package(globals: &mut TeXGlobals, c: small_number) -> TeXResult<()> {
    // var h:scaled; {height of box}
    // @!p:pointer; {first node in a box}
    // @!d:scaled; {max depth}
    /// max depth
    let d: scaled;
    // begin d:=box_max_depth; unsave; save_ptr:=save_ptr-3;
    d = box_max_depth!(globals);
    unsave(globals)?;
    globals.save_ptr = globals.save_ptr - 3;
    // if mode=-hmode then cur_box:=hpack(link(head),saved(2),saved(1))
    if mode!(globals) == -hmode {
        globals.cur_box = hpack(
            globals,
            link!(globals, head!(globals)),
            scaled::new_from_inner(saved!(globals, 2)),
            small_number::new(saved!(globals, 1) as _),
        )?;
    }
    // else  begin cur_box:=vpackage(link(head),saved(2),saved(1),d);
    else {
        todo!("not -hmode")
        // if c=vtop_code then @<Readjust the height and depth of |cur_box|,
        //   for \.{\\vtop}@>;
        // end;
    }
    // pop_nest; box_end(saved(0));
    pop_nest(globals);
    box_end(globals, saved!(globals, 0))?;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0211::hmode;
use crate::section_0217::pop_nest;
use crate::section_0281::unsave;
use crate::section_0649::hpack;
use crate::section_1075::box_end;
