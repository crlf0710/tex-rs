//! ` `

// @<Declare act...@>=
// procedure indent_in_hmode;
pub(crate) fn indent_in_hmode(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var p,@!q:pointer;
    // begin if cur_chr>0 then {\.{\\indent}}
    if globals.cur_chr.get() > 0 {
        /// `\indent`
        const _: () = ();

        let (mut p, q);

        // begin p:=new_null_box; width(p):=par_indent;
        p = new_null_box(globals)?;
        width!(globals, p) = par_indent!(globals);
        // if abs(mode)=hmode then space_factor:=1000
        if mode!(globals).get().abs() == hmode {
            space_factor!(globals) = 1000;
        }
        // else  begin q:=new_noad; math_type(nucleus(q)):=sub_box;
        else {
            q = new_noad(globals)?;
            math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
            // info(nucleus(q)):=p; p:=q;
            info_inner!(globals, nucleus!(q)) = p;
            p = q;
            // end;
        }
        // tail_append(p);
        tail_append!(globals, p);
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0118::info_inner;
use crate::section_0135::width;
use crate::section_0136::new_null_box;
use crate::section_0211::hmode;
use crate::section_0213::mode;
use crate::section_0213::space_factor;
use crate::section_0214::tail_append;
use crate::section_0247::par_indent;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0686::new_noad;
