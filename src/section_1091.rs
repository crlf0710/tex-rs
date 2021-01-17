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
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn new_graf(globals: &mut TeXGlobals, indented: boolean) {
    // begin prev_graf:=0;
    // if (mode=vmode)or(head<>tail) then
    //   tail_append(new_param_glue(par_skip_code));
    // push_nest; mode:=hmode; space_factor:=1000; set_cur_lang; clang:=cur_lang;
    mode!(globals) = hmode.into();
    // prev_graf:=(norm_min(left_hyphen_min)*@'100+norm_min(right_hyphen_min))
    //              *@'200000+cur_lang;
    // if indented then
    //   begin tail:=new_null_box; link(head):=tail; width(tail):=par_indent;@+
    //   end;
    // if every_par<>null then begin_token_list(every_par,every_par_text);
    // if nest_ptr=1 then build_page; {put |par_skip| glue on current page}
    // end;
}

use crate::section_0211::hmode;

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0101::small_number;