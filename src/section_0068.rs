//! @ Old versions of \TeX\ needed a procedure called |print_ASCII| whose function
//! is now subsumed by |print|. We retain the old name here as a possible aid to
//! future software arch\ae ologists.

//
// @d print_ASCII == print
#[allow(non_snake_case)]
pub(crate) fn print_ASCII(globals: &mut TeXGlobals, s: integer) {
    print(globals, s);
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
