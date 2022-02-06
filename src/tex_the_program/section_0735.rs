//! ` `
// @<Declare math...@>=
// procedure make_under(@!q:pointer);
pub(crate) fn make_under(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // var p,@!x,@!y: pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (p, x, y);
    // @!delta:scaled; {overall height plus depth}
    /// overall height plus depth
    let delta;
    // begin x:=clean_box(nucleus(q),cur_style);
    x = clean_box(globals, nucleus!(q), globals.cur_style)?;
    // p:=new_kern(3*default_rule_thickness); link(x):=p;
    p = new_kern(
        globals,
        scaled::new_from_inner(3 * default_rule_thickness!(globals).inner()),
    )?;
    link!(globals, x) = p;
    // link(p):=fraction_rule(default_rule_thickness);
    link!(globals, p) = fraction_rule(globals, default_rule_thickness!(globals))?;
    // y:=vpack(x,natural);
    y = vpack(globals, x, natural0!(), natural1!())?;
    // delta:=height(y)+depth(y)+default_rule_thickness;
    delta = height!(globals, y) + depth!(globals, y) + default_rule_thickness!(globals);
    // height(y):=height(x); depth(y):=delta-height(y);
    height!(globals, y) = height!(globals, x);
    depth!(globals, y) = delta - height!(globals, y);
    // info(nucleus(q)):=y; math_type(nucleus(q)):=sub_box;
    info_inner!(globals, nucleus!(q)) = y;
    math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0156::new_kern;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0668::vpack;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0701::default_rule_thickness;
use crate::section_0704::fraction_rule;
use crate::section_0720::clean_box;
