//! @ To start a row (i.e., a `row' that rhymes with `dough' but not with `bough'),
//! we enter a new semantic level, copy the first tabskip glue, and change
//! from internal vertical mode to restricted horizontal mode or vice versa.
//! The |space_factor| and |prev_depth| are not used on this semantic level,
//! but we clear them to zero just to be tidy.
//
// @p @t\4@>@<Declare the procedure called |init_span|@>@t@>@/
// procedure init_row;
pub(crate) fn init_row(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin push_nest; mode:=(-hmode-vmode)-mode;
    push_nest(globals);
    mode!(globals) = ((-hmode - vmode) - mode!(globals).get()).into();
    // if mode=-hmode then space_factor:=0 @+else prev_depth:=0;
    if mode!(globals) == -hmode {
        space_factor!(globals) = 0;
    } else {
        prev_depth!(globals) = scaled::zero();
    }
    // tail_append(new_glue(glue_ptr(preamble)));
    tail_append!(
        globals,
        new_glue(globals, glue_ptr!(globals, preamble!(globals)))?
    );
    // subtype(tail):=tab_skip_code+1;@/
    subtype!(globals, tail!(globals)) = tab_skip_code + 1;
    // cur_align:=link(preamble); cur_tail:=cur_head; init_span(cur_align);
    globals.cur_align = link!(globals, preamble!(globals));
    globals.cur_tail = globals.cur_head;
    init_span(globals, globals.cur_align)?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0118::link;
use crate::section_0133::subtype;
use crate::section_0149::glue_ptr;
use crate::section_0153::new_glue;
use crate::section_0211::hmode;
use crate::section_0211::vmode;
use crate::section_0213::mode;
use crate::section_0213::prev_depth;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0216::push_nest;
use crate::section_0224::tab_skip_code;
use crate::section_0770::preamble;
use crate::section_0787::init_span;
