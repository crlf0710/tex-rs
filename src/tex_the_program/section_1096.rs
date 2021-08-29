//! ` `

// @<Declare act...@>=
// procedure end_graf;
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn end_graf(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if mode=hmode then
    if mode!(globals) == hmode {
        // begin if head=tail then pop_nest {null paragraphs are ignored}
        if head!(globals) == tail!(globals) {
            /// null paragraphs are ignored
            pop_nest(globals);
        }
        // else line_break(widow_penalty);
        else {
            line_break(globals, widow_penalty!(globals))?;
        }
        // normal_paragraph;
        normal_paragraph(globals)?;
        // error_count:=0;
        globals.error_count = 0.into();
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0211::hmode;
use crate::section_0217::pop_nest;
use crate::section_0815::line_break;
use crate::section_1070::normal_paragraph;
