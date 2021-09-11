//! ` `
// @<Declare subprocedures for |prefixed_command|@>=
// procedure alter_box_dimen;
pub(crate) fn alter_box_dimen(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:small_number; {|width_offset| or |height_offset| or |depth_offset|}
    /// `width_offset` or `height_offset` or `depth_offset`
    let c: small_number;
    // @!b:eight_bits; {box number}
    /// box number
    let b: eight_bits;
    // begin c:=cur_chr; scan_eight_bit_int; b:=cur_val; scan_optional_equals;
    c = small_number::new(globals.cur_chr.get() as _);
    scan_eight_bit_int(globals)?;
    b = globals.cur_val as _;
    scan_optional_equals(globals)?;
    // scan_normal_dimen;
    scan_normal_dimen!(globals)?;
    // if box(b)<>null then mem[box(b)+c].sc:=cur_val;
    if r#box!(globals, b) != null {
        globals.mem[r#box!(globals, b) as pointer + c.get() as pointer][MEMORY_WORD_SC] =
            scaled::new_from_inner(globals.cur_val);
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0025::eight_bits;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0101::MEMORY_WORD_SC;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0230::r#box;
use crate::section_0405::scan_optional_equals;
use crate::section_0433::scan_eight_bit_int;
use crate::section_0448::scan_normal_dimen;
