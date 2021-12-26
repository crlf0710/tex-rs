//! ` `
// @<Declare act...@>=
// procedure append_italic_correction;
pub(crate) fn append_italic_correction(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // var p:pointer; {|char_node| at the tail of the current list}
    /// `char_node` at the tail of the current list
    let p;
    // @!f:internal_font_number; {the font in the |char_node|}
    /// the font in the `char_node`
    let f;
    // begin if tail<>head then
    if tail!(globals) != head!(globals) {
        // begin if is_char_node(tail) then p:=tail
        if is_char_node!(globals, tail!(globals)) {
            p = tail!(globals);
        }
        // else if type(tail)=ligature_node then p:=lig_char(tail)
        else if r#type!(globals, tail!(globals)) == ligature_node {
            p = lig_char!(tail!(globals));
        }
        // else return;
        else {
            crate::return_nojump!();
        }
        // f:=font(p);
        f = font!(globals, p);
        // tail_append(new_kern(char_italic(f)(char_info(f)(character(p)))));
        let p_char_numeric = character!(globals, p).numeric_value();
        let p_char_italic = char_italic!(globals, f, char_info!(globals, f, p_char_numeric));
        tail_append!(globals, new_kern(globals, p_char_italic)?);
        // subtype(tail):=explicit;
        subtype!(globals, tail!(globals)) = kern_node_subtype::explicit as _;
        // end;
    }
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0134::character;
use crate::section_0134::font;
use crate::section_0134::is_char_node;
use crate::section_0143::lig_char;
use crate::section_0143::ligature_node;
use crate::section_0155::kern_node_subtype;
use crate::section_0156::new_kern;
use crate::section_0213::head;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0554::char_info;
use crate::section_0554::char_italic;
