//! @ The next subroutine is much simpler; it is used for numerators and
//! denominators of fractions as well as for displayed operators and
//! their limits above and below. It takes a given box~|b| and
//! changes it so that the new box is centered in a box of width~|w|.
//! The centering is done by putting \.{\\hss} glue at the left and right
//! of the list inside |b|, then packaging the new box; thus, the
//! actual box might not really be centered, if it already contains
//! infinite glue.
//!
//! The given box might contain a single character whose italic correction
//! has been added to the width of the box; in this case a compensating
//! kern is inserted.
//
// @p function rebox(@!b:pointer;@!w:scaled):pointer;
pub(crate) fn rebox(globals: &mut TeXGlobals, b: pointer, w: scaled) -> TeXResult<pointer> {
    // var p:pointer; {temporary register for list manipulation}
    // @!f:internal_font_number; {font in a one-character box}
    // @!v:scaled; {width of a character without italic correction}
    // begin if (width(b)<>w)and(list_ptr(b)<>null) then
    if width!(globals, b) != w && list_ptr!(globals, b) != null {
        // begin if type(b)=vlist_node then b:=hpack(b,natural);
        // p:=list_ptr(b);
        // if (is_char_node(p))and(link(p)=null) then
        //   begin f:=font(p); v:=char_width(f)(char_info(f)(character(p)));
        //   if v<>width(b) then link(p):=new_kern(width(b)-v);
        //   end;
        // free_node(b,box_node_size);
        // b:=new_glue(ss_glue); link(b):=p;
        // while link(p)<>null do p:=link(p);
        // link(p):=new_glue(ss_glue);
        // rebox:=hpack(b,w,exactly);
        // end
        todo!("rebox");
    }
    // else  begin width(b):=w; rebox:=b;
    else {
        width!(globals, b) = w;
        // end;
        crate::ok_nojump!(b)
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0135::list_ptr;
use crate::section_0135::width;
