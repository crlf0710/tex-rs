//! @ Here is a subroutine that creates a new box, whose list contains a
//! single character, and whose width includes the italic correction for
//! that character. The height or depth of the box will be negative, if
//! the height or depth of the character is negative; thus, this routine
//! may deliver a slightly different result than |hpack| would produce.
//
// @<Declare subprocedures for |var_delimiter|@>=
// function char_box(@!f:internal_font_number;@!c:quarterword):pointer;
pub(crate) fn char_box(
    globals: &mut TeXGlobals,
    f: internal_font_number,
    c: ASCII_code,
) -> TeXResult<pointer> {
    // var q:four_quarters;
    let q;
    // @!hd:eight_bits; {|height_depth| byte}
    /// `height_depth` byte
    let hd;
    // @!b,@!p:pointer; {the new box and its character node}
    /// the new box and its character node
    let (b, p);
    // begin q:=char_info(f)(c); hd:=height_depth(q);
    q = char_info!(globals, f, c.numeric_value());
    hd = q.height_depth();
    // b:=new_null_box; width(b):=char_width(f)(q)+char_italic(f)(q);
    b = new_null_box(globals)?;
    width!(globals, b) = char_width!(globals, f, q) + char_italic!(globals, f, q);
    // height(b):=char_height(f)(hd); depth(b):=char_depth(f)(hd);
    height!(globals, b) = char_height!(globals, f, hd);
    depth!(globals, b) = char_depth!(globals, f, hd);
    // p:=get_avail; character(p):=c; font(p):=f; list_ptr(b):=p; char_box:=b;
    p = get_avail(globals);
    assign_font_and_character!(globals, p, f, c);
    list_ptr!(globals, b) = p;
    // end;
    crate::ok_nojump!(b)
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0120::get_avail;
use crate::section_0134::assign_font_and_character;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::list_ptr;
use crate::section_0135::width;
use crate::section_0136::new_null_box;
use crate::section_0548::internal_font_number;
use crate::section_0554::char_depth;
use crate::section_0554::char_height;
use crate::section_0554::char_info;
use crate::section_0554::char_italic;
use crate::section_0554::char_width;
