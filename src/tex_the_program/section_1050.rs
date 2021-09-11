//! ` `
// @<Declare act...@>=
// procedure report_illegal_case;
#[allow(unused_variables)]
pub(crate) fn report_illegal_case(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin you_cant;
    you_cant(globals);
    // help4("Sorry, but I'm not programmed to handle this case;")@/
    // ("I'll just pretend that you didn't ask for it.")@/
    // ("If you're in the wrong mode, you might be able to")@/
    // ("return to the right one by typing `I}' or `I$' or `I\par'.");@/
    help4!(
        globals,
        crate::strpool_str!("Sorry, but I'm not programmed to handle this case;"),
        crate::strpool_str!("I'll just pretend that you didn't ask for it."),
        crate::strpool_str!("If you're in the wrong mode, you might be able to"),
        crate::strpool_str!("return to the right one by typing `I}' or `I$' or `I\\par'.")
    );
    // error;
    error(globals)?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0079::help4;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_1049::you_cant;
