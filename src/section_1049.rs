//! @ The `|you_cant|' procedure prints a line saying that the current command
//! is illegal in the current mode; it identifies these things symbolically.
//
// @<Declare action...@>=
// procedure you_cant;
pub(crate) fn you_cant(globals: &mut TeXGlobals) {
    // begin print_err("You can't use `");
    print_err!(globals, strpool_str!("You can't use `"));
    // @.You can't use x in y mode@>
    // print_cmd_chr(cur_cmd,cur_chr);
    print_cmd_chr(globals, globals.cur_cmd, globals.cur_chr);
    // print("' in "); print_mode(mode);
    print(globals, strpool_str!("' in ").get() as _);
    print_mode(globals, mode!(globals).get() as _);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0211::print_mode;
use crate::section_0298::print_cmd_chr;
