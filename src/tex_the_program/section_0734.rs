//! @ Most of the actual construction work of |mlist_to_hlist| is done
//! by procedures with names
//! like |make_fraction|, |make_radical|, etc. To illustrate
//! the general setup of such procedures, let's begin with a couple of
//! simple ones.

use crate::tex_the_program::section_0681::nucleus;
//
// @<Declare math...@>=
// procedure make_over(@!q:pointer);
pub(crate) fn make_over(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // begin info(nucleus(q)):=@|
    //   overbar(clean_box(nucleus(q),cramped_style(cur_style)),@|
    //   3*default_rule_thickness,default_rule_thickness);
    let b = clean_box(
        globals,
        nucleus!(q),
        cramped_style!(globals.cur_style).into(),
    )?;
    info_inner!(globals, nucleus!(q)) = overbar(
        globals,
        b,
        scaled::new_from_inner(3 * default_rule_thickness!(globals).inner()),
        default_rule_thickness!(globals),
    )?;
    // math_type(nucleus(q)):=sub_box;
    math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0701::default_rule_thickness;
use crate::section_0702::cramped_style;
use crate::section_0705::overbar;
use crate::section_0720::clean_box;
