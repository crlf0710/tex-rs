//! @ Another handy subroutine computes the height plus depth of
//! a given character:
//
// @<Declare subprocedures for |var_delimiter|@>=
// function height_plus_depth(@!f:internal_font_number;@!c:quarterword):scaled;
pub(crate) fn height_plus_depth(
    globals: &mut TeXGlobals,
    f: internal_font_number,
    c: ASCII_code,
) -> scaled {
    // var q:four_quarters;
    let q;
    // @!hd:eight_bits; {|height_depth| byte}
    /// `height_depth` byte
    let hd;
    // begin q:=char_info(f)(c); hd:=height_depth(q);
    q = char_info!(globals, f, c.numeric_value());
    hd = q.height_depth();
    // height_plus_depth:=char_height(f)(hd)+char_depth(f)(hd);
    char_height!(globals, f, hd) + char_depth!(globals, f, hd)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0101::scaled;
use crate::section_0548::internal_font_number;
use crate::section_0554::char_depth;
use crate::section_0554::char_height;
use crate::section_0554::char_info;
