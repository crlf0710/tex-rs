//! @ The |make_left_right| function constructs a left or right delimiter of
//! the required size and returns the value |open_noad| or |close_noad|. The
//! |right_noad| and |left_noad| will both be based on the original |style|,
//! so they will have consistent sizes.
//!
//! We use the fact that |right_noad-left_noad=close_noad-open_noad|.
//
// @<Declare math...@>=
// function make_left_right(@!q:pointer;@!style:small_number;
//   @!max_d,@!max_h:scaled):small_number;
pub(crate) fn make_left_right(
    globals: &mut TeXGlobals,
    q: pointer,
    style: small_number,
    max_d: scaled,
    max_h: scaled,
) -> TeXResult<small_number> {
    // var delta,@!delta1,@!delta2:scaled; {dimensions used in the calculation}
    /// dimensions used in the calculation
    let (mut delta, mut delta1, mut delta2);
    // begin if style<script_style then cur_size:=text_size
    if style.get() < style_node_subtype::script_style.get() {
        globals.cur_size = text_size.into();
    }
    // else cur_size:=16*((style-text_style) div 2);
    else {
        globals.cur_size = (16 * ((style.get() - style_node_subtype::text_style.get()) / 2)).into();
    }
    // delta2:=max_d+axis_height(cur_size);
    delta2 = max_d + axis_height!(globals, globals.cur_size.get());
    // delta1:=max_h+max_d-delta2;
    delta1 = max_h + max_d - delta2;
    // if delta2>delta1 then delta1:=delta2; {|delta1| is max distance from axis}
    if delta2 > delta1 {
        delta1 = delta2;
    }
    /// `delta1` is max distance from axis
    const _: () = ();
    // delta:=(delta1 div 500)*delimiter_factor;
    delta = scaled::new_from_inner((delta1.inner() / 500) * delimiter_factor!(globals));
    // delta2:=delta1+delta1-delimiter_shortfall;
    delta2 = delta1 + delta1 - delimiter_shortfall!(globals);
    // if delta<delta2 then delta:=delta2;
    if delta < delta2 {
        delta = delta2;
    }
    // new_hlist(q):=var_delimiter(delimiter(q),cur_size,delta);
    new_hlist!(globals, q) = var_delimiter(globals, delimiter!(q), globals.cur_size, delta)? as _;
    // make_left_right:=type(q)-(left_noad-open_noad); {|open_noad| or |close_noad|}
    /// `open_noad` or `close_noad`
    let make_left_right = (r#type!(globals, q) - (left_noad - open_noad)).into();
    // end;
    crate::ok_nojump!(make_left_right)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0133::r#type;
use crate::section_0236::delimiter_factor;
use crate::section_0247::delimiter_shortfall;
use crate::section_0682::open_noad;
use crate::section_0687::delimiter;
use crate::section_0687::left_noad;
use crate::section_0688::style_node_subtype;
use crate::section_0699::text_size;
use crate::section_0700::axis_height;
use crate::section_0706::var_delimiter;
use crate::section_0725::new_hlist;
