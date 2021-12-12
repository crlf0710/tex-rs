//! @ It is convenient to have a procedure that converts a |math_char|
//! field to an ``unpacked'' form. The |fetch| routine sets |cur_f|, |cur_c|,
//! and |cur_i| to the font code, character code, and character information bytes of
//! a given noad field. It also takes care of issuing error messages for
//! nonexistent characters; in such cases, |char_exists(cur_i)| will be |false|
//! after |fetch| has acted, and the field will also have been reset to |empty|.
//
// @p procedure fetch(@!a:pointer); {unpack the |math_char| field |a|}
/// unpack the `math_char` field `a`
#[allow(unused_variables, unused_assignments)]
pub(crate) fn fetch(globals: &mut TeXGlobals, a: pointer) -> FetchedMathCharInfo {
    let (cur_c, cur_f, cur_i);
    // begin cur_c:=character(a); cur_f:=fam_fnt(fam(a)+cur_size);
    cur_c = character!(globals, a);
    let fam = fam!(globals, a).get();
    cur_f = fam_fnt!(globals, fam + globals.cur_size.get() as u16).into();
    // if cur_f=null_font then
    if cur_f == null_font {
        // @<Complain about an undefined family and set |cur_i| null@>
        todo!("complain");
    }
    // else  begin if (qo(cur_c)>=font_bc[cur_f])and(qo(cur_c)<=font_ec[cur_f]) then
    else {
        if cur_c >= globals.font_bc[cur_f] && cur_c <= globals.font_ec[cur_f] {
            // cur_i:=char_info(cur_f)(cur_c)
            cur_i = char_info!(globals, cur_f, cur_c.numeric_value());
        // else cur_i:=null_character;
        } else {
            cur_i = NULL_CHARACTER;
        }
        // if not(char_exists(cur_i)) then
        if !cur_i.char_exists() {
            // begin char_warning(cur_f,qo(cur_c));
            // math_type(a):=empty; cur_i:=null_character;
            // end;
            todo!("char_warning");
        }
        // end;
    }
    // end;
    FetchedMathCharInfo {
        cur_f,
        cur_c,
        cur_i,
    }
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0134::character;
use crate::section_0230::fam_fnt;
use crate::section_0232::null_font;
use crate::section_0554::char_info;
use crate::section_0556::NULL_CHARACTER;
use crate::section_0681::fam;
use crate::section_0724::FetchedMathCharInfo;
