//! @ According to the rules in the \.{DVI} file specifications, we ensure alignment
//! @^square roots@>
//! between a square root sign and the rule above its nucleus by assuming that the
//! baseline of the square-root symbol is the same as the bottom of the rule. The
//! height of the square-root symbol will be the thickness of the rule, and the
//! depth of the square-root symbol should exceed or equal the height-plus-depth
//! of the nucleus plus a certain minimum clearance~|clr|. The symbol will be
//! placed so that the actual clearance is |clr| plus half the excess.
//
// @<Declare math...@>=
// procedure make_radical(@!q:pointer);
pub(crate) fn make_radical(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // var x,@!y:pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (x, y);
    // @!delta,@!clr:scaled; {dimensions involved in the calculation}
    /// dimensions involved in the calculation
    let (delta, mut clr);
    // begin x:=clean_box(nucleus(q),cramped_style(cur_style));
    x = clean_box(
        globals,
        nucleus!(q),
        cramped_style!(globals.cur_style).into(),
    )?;
    // if cur_style<text_style then {display style}
    if globals.cur_style.get() < style_node_subtype::text_style.get() {
        /// display style
        const _: () = ();
        // clr:=default_rule_thickness+(abs(math_x_height(cur_size)) div 4)
        todo!("display style");
    }
    // else  begin clr:=default_rule_thickness; clr:=clr + (abs(clr) div 4);
    else {
        let default_rule_thickness = default_rule_thickness!(globals);
        clr = default_rule_thickness
            + scaled::new_from_inner(default_rule_thickness.abs().inner() / 4);
        // end;
    }
    // y:=var_delimiter(left_delimiter(q),cur_size,height(x)+depth(x)+clr+
    //   default_rule_thickness);
    y = var_delimiter(
        globals,
        left_delimiter!(q),
        globals.cur_size,
        height!(globals, x) + depth!(globals, x) + clr + default_rule_thickness!(globals),
    )?;
    // delta:=depth(y)-(height(x)+depth(x)+clr);
    delta = depth!(globals, y) - (height!(globals, x) + depth!(globals, x) + clr);
    // if delta>0 then clr:=clr+half(delta); {increase the actual clearance}
    if delta > scaled::zero() {
        /// increase the actual clearance
        const _: () = ();
        clr += scaled::new_from_inner(half(delta.inner()));
    }
    // shift_amount(y):=-(height(x)+clr);
    shift_amount!(globals, y) = -(height!(globals, x) + clr);
    // link(y):=overbar(x,clr,height(y));
    link!(globals, y) = overbar(globals, x, clr, height!(globals, y))?;
    // info(nucleus(q)):=hpack(y,natural); math_type(nucleus(q)):=sub_box;
    info_inner!(globals, nucleus!(q)) = hpack(globals, y, natural0!(), natural1!())?;
    math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0100::half;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::shift_amount;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0649::hpack;
use crate::section_0668::vpack;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0683::left_delimiter;
use crate::section_0688::style_node_subtype;
use crate::section_0701::default_rule_thickness;
use crate::section_0702::cramped_style;
use crate::section_0705::overbar;
use crate::section_0706::var_delimiter;
use crate::section_0720::clean_box;
