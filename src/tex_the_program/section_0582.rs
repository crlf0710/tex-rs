//! @ Here is a function that returns a pointer to a character node for a
//! given character in a given font. If that character doesn't exist,
//! |null| is returned instead.

//
// @p function new_character(@!f:internal_font_number;@!c:eight_bits):pointer;
pub(crate) fn new_character(
    globals: &mut TeXGlobals,
    f: internal_font_number,
    c: ASCII_code,
) -> pointer {
    // label exit;
    // var p:pointer; {newly allocated node}
    // begin if font_bc[f]<=c then if font_ec[f]>=c then
    if globals.font_bc[f] <= c && globals.font_ec[f] >= c {
        // if char_exists(char_info(f)(qi(c))) then
        if char_info!(globals, f, c.numeric_value()).char_exists() {
            /// newly allocated node
            let p: pointer;

            // begin p:=get_avail; font(p):=f; character(p):=qi(c);
            p = get_avail(globals);
            assign_font_and_character!(globals, p, f, c);
            // new_character:=p; return;
            return p;
            // end;
        }
    }
    // char_warning(f,c);
    char_warning(globals, f, c);
    // new_character:=null;
    return null;
    // exit:end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0134::assign_font_and_character;
use crate::section_0548::internal_font_number;
use crate::section_0554::char_info;
use crate::section_0581::char_warning;
