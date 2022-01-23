//! @ When we build an extensible character, it's handy to have the
//! following subroutine, which puts a given character on top
//! of the characters already in box |b|:
//
// @<Declare subprocedures for |var_delimiter|@>=
// procedure stack_into_box(@!b:pointer;@!f:internal_font_number;
//   @!c:quarterword);
pub(crate) fn stack_into_box(
    globals: &mut TeXGlobals,
    b: pointer,
    f: internal_font_number,
    c: ASCII_code,
) -> TeXResult<()> {
    // var p:pointer; {new node placed into |b|}
    /// new node placed into `b`
    let p;
    // begin p:=char_box(f,c); link(p):=list_ptr(b); list_ptr(b):=p;
    p = char_box(globals, f, c)?;
    link!(globals, p) = list_ptr!(globals, b);
    list_ptr!(globals, b) = p;
    // height(b):=height(p);
    height!(globals, b) = height!(globals, p);
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0135::height;
use crate::section_0135::list_ptr;
use crate::section_0548::internal_font_number;
use crate::section_0709::char_box;
