//! @ When a box is being appended to the current vertical list, the
//! baselineskip calculation is handled by the |append_to_vlist| routine.
//
// @p procedure append_to_vlist(@!b:pointer);
#[allow(unused_variables)]
pub(crate) fn append_to_vlist(globals: &mut TeXGlobals, b: pointer) -> TeXResult<()> {
    // var d:scaled; {deficiency of space between baselines}
    // @!p:pointer; {a new glue node}
    // begin if prev_depth>ignore_depth then
    if prev_depth!(globals) > ignore_depth {
        /// deficiency of space between baselines
        let d: scaled;
        /// a new glue node
        let p: pointer;
        // begin d:=width(baseline_skip)-prev_depth-height(b);
        d = width!(globals, baseline_skip!(globals)) - prev_depth!(globals) - height!(globals, b);
        // if d<line_skip_limit then p:=new_param_glue(line_skip_code)
        if d < line_skip_limit!(globals) {
            p = new_param_glue(globals, line_skip_code.into())?;
        }
        // else  begin p:=new_skip_param(baseline_skip_code);
        else {
            p = new_skip_param(globals, baseline_skip_code.into())?;
            // width(temp_ptr):=d; {|temp_ptr=glue_ptr(p)|}
            /// `temp_ptr=glue_ptr(p)`
            const _: () = ();
            width!(globals, globals.temp_ptr) = d;
            // end;
        }
        // link(tail):=p; tail:=p;
        link!(globals, tail!(globals)) = p;
        tail!(globals) = p;
        // end;
    }
    // link(tail):=b; tail:=b; prev_depth:=depth(b);
    link!(globals, tail!(globals)) = b;
    tail!(globals) = b;
    prev_depth!(globals) = depth!(globals, b);
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::width;
use crate::section_0152::new_param_glue;
use crate::section_0154::new_skip_param;
use crate::section_0212::ignore_depth;
use crate::section_0213::prev_depth;
use crate::section_0213::tail;
use crate::section_0224::baseline_skip;
use crate::section_0224::baseline_skip_code;
use crate::section_0224::line_skip_code;
use crate::section_0247::line_skip_limit;
