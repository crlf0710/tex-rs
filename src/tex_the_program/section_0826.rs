//! ` `
// @<Declare subprocedures for |line_break|@>=
// function finite_shrink(@!p:pointer):pointer; {recovers from infinite shrinkage}
/// recovers from infinite shrinkage
pub(crate) fn finite_shrink(globals: &mut TeXGlobals, p: pointer) -> TeXResult<pointer> {
    // var q:pointer; {new glue specification}
    /// new glue specification
    let q: pointer;
    // begin if no_shrink_error_yet then
    if globals.no_shrink_error_yet {
        // begin no_shrink_error_yet:=false;
        globals.no_shrink_error_yet = false;
        // print_err("Infinite glue shrinkage found in a paragraph");
        print_err!(
            globals,
            crate::strpool_str!("Infinite glue shrinkage found in a paragraph")
        );
        // @.Infinite glue shrinkage...@>
        // help5("The paragraph just ended includes some glue that has")@/
        // ("infinite shrinkability, e.g., `\hskip 0pt minus 1fil'.")@/
        // ("Such glue doesn't belong there---it allows a paragraph")@/
        // ("of any length to fit on one line. But it's safe to proceed,")@/
        // ("since the offensive shrinkability has been made finite.");
        help5!(
            globals,
            crate::strpool_str!("The paragraph just ended includes some glue that has"),
            crate::strpool_str!("infinite shrinkability, e.g., `\\hskip 0pt minus 1fil'."),
            crate::strpool_str!("Such glue doesn't belong there---it allows a paragraph"),
            crate::strpool_str!("of any length to fit on one line. But it's safe to proceed,"),
            crate::strpool_str!("since the offensive shrinkability has been made finite.")
        );
        // error;
        error(globals)?;
        // end;
    }
    // q:=new_spec(p); shrink_order(q):=normal;
    q = new_spec(globals, p)?;
    shrink_order!(globals, p) = glue_ord::normal as _;
    // delete_glue_ref(p); finite_shrink:=q;
    delete_glue_ref(globals, p);
    crate::ok_nojump!(q)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0073::print_err;
use crate::section_0079::help5;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
use crate::section_0115::pointer;
use crate::section_0150::glue_ord;
use crate::section_0150::shrink_order;
use crate::section_0151::new_spec;
use crate::section_0201::delete_glue_ref;
