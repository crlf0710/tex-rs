//! ` `
// @<Declare act...@>=
// procedure math_radical;
pub(crate) fn math_radical(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin tail_append(get_node(radical_noad_size));
    tail_append!(globals, get_node(globals, radical_noad_size as _)?);
    // type(tail):=radical_noad; subtype(tail):=normal;
    r#type!(globals, tail!(globals)) = radical_noad;
    subtype!(globals, tail!(globals)) = noad_subtype::normal as _;
    // mem[nucleus(tail)].hh:=empty_field;
    globals.mem[nucleus!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // mem[subscr(tail)].hh:=empty_field;
    globals.mem[subscr!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // mem[supscr(tail)].hh:=empty_field;
    globals.mem[supscr!(tail!(globals))][MEMORY_WORD_HH] = empty_field;
    // scan_delimiter(left_delimiter(tail),true); scan_math(nucleus(tail));
    scan_delimiter(globals, left_delimiter!(tail!(globals)), true)?;
    scan_math(globals, nucleus!(tail!(globals)))?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::MEMORY_WORD_HH;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0681::nucleus;
use crate::section_0681::subscr;
use crate::section_0681::supscr;
use crate::section_0683::left_delimiter;
use crate::section_0683::radical_noad;
use crate::section_0683::radical_noad_size;
use crate::section_0684::empty_field;
use crate::section_0686::noad_subtype;
use crate::section_1151::scan_math;
use crate::section_1160::scan_delimiter;
