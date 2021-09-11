//! @ Sometimes we need to convert \TeX's internal code numbers into symbolic
//! form. The |print_skip_param| routine gives the symbolic name of a glue
//! parameter.
//
// @<Declare the procedure called |print_skip_param|@>=
// procedure print_skip_param(@!n:integer);
pub(crate) fn print_skip_param(globals: &mut TeXGlobals, n: integer) {
    // begin case n of
    if false {
        unreachable!();
    }
    // line_skip_code: print_esc("lineskip");
    else if n == line_skip_code as integer {
        print_esc(globals, crate::strpool_str!("lineskip"));
    }
    // baseline_skip_code: print_esc("baselineskip");
    else if n == baseline_skip_code as integer {
        print_esc(globals, crate::strpool_str!("baselineskip"));
    }
    // par_skip_code: print_esc("parskip");
    else if n == par_skip_code as integer {
        print_esc(globals, crate::strpool_str!("parskip"));
    }
    // above_display_skip_code: print_esc("abovedisplayskip");
    else if n == above_display_skip_code as integer {
        print_esc(globals, crate::strpool_str!("abovedisplayskip"));
    }
    // below_display_skip_code: print_esc("belowdisplayskip");
    else if n == below_display_skip_code as integer {
        print_esc(globals, crate::strpool_str!("belowdisplayskip"));
    }
    // above_display_short_skip_code: print_esc("abovedisplayshortskip");
    else if n == above_display_short_skip_code as integer {
        print_esc(globals, crate::strpool_str!("abovedisplayshortskip"));
    }
    // below_display_short_skip_code: print_esc("belowdisplayshortskip");
    else if n == below_display_short_skip_code as integer {
        print_esc(globals, crate::strpool_str!("belowdisplayshortskip"));
    }
    // left_skip_code: print_esc("leftskip");
    else if n == left_skip_code as integer {
        print_esc(globals, crate::strpool_str!("leftskip"));
    }
    // right_skip_code: print_esc("rightskip");
    else if n == right_skip_code as integer {
        print_esc(globals, crate::strpool_str!("rightskip"));
    }
    // top_skip_code: print_esc("topskip");
    else if n == top_skip_code as integer {
        print_esc(globals, crate::strpool_str!("topskip"));
    }
    // split_top_skip_code: print_esc("splittopskip");
    else if n == split_top_skip_code as integer {
        print_esc(globals, crate::strpool_str!("splittopskip"));
    }
    // tab_skip_code: print_esc("tabskip");
    else if n == tab_skip_code as integer {
        print_esc(globals, crate::strpool_str!("tabskip"));
    }
    // space_skip_code: print_esc("spaceskip");
    else if n == space_skip_code as integer {
        print_esc(globals, crate::strpool_str!("spaceskip"));
    }
    // xspace_skip_code: print_esc("xspaceskip");
    else if n == xspace_skip_code as integer {
        print_esc(globals, crate::strpool_str!("xspaceskip"));
    }
    // par_fill_skip_code: print_esc("parfillskip");
    else if n == par_fill_skip_code as integer {
        print_esc(globals, crate::strpool_str!("parfillskip"));
    }
    // thin_mu_skip_code: print_esc("thinmuskip");
    else if n == thin_mu_skip_code as integer {
        print_esc(globals, crate::strpool_str!("thinmuskip"));
    }
    // med_mu_skip_code: print_esc("medmuskip");
    else if n == med_mu_skip_code as integer {
        print_esc(globals, crate::strpool_str!("medmuskip"));
    }
    // thick_mu_skip_code: print_esc("thickmuskip");
    else if n == thick_mu_skip_code as integer {
        print_esc(globals, crate::strpool_str!("thickmuskip"));
    }
    // othercases print("[unknown glue parameter!]")
    else {
        crate::trace_error_expr!("n = {}", n);
        print(
            globals,
            crate::strpool_str!("[unknown glue parameter!]").get() as _,
        );
    }
    // endcases;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0224::*;
