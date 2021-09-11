//! @ We can print the symbolic name of an integer parameter as follows.
//

// @p procedure print_param(@!n:integer);
#[allow(unused_variables)]
pub(crate) fn print_param(globals: &mut TeXGlobals, n: integer) {
    // begin case n of
    if false {
        unreachable!();
    }
    // pretolerance_code:print_esc("pretolerance");
    else if n == pretolerance_code as integer {
        print_esc(globals, crate::strpool_str!("pretolerance"));
    }
    // tolerance_code:print_esc("tolerance");
    else if n == tolerance_code as integer {
        print_esc(globals, crate::strpool_str!("tolerance"));
    }
    // line_penalty_code:print_esc("linepenalty");
    else if n == line_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("linepenalty"));
    }
    // hyphen_penalty_code:print_esc("hyphenpenalty");
    else if n == hyphen_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("hyphenpenalty"));
    }
    // ex_hyphen_penalty_code:print_esc("exhyphenpenalty");
    else if n == ex_hyphen_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("exhyphenpenalty"));
    }
    // club_penalty_code:print_esc("clubpenalty");
    else if n == club_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("clubpenalty"));
    }
    // widow_penalty_code:print_esc("widowpenalty");
    else if n == widow_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("widowpenalty"));
    }
    // display_widow_penalty_code:print_esc("displaywidowpenalty");
    else if n == display_widow_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("displaywidowpenalty"));
    }
    // broken_penalty_code:print_esc("brokenpenalty");
    else if n == broken_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("brokenpenalty"));
    }
    // bin_op_penalty_code:print_esc("binoppenalty");
    else if n == bin_op_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("binoppenalty"));
    }
    // rel_penalty_code:print_esc("relpenalty");
    else if n == rel_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("relpenalty"));
    }
    // pre_display_penalty_code:print_esc("predisplaypenalty");
    else if n == pre_display_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("predisplaypenalty"));
    }
    // post_display_penalty_code:print_esc("postdisplaypenalty");
    else if n == post_display_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("postdisplaypenalty"));
    }
    // inter_line_penalty_code:print_esc("interlinepenalty");
    else if n == inter_line_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("interlinepenalty"));
    }
    // double_hyphen_demerits_code:print_esc("doublehyphendemerits");
    else if n == double_hyphen_demerits_code as integer {
        print_esc(globals, crate::strpool_str!("doublehyphendemerits"));
    }
    // final_hyphen_demerits_code:print_esc("finalhyphendemerits");
    else if n == final_hyphen_demerits_code as integer {
        print_esc(globals, crate::strpool_str!("finalhyphendemerits"));
    }
    // adj_demerits_code:print_esc("adjdemerits");
    else if n == adj_demerits_code as integer {
        print_esc(globals, crate::strpool_str!("adjdemerits"));
    }
    // mag_code:print_esc("mag");
    else if n == mag_code as integer {
        print_esc(globals, crate::strpool_str!("mag"));
    }
    // delimiter_factor_code:print_esc("delimiterfactor");
    else if n == delimiter_factor_code as integer {
        print_esc(globals, crate::strpool_str!("delimiterfactor"));
    }
    // looseness_code:print_esc("looseness");
    else if n == looseness_code as integer {
        print_esc(globals, crate::strpool_str!("looseness"));
    }
    // time_code:print_esc("time");
    else if n == time_code as integer {
        print_esc(globals, crate::strpool_str!("time"));
    }
    // day_code:print_esc("day");
    else if n == day_code as integer {
        print_esc(globals, crate::strpool_str!("day"));
    }
    // month_code:print_esc("month");
    else if n == month_code as integer {
        print_esc(globals, crate::strpool_str!("month"));
    }
    // year_code:print_esc("year");
    else if n == year_code as integer {
        print_esc(globals, crate::strpool_str!("year"));
    }
    // show_box_breadth_code:print_esc("showboxbreadth");
    else if n == show_box_breadth_code as integer {
        print_esc(globals, crate::strpool_str!("showboxbreadth"));
    }
    // show_box_depth_code:print_esc("showboxdepth");
    else if n == show_box_depth_code as integer {
        print_esc(globals, crate::strpool_str!("showboxdepth"));
    }
    // hbadness_code:print_esc("hbadness");
    else if n == hbadness_code as integer {
        print_esc(globals, crate::strpool_str!("hbadness"));
    }
    // vbadness_code:print_esc("vbadness");
    else if n == vbadness_code as integer {
        print_esc(globals, crate::strpool_str!("vbadness"));
    }
    // pausing_code:print_esc("pausing");
    else if n == pausing_code as integer {
        print_esc(globals, crate::strpool_str!("pausing"));
    }
    // tracing_online_code:print_esc("tracingonline");
    else if n == tracing_online_code as integer {
        print_esc(globals, crate::strpool_str!("tracingonline"));
    }
    // tracing_macros_code:print_esc("tracingmacros");
    else if n == tracing_macros_code as integer {
        print_esc(globals, crate::strpool_str!("tracingmacros"));
    }
    // tracing_stats_code:print_esc("tracingstats");
    else if n == tracing_stats_code as integer {
        print_esc(globals, crate::strpool_str!("tracingstats"));
    }
    // tracing_paragraphs_code:print_esc("tracingparagraphs");
    else if n == tracing_paragraphs_code as integer {
        print_esc(globals, crate::strpool_str!("tracingparagraphs"));
    }
    // tracing_pages_code:print_esc("tracingpages");
    else if n == tracing_pages_code as integer {
        print_esc(globals, crate::strpool_str!("tracingpages"));
    }
    // tracing_output_code:print_esc("tracingoutput");
    else if n == tracing_output_code as integer {
        print_esc(globals, crate::strpool_str!("tracingoutput"));
    }
    // tracing_lost_chars_code:print_esc("tracinglostchars");
    else if n == tracing_lost_chars_code as integer {
        print_esc(globals, crate::strpool_str!("tracinglostchars"));
    }
    // tracing_commands_code:print_esc("tracingcommands");
    else if n == tracing_commands_code as integer {
        print_esc(globals, crate::strpool_str!("tracingcommands"));
    }
    // tracing_restores_code:print_esc("tracingrestores");
    else if n == tracing_restores_code as integer {
        print_esc(globals, crate::strpool_str!("tracingrestores"));
    }
    // uc_hyph_code:print_esc("uchyph");
    else if n == uc_hyph_code as integer {
        print_esc(globals, crate::strpool_str!("uchyph"));
    }
    // output_penalty_code:print_esc("outputpenalty");
    else if n == output_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("outputpenalty"));
    }
    // max_dead_cycles_code:print_esc("maxdeadcycles");
    else if n == max_dead_cycles_code as integer {
        print_esc(globals, crate::strpool_str!("maxdeadcycles"));
    }
    // hang_after_code:print_esc("hangafter");
    else if n == hang_after_code as integer {
        print_esc(globals, crate::strpool_str!("hangafter"));
    }
    // floating_penalty_code:print_esc("floatingpenalty");
    else if n == floating_penalty_code as integer {
        print_esc(globals, crate::strpool_str!("floatingpenalty"));
    }
    // global_defs_code:print_esc("globaldefs");
    else if n == global_defs_code as integer {
        print_esc(globals, crate::strpool_str!("globaldefs"));
    }
    // cur_fam_code:print_esc("fam");
    else if n == cur_fam_code as integer {
        print_esc(globals, crate::strpool_str!("fam"));
    }
    // escape_char_code:print_esc("escapechar");
    else if n == escape_char_code as integer {
        print_esc(globals, crate::strpool_str!("escapechar"));
    }
    // default_hyphen_char_code:print_esc("defaulthyphenchar");
    else if n == default_hyphen_char_code as integer {
        print_esc(globals, crate::strpool_str!("defaulthyphenchar"));
    }
    // default_skew_char_code:print_esc("defaultskewchar");
    else if n == default_skew_char_code as integer {
        print_esc(globals, crate::strpool_str!("defaultskewchar"));
    }
    // end_line_char_code:print_esc("endlinechar");
    else if n == end_line_char_code as integer {
        print_esc(globals, crate::strpool_str!("endlinechar"));
    }
    // new_line_char_code:print_esc("newlinechar");
    else if n == new_line_char_code as integer {
        print_esc(globals, crate::strpool_str!("newlinechar"));
    }
    // language_code:print_esc("language");
    else if n == language_code as integer {
        print_esc(globals, crate::strpool_str!("language"));
    }
    // left_hyphen_min_code:print_esc("lefthyphenmin");
    else if n == left_hyphen_min_code as integer {
        print_esc(globals, crate::strpool_str!("lefthyphenmin"));
    }
    // right_hyphen_min_code:print_esc("righthyphenmin");
    else if n == right_hyphen_min_code as integer {
        print_esc(globals, crate::strpool_str!("righthyphenmin"));
    }
    // holding_inserts_code:print_esc("holdinginserts");
    else if n == holding_inserts_code as integer {
        print_esc(globals, crate::strpool_str!("holdinginserts"));
    }
    // error_context_lines_code:print_esc("errorcontextlines");
    else if n == error_context_lines_code as integer {
        print_esc(globals, crate::strpool_str!("errorcontextlines"));
    }
    // othercases print("[unknown integer parameter!]")
    else {
        crate::trace_error_expr!("n = {}", n);
        print(
            globals,
            crate::strpool_str!("[unknown integer parameter!]").get() as _,
        );
    }
    // endcases;
    // end;
}

use crate::info::strpool_str;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0236::*;
