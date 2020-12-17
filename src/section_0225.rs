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
    else if n == line_skip_code as _ {
        print_esc(globals, strpool_str!("lineskip"));
    }
    // baseline_skip_code: print_esc("baselineskip");
    // par_skip_code: print_esc("parskip");
    // above_display_skip_code: print_esc("abovedisplayskip");
    // below_display_skip_code: print_esc("belowdisplayskip");
    // above_display_short_skip_code: print_esc("abovedisplayshortskip");
    // below_display_short_skip_code: print_esc("belowdisplayshortskip");
    // left_skip_code: print_esc("leftskip");
    // right_skip_code: print_esc("rightskip");
    // top_skip_code: print_esc("topskip");
    // split_top_skip_code: print_esc("splittopskip");
    // tab_skip_code: print_esc("tabskip");
    // space_skip_code: print_esc("spaceskip");
    // xspace_skip_code: print_esc("xspaceskip");
    // par_fill_skip_code: print_esc("parfillskip");
    // thin_mu_skip_code: print_esc("thinmuskip");
    // med_mu_skip_code: print_esc("medmuskip");
    // thick_mu_skip_code: print_esc("thickmuskip");
    // othercases print("[unknown glue parameter!]")
    else {
        print(globals, strpool_str!("[unknown glue parameter!]").get() as _);
    }
    // endcases;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0224::line_skip_code;
