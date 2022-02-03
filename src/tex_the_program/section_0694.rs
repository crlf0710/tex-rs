//! ` `
// @<Declare procedures needed for displaying...@>=
// procedure print_style(@!c:integer);
pub(crate) fn print_style(globals: &mut TeXGlobals, c: integer) {
    // begin case c div 2 of
    let c_div_2 = c / 2;
    // 0: print_esc("displaystyle"); {|display_style=0|}
    if c_div_2 == 0 {
        /// `display_style=0`
        const _: () = ();
        print_esc(globals, crate::strpool_str!("displaystyle"));
    }
    // 1: print_esc("textstyle"); {|text_style=2|}
    else if c_div_2 == 1 {
        /// `text_style=2`
        const _: () = ();
        print_esc(globals, crate::strpool_str!("textstyle"));
    }
    // 2: print_esc("scriptstyle"); {|script_style=4|}
    else if c_div_2 == 2 {
        /// `script_style=4`
        const _: () = ();
        print_esc(globals, crate::strpool_str!("scriptstyle"));
    }
    // 3: print_esc("scriptscriptstyle"); {|script_script_style=6|}
    else if c_div_2 == 3 {
        /// `script_script_style=6`
        const _: () = ();
        print_esc(globals, crate::strpool_str!("scriptscriptstyle"));
    }
    // othercases print("Unknown style!")
    else {
        print(globals, crate::strpool_str!("Unknown style!").get() as _);
    }
    // endcases;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
