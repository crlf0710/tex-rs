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
pub(crate) fn rebox(globals: &mut TeXGlobals, mut b: pointer, w: scaled) -> TeXResult<pointer> {
    // var p:pointer; {temporary register for list manipulation}
    /// temporary register for list manipulation
    let mut p;
    // @!f:internal_font_number; {font in a one-character box}
    /// font in a one-character box
    let f;
    // @!v:scaled; {width of a character without italic correction}
    /// width of a character without italic correction
    let v;
    // begin if (width(b)<>w)and(list_ptr(b)<>null) then
    if width!(globals, b) != w && list_ptr!(globals, b) != null {
        // begin if type(b)=vlist_node then b:=hpack(b,natural);
        if r#type!(globals, b) == vlist_node {
            b = hpack(globals, b, natural0!(), natural1!())?;
        }
        // p:=list_ptr(b);
        p = list_ptr!(globals, b);
        // if (is_char_node(p))and(link(p)=null) then
        if is_char_node!(globals, p) && link!(globals, p) == null {
            // begin f:=font(p); v:=char_width(f)(char_info(f)(character(p)));
            f = font!(globals, p);
            let character_p = character!(globals, p);
            v = char_width!(
                globals,
                f,
                char_info!(globals, f, character_p.numeric_value())
            );
            // if v<>width(b) then link(p):=new_kern(width(b)-v);
            if v != width!(globals, b) {
                link!(globals, p) = new_kern(globals, width!(globals, b) - v)?;
            }
            // end;
        }
        // free_node(b,box_node_size);
        free_node(globals, b, box_node_size as _);
        // b:=new_glue(ss_glue); link(b):=p;
        b = new_glue(globals, ss_glue)?;
        link!(globals, b) = p;
        // while link(p)<>null do p:=link(p);
        while link!(globals, p) != null {
            p = link!(globals, p);
        }
        // link(p):=new_glue(ss_glue);
        link!(globals, p) = new_glue(globals, ss_glue)?;
        // rebox:=hpack(b,w,exactly);
        let rebox = hpack(globals, b, w, exactly.into())?;
        // end
        crate::ok_nojump!(rebox)
    }
    // else  begin width(b):=w; rebox:=b;
    else {
        width!(globals, b) = w;
        let rebox = b;
        // end;
        crate::ok_nojump!(rebox)
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0130::free_node;
use crate::section_0133::r#type;
use crate::section_0134::character;
use crate::section_0134::font;
use crate::section_0134::is_char_node;
use crate::section_0135::box_node_size;
use crate::section_0135::list_ptr;
use crate::section_0135::width;
use crate::section_0137::vlist_node;
use crate::section_0153::new_glue;
use crate::section_0156::new_kern;
use crate::section_0162::ss_glue;
use crate::section_0554::char_info;
use crate::section_0554::char_width;
use crate::section_0644::exactly;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0649::hpack;
