//! @ Here is a procedure that is called when the |page_contents| is changing
//! from |empty| to |inserts_only| or |box_there|.
//
// @d set_page_so_far_zero(#)==page_so_far[#]:=0
macro_rules! set_page_so_far_zero {
    ($globals:expr, $idx:expr) => {{
        $globals.page_so_far[$idx] = scaled::zero();
    }}
}

// @p procedure freeze_page_specs(@!s:small_number);
pub(crate) fn freeze_page_specs(globals: &mut TeXGlobals, s: page_contents_kind) {
    // begin page_contents:=s;
    globals.page_contents = s;
    // page_goal:=vsize; page_max_depth:=max_depth;
    page_goal!(globals) = vsize!(globals);
    globals.page_max_depth = max_depth!(globals);
    // page_depth:=0; do_all_six(set_page_so_far_zero);
    page_depth!(globals) = scaled::zero();
    do_all_six!(set_page_so_far_zero !; @globals = globals);
    // least_page_cost:=awful_bad;
    globals.least_page_cost = awful_bad;
    // @!stat if tracing_pages>0 then
    region_stat! {
        if tracing_pages!(globals) > 0 {
            // begin begin_diagnostic;
            begin_diagnostic(globals);
            // print_nl("%% goal height="); print_scaled(page_goal);
            print_nl(globals, strpool_str("%% goal height="));
            print_scaled(globals, page_goal!(globals));
            // @.goal height@>
            // print(", max depth="); print_scaled(page_max_depth);
            print(globals, strpool_str!(", max depth=").get() as _);
            print_scaled(globals, globals.page_max_depth);
            // end_diagnostic(false);
            end_diagnostic(globals, false);
            // end;@;@+tats@;@/
        }
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0833::awful_bad;
use crate::section_0980::page_contents_kind;

