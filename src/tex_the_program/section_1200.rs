//! ` `
// @<Declare act...@>=
// procedure resume_after_display;
pub(crate) fn resume_after_display(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if cur_group<>math_shift_group then confusion("display");
    if globals.cur_group != math_shift_group {
        confusion(globals, crate::strpool_str!("display"))?;
    }
    // @:this can't happen display}{\quad display@>
    // unsave; prev_graf:=prev_graf+3;
    unsave(globals)?;
    prev_graf!(globals) += 3;
    // push_nest; mode:=hmode; space_factor:=1000; set_cur_lang; clang:=cur_lang;
    push_nest(globals);
    mode!(globals) = hmode.into();
    space_factor!(globals) = 1000;
    set_cur_lang!(globals);
    clang!(globals) = globals.cur_lang.numeric_value() as _;
    // prev_graf:=(norm_min(left_hyphen_min)*@'100+norm_min(right_hyphen_min))
    //              *@'200000+cur_lang;
    prev_graf!(globals) = (norm_min(left_hyphen_min!(globals)).get() as integer * 0o100
        + norm_min(right_hyphen_min!(globals)).get() as integer)
        * 0o200000
        + globals.cur_lang.numeric_value() as integer;
    // @<Scan an optional space@>;
    crate::section_0443::Scan_an_optional_space!(globals);
    // if nest_ptr=1 then build_page;
    if globals.nest_ptr == 1 {
        build_page(globals)?;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0211::hmode;
use crate::section_0213::clang;
use crate::section_0213::mode;
use crate::section_0213::prev_graf;
use crate::section_0213::space_factor;
use crate::section_0216::push_nest;
use crate::section_0236::left_hyphen_min;
use crate::section_0236::right_hyphen_min;
use crate::section_0269::math_shift_group;
use crate::section_0281::unsave;
use crate::section_0934::set_cur_lang;
use crate::section_0994::build_page;
use crate::section_1091::norm_min;
