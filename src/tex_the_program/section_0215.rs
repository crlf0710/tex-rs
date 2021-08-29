//! @ We will see later that the vertical list at the bottom semantic level is split
//! into two parts; the ``current page'' runs from |page_head| to |page_tail|,
//! and the ``contribution list'' runs from |contrib_head| to |tail| of
//! semantic level zero. The idea is that contributions are first formed in
//! vertical mode, then ``contributed'' to the current page (during which time
//! the page-breaking decisions are made). For now, we don't need to know
//! any more details about the page-building process.
//
// @<Set init...@>=
#[distributed_slice(SET_INIT_KEYVAR)]
fn set_initial_values_of_key_variables_0074(globals: &mut TeXGlobals) {
    // nest_ptr:=0; max_nest_stack:=0;
    globals.nest_ptr = 0.into();
    globals.max_nest_stack = 0.into();
    // mode:=vmode; head:=contrib_head; tail:=contrib_head;
    mode!(globals) = vmode.into();
    head!(globals) = contrib_head;
    tail!(globals) = contrib_head;
    // prev_depth:=ignore_depth; mode_line:=0;
    prev_depth!(globals) = ignore_depth;
    mode_line!(globals) = 0;
    // prev_graf:=0; shown_mode:=0;
    prev_graf!(globals) = 0;
    globals.shown_mode = 0.into();
    // @<Start a new current page@>;
    Start_a_new_current_page!(globals);
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::SET_INIT_KEYVAR;
use crate::section_0162::contrib_head;
use crate::section_0211::vmode;
use crate::section_0212::ignore_depth;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
