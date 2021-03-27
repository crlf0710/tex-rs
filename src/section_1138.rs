//! ` `
// @<Declare act...@>=
// procedure init_math;
pub(crate) fn init_math(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label reswitch,found,not_found,done;
    // var w:scaled; {new or partial |pre_display_size|}
    // @!l:scaled; {new |display_width|}
    // @!s:scaled; {new |display_indent|}
    // @!p:pointer; {current node when calculating |pre_display_size|}
    // @!q:pointer; {glue specification when calculating |pre_display_size|}
    // @!f:internal_font_number; {font in current |char_node|}
    // @!n:integer; {scope of paragraph shape specification}
    // @!v:scaled; {|w| plus possible glue amount}
    // @!d:scaled; {increment to |v|}
    // begin get_token; {|get_x_token| would fail on \.{\\ifmmode}\thinspace!}
    /// `get_x_token` would fail on `\ifmmode`!
    const _ : () = ();
    get_token(globals)?;
    // if (cur_cmd=math_shift)and(mode>0) then @<Go into display math mode@>
    if globals.cur_cmd == math_shift && mode!(globals) > 0 {
        todo!("go into display math mode");
    }
    // else  begin back_input; @<Go into ordinary math mode@>;
    else {
        back_input(globals);
        Go_into_ordinary_math_mode!(globals);
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0207::math_shift;
use crate::section_0325::back_input;
use crate::section_0365::get_token;