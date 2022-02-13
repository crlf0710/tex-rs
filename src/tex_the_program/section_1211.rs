//! @ If the user says, e.g., `\.{\\global\\global}', the redundancy is
//! silently accepted.

pub(crate) macro Assignments($globals:expr, $cur_cmd:expr, $a:expr, $lbl_done:lifetime) {{
    crate::trace_span_verbose!("Assignments");
    let processed = false
        || crate::section_1217::Assignments_1217!($globals, $cur_cmd, $a)
        || crate::section_1218::Assignments_1218!($globals, $cur_cmd, $a)
        || crate::section_1221::Assignments_1221!($globals, $cur_cmd, $a)
        || crate::section_1224::Assignments_1224!($globals, $cur_cmd, $a)
        || crate::section_1225::Assignments_1225!($globals, $cur_cmd, $a)
        || crate::section_1226::Assignments_1226!($globals, $cur_cmd, $a)
        || crate::section_1228::Assignments_1228!($globals, $cur_cmd, $a)
        || crate::section_1232::Assignments_1232!($globals, $cur_cmd, $a)
        || crate::section_1234::Assignments_1234!($globals, $cur_cmd, $a)
        || crate::section_1235::Assignments_1235!($globals, $cur_cmd, $a)
        || crate::section_1241::Assignments_1241!($globals, $cur_cmd, $a)
        || crate::section_1242::Assignments_1242!($globals, $cur_cmd, $a)
        || crate::section_1248::Assignments_1248!($globals, $cur_cmd, $a)
        || crate::section_1252::Assignments_1252!($globals, $cur_cmd, $a, $lbl_done)
        || crate::section_1253::Assignments_1253!($globals, $cur_cmd, $a)
        || crate::section_1256::Assignments_1256!($globals, $cur_cmd, $a)
        || crate::section_1264::Assignments_1264!($globals, $cur_cmd, $a);
    processed
}}

// @<Declare act...@>=
// @t\4@>@<Declare subprocedures for |prefixed_command|@>@t@>@;@/
// procedure prefixed_command;
#[allow(unused_variables)]
#[cfg_attr(
    feature = "trace_verbose",
    tracing::instrument(level = "trace", skip(globals))
)]
pub(crate) fn prefixed_command(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done,exit;
    // var a:small_number; {accumulated prefix codes so far}
    // @!f:internal_font_number; {identifies a font}
    // @!j:halfword; {index into a \.{\\parshape} specification}
    // @!k:font_index; {index into |font_info|}
    // @!p,@!q:pointer; {for temporary short-term use}
    // @!n:integer; {ditto}
    // @!e:boolean; {should a definition be expanded? or was \.{\\let} not done?}
    // begin a:=0;
    let mut a = 0;
    // while cur_cmd=prefix do
    while globals.cur_cmd == prefix {
        //   begin if not odd(a div cur_chr) then a:=a+cur_chr;
        if !(a / globals.cur_chr.get()).is_odd() {
            a += globals.cur_chr.get();
        }
        // @<Get the next non-blank non-relax...@>;
        crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!(globals);
        // if cur_cmd<=max_non_prefixed_command then
        if globals.cur_cmd <= max_non_prefixed_command {
            crate::trace_error_expr!("cur_cmd={}", globals.cur_cmd);
            // @<Discard erroneous prefixes and |return|@>;
            crate::section_1212::Discard_erroneous_prefixes_and_return!(globals);
            // end;
        }
    }
    crate::trace_expr_verbose!("cur_cmd={}", globals.cur_cmd);
    // @<Discard the prefixes \.{\\long} and \.{\\outer} if they are irrelevant@>;
    crate::section_1213::Discard_the_prefixes_long_and_outer_if_they_are_irrelevant!(globals, a);
    crate::region_forward_label!(
    |'done|
    {
    // @<Adjust \(f)for the setting of \.{\\globaldefs}@>;
    crate::section_1214::Adjust_f_for_the_setting_of_globaldefs!(globals, a);
    // case cur_cmd of
    // @t\4@>@<Assignments@>@;
    if Assignments!(globals, globals.cur_cmd, a, 'done) {
        /// already processed
        do_nothing!();
    }
    // othercases confusion("prefix")
    else {
        confusion(globals, crate::strpool_str!("prefix"))?;
        // @:this can't happen prefix}{\quad prefix@>
        // endcases;
    }
    // done: @<Insert a token saved by \.{\\afterassignment}, if any@>;
    }
    'done <-
    );
    crate::section_1269::Insert_a_token_saved_by_afterassignment__if_any!(globals);
    // exit:end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::pascal::IsOddOrEven;
use crate::section_0004::TeXGlobals;
use crate::section_0016::do_nothing;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0208::max_non_prefixed_command;
use crate::section_0209::*;
use crate::section_0210::*;
use crate::section_0405::scan_optional_equals;
use crate::section_0440::scan_int;
