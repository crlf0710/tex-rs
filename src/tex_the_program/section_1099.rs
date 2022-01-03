//! ` `
//! @<Declare act...@>=
// procedure begin_insert_or_adjust;
pub(crate) fn begin_insert_or_adjust(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if cur_cmd=vadjust then cur_val:=255
    if globals.cur_cmd == vadjust {
        globals.cur_val = 255;
    }
    // else  begin scan_eight_bit_int;
    else {
        scan_eight_bit_int(globals)?;
        // if cur_val=255 then
        if globals.cur_val == 255 {
            todo!("error");
            // begin print_err("You can't "); print_esc("insert"); print_int(255);
            // @.You can't \\insert255@>
            // help1("I'm changing to \insert0; box 255 is special.");
            // error; cur_val:=0;
            // end;
        }
        // end;
    }
    // saved(0):=cur_val; incr(save_ptr);
    saved!(globals, 0) = globals.cur_val;
    incr!(globals.save_ptr);
    // new_save_level(insert_group); scan_left_brace; normal_paragraph;
    new_save_level(globals, insert_group.into());
    scan_left_brace(globals)?;
    normal_paragraph(globals)?;
    // push_nest; mode:=-vmode; prev_depth:=ignore_depth;
    push_nest(globals);
    mode!(globals) = (-vmode).into();
    prev_depth!(globals) = ignore_depth;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0081::TeXResult;
use crate::section_0208::vadjust;
use crate::section_0211::vmode;
use crate::section_0212::ignore_depth;
use crate::section_0213::mode;
use crate::section_0213::prev_depth;
use crate::section_0216::push_nest;
use crate::section_0269::insert_group;
use crate::section_0274::new_save_level;
use crate::section_0274::saved;
use crate::section_0403::scan_left_brace;
use crate::section_0433::scan_eight_bit_int;
use crate::section_1070::normal_paragraph;
