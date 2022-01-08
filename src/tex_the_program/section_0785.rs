//! @ The tricky part about alignments is getting the templates into the
//! scanner at the right time, and recovering control when a row or column
//! is finished.
//!
//! We usually begin a row after each \.{\\cr} has been sensed, unless that
//! \.{\\cr} is followed by \.{\\noalign} or by the right brace that terminates
//! the alignment. The |align_peek| routine is used to look ahead and do
//! the right thing; it either gets a new row started, or gets a \.{\\noalign}
//! started, or finishes off the alignment.
//
// @<Declare the procedure called |align_peek|@>=
// procedure align_peek;
pub(crate) fn align_peek(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label restart;
    // begin restart: align_state:=1000000; @<Get the next non-blank non-call token@>;
    globals.align_state = 1000000;
    crate::section_0406::Get_the_next_non_blank_non_call_token!(globals);
    // if cur_cmd=no_align then
    if globals.cur_cmd == no_align {
        // begin scan_left_brace; new_save_level(no_align_group);
        scan_left_brace(globals)?;
        new_save_level(globals, no_align_group.into());
        // if mode=-vmode then normal_paragraph;
        if mode!(globals) == -vmode {
            normal_paragraph(globals)?;
        }
        // end
    }
    // else if cur_cmd=right_brace then fin_align
    else if globals.cur_cmd == right_brace {
        fin_align(globals)?;
    }
    // else if (cur_cmd=car_ret)and(cur_chr=cr_cr_code) then
    else if globals.cur_cmd == car_ret
        && globals.cur_chr.get() as integer == cr_cr_code as integer
    {
        // goto restart {ignore \.{\\crcr}}
        todo!("align_peek 3");
    }
    // else  begin init_row; {start a new row}
    else {
        /// start a new row
        init_row(globals)?;
        // init_col; {start a new column and replace what we peeked at}
        /// start a new column and replace what we peeked at
        init_col(globals);
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0207::car_ret;
use crate::section_0207::right_brace;
use crate::section_0208::no_align;
use crate::section_0211::vmode;
use crate::section_0213::mode;
use crate::section_0269::no_align_group;
use crate::section_0274::new_save_level;
use crate::section_0403::scan_left_brace;
use crate::section_0780::cr_cr_code;
use crate::section_0786::init_row;
use crate::section_0788::init_col;
use crate::section_0800::fin_align;
use crate::section_1070::normal_paragraph;
