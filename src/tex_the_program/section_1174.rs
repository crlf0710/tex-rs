//! ` `

// @<Declare act...@>=
// @t\4@>@<Declare the function called |fin_mlist|@>@t@>@;@/
// procedure build_choices;
pub(crate) fn build_choices(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // var p:pointer; {the current mlist}
    /// the current mlist
    let p;
    // begin unsave; p:=fin_mlist(null);
    unsave(globals)?;
    p = fin_mlist(globals, null);
    // case saved(-1) of
    let saved_neg_1 = saved!(globals, @neg 1);
    // 0:display_mlist(tail):=p;
    if saved_neg_1 == 0 {
        display_mlist!(globals, tail!(globals)) = p;
    }
    // 1:text_mlist(tail):=p;
    else if saved_neg_1 == 1 {
        text_mlist!(globals, tail!(globals)) = p;
    }
    // 2:script_mlist(tail):=p;
    else if saved_neg_1 == 2 {
        script_mlist!(globals, tail!(globals)) = p;
    }
    // 3:begin script_script_mlist(tail):=p; decr(save_ptr); return;
    else if saved_neg_1 == 3 {
        script_script_mlist!(globals, tail!(globals)) = p;
        decr!(globals.save_ptr);
        crate::return_nojump!()
        // end;
    }
    // end; {there are no other cases}
    else {
        /// there are no other cases
        const _: () = ();
        unreachable!()
    }
    // incr(saved(-1)); push_math(math_choice_group); scan_left_brace;
    incr!(saved!(globals, @neg 1));
    push_math(globals, math_choice_group.into());
    scan_left_brace(globals)?;
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0016::incr;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0213::tail;
use crate::section_0269::math_choice_group;
use crate::section_0274::saved;
use crate::section_0281::unsave;
use crate::section_0403::scan_left_brace;
use crate::section_0689::display_mlist;
use crate::section_0689::script_mlist;
use crate::section_0689::script_script_mlist;
use crate::section_0689::text_mlist;
use crate::section_1136::push_math;
use crate::section_1184::fin_mlist;
