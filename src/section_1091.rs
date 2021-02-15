//! ` `

// @<Declare act...@>=
// function norm_min(@!h:integer):small_number;
pub(crate) fn norm_min(h: integer) -> small_number {
    // begin if h<=0 then norm_min:=1@+else if h>=63 then norm_min:=63@+
    if h <= 0 {
        return 1.into();
    } else if h >= 63 {
        return 63.into();
    }
    // else norm_min:=h;
    else {
        return small_number::new(h as _);
    }
    // end;
}
// @#

// procedure new_graf(@!indented:boolean);
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn new_graf(globals: &mut TeXGlobals, indented: boolean) -> TeXResult<()> {
    // begin prev_graf:=0;
    prev_graf!(globals) = 0;
    // if (mode=vmode)or(head<>tail) then
    if mode!(globals).get() as integer == vmode as integer || head!(globals) != tail!(globals) {
        // tail_append(new_param_glue(par_skip_code));
        tail_append!(globals, new_param_glue(globals, par_skip_code.into())?);
    }
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
    // if indented then
    if indented {
        // begin tail:=new_null_box; link(head):=tail; width(tail):=par_indent;@+
        tail!(globals) = new_null_box(globals)?;
        link!(globals, head!(globals)) = tail!(globals);
        width!(globals, tail!(globals)) = par_indent!(globals);
        // end;
    }
    // if every_par<>null then begin_token_list(every_par,every_par_text);
    if every_par!(globals) != null {
        begin_token_list(globals, every_par!(globals), every_par_text);
    }
    // if nest_ptr=1 then build_page; {put |par_skip| glue on current page}
    if globals.nest_ptr == 1 {
        /// put `par_skip` glue on current page
        build_page(globals)?;
    }
    // end;
    ok_nojump!()
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::null;
use crate::section_0136::new_null_box;
use crate::section_0152::new_param_glue;
use crate::section_0211::hmode;
use crate::section_0211::vmode;
use crate::section_0216::push_nest;
use crate::section_0224::par_skip_code;
use crate::section_0307::every_par_text;
use crate::section_0323::begin_token_list;
use crate::section_0994::build_page;
