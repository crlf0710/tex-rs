//! @ The integer parameter names must be entered into the hash table.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0238(globals: &mut TeXGlobals) {
    // primitive("pretolerance",assign_int,int_base+pretolerance_code);@/
    // @!@:pretolerance_}{\.{\\pretolerance} primitive@>
    // primitive("tolerance",assign_int,int_base+tolerance_code);@/
    // @!@:tolerance_}{\.{\\tolerance} primitive@>
    // primitive("linepenalty",assign_int,int_base+line_penalty_code);@/
    // @!@:line_penalty_}{\.{\\linepenalty} primitive@>
    // primitive("hyphenpenalty",assign_int,int_base+hyphen_penalty_code);@/
    // @!@:hyphen_penalty_}{\.{\\hyphenpenalty} primitive@>
    // primitive("exhyphenpenalty",assign_int,int_base+ex_hyphen_penalty_code);@/
    // @!@:ex_hyphen_penalty_}{\.{\\exhyphenpenalty} primitive@>
    // primitive("clubpenalty",assign_int,int_base+club_penalty_code);@/
    // @!@:club_penalty_}{\.{\\clubpenalty} primitive@>
    // primitive("widowpenalty",assign_int,int_base+widow_penalty_code);@/
    // @!@:widow_penalty_}{\.{\\widowpenalty} primitive@>
    // primitive("displaywidowpenalty",
    //   assign_int,int_base+display_widow_penalty_code);@/
    // @!@:display_widow_penalty_}{\.{\\displaywidowpenalty} primitive@>
    // primitive("brokenpenalty",assign_int,int_base+broken_penalty_code);@/
    // @!@:broken_penalty_}{\.{\\brokenpenalty} primitive@>
    // primitive("binoppenalty",assign_int,int_base+bin_op_penalty_code);@/
    // @!@:bin_op_penalty_}{\.{\\binoppenalty} primitive@>
    // primitive("relpenalty",assign_int,int_base+rel_penalty_code);@/
    // @!@:rel_penalty_}{\.{\\relpenalty} primitive@>
    // primitive("predisplaypenalty",assign_int,int_base+pre_display_penalty_code);@/
    // @!@:pre_display_penalty_}{\.{\\predisplaypenalty} primitive@>
    // primitive("postdisplaypenalty",assign_int,int_base+post_display_penalty_code);@/
    // @!@:post_display_penalty_}{\.{\\postdisplaypenalty} primitive@>
    // primitive("interlinepenalty",assign_int,int_base+inter_line_penalty_code);@/
    // @!@:inter_line_penalty_}{\.{\\interlinepenalty} primitive@>
    // primitive("doublehyphendemerits",
    //   assign_int,int_base+double_hyphen_demerits_code);@/
    // @!@:double_hyphen_demerits_}{\.{\\doublehyphendemerits} primitive@>
    // primitive("finalhyphendemerits",
    //   assign_int,int_base+final_hyphen_demerits_code);@/
    // @!@:final_hyphen_demerits_}{\.{\\finalhyphendemerits} primitive@>
    // primitive("adjdemerits",assign_int,int_base+adj_demerits_code);@/
    // @!@:adj_demerits_}{\.{\\adjdemerits} primitive@>
    // primitive("mag",assign_int,int_base+mag_code);@/
    // @!@:mag_}{\.{\\mag} primitive@>
    // primitive("delimiterfactor",assign_int,int_base+delimiter_factor_code);@/
    // @!@:delimiter_factor_}{\.{\\delimiterfactor} primitive@>
    // primitive("looseness",assign_int,int_base+looseness_code);@/
    // @!@:looseness_}{\.{\\looseness} primitive@>
    // primitive("time",assign_int,int_base+time_code);@/
    // @!@:time_}{\.{\\time} primitive@>
    // primitive("day",assign_int,int_base+day_code);@/
    // @!@:day_}{\.{\\day} primitive@>
    // primitive("month",assign_int,int_base+month_code);@/
    // @!@:month_}{\.{\\month} primitive@>
    // primitive("year",assign_int,int_base+year_code);@/
    // @!@:year_}{\.{\\year} primitive@>
    // primitive("showboxbreadth",assign_int,int_base+show_box_breadth_code);@/
    // @!@:show_box_breadth_}{\.{\\showboxbreadth} primitive@>
    // primitive("showboxdepth",assign_int,int_base+show_box_depth_code);@/
    // @!@:show_box_depth_}{\.{\\showboxdepth} primitive@>
    // primitive("hbadness",assign_int,int_base+hbadness_code);@/
    // @!@:hbadness_}{\.{\\hbadness} primitive@>
    // primitive("vbadness",assign_int,int_base+vbadness_code);@/
    // @!@:vbadness_}{\.{\\vbadness} primitive@>
    // primitive("pausing",assign_int,int_base+pausing_code);@/
    // @!@:pausing_}{\.{\\pausing} primitive@>
    // primitive("tracingonline",assign_int,int_base+tracing_online_code);@/
    primitive(globals, strpool_str!("tracingonline"), assign_int, (int_base + tracing_online_code as word) as _);
    // @!@:tracing_online_}{\.{\\tracingonline} primitive@>
    // primitive("tracingmacros",assign_int,int_base+tracing_macros_code);@/
    primitive(globals, strpool_str!("tracingmacros"), assign_int, (int_base + tracing_macros_code as word) as _);
    // @!@:tracing_macros_}{\.{\\tracingmacros} primitive@>
    // primitive("tracingstats",assign_int,int_base+tracing_stats_code);@/
    // @!@:tracing_stats_}{\.{\\tracingstats} primitive@>
    // primitive("tracingparagraphs",assign_int,int_base+tracing_paragraphs_code);@/
    // @!@:tracing_paragraphs_}{\.{\\tracingparagraphs} primitive@>
    // primitive("tracingpages",assign_int,int_base+tracing_pages_code);@/
    // @!@:tracing_pages_}{\.{\\tracingpages} primitive@>
    // primitive("tracingoutput",assign_int,int_base+tracing_output_code);@/
    // @!@:tracing_output_}{\.{\\tracingoutput} primitive@>
    // primitive("tracinglostchars",assign_int,int_base+tracing_lost_chars_code);@/
    primitive(globals, strpool_str!("tracinglostchars"), assign_int, (int_base + tracing_lost_chars_code as word) as _);
    // @!@:tracing_lost_chars_}{\.{\\tracinglostchars} primitive@>
    // primitive("tracingcommands",assign_int,int_base+tracing_commands_code);@/
    primitive(globals, strpool_str!("tracingcommands"), assign_int, (int_base + tracing_commands_code as word) as _);
    // @!@:tracing_commands_}{\.{\\tracingcommands} primitive@>
    // primitive("tracingrestores",assign_int,int_base+tracing_restores_code);@/
    // @!@:tracing_restores_}{\.{\\tracingrestores} primitive@>
    // primitive("uchyph",assign_int,int_base+uc_hyph_code);@/
    // @!@:uc_hyph_}{\.{\\uchyph} primitive@>
    // primitive("outputpenalty",assign_int,int_base+output_penalty_code);@/
    // @!@:output_penalty_}{\.{\\outputpenalty} primitive@>
    // primitive("maxdeadcycles",assign_int,int_base+max_dead_cycles_code);@/
    // @!@:max_dead_cycles_}{\.{\\maxdeadcycles} primitive@>
    // primitive("hangafter",assign_int,int_base+hang_after_code);@/
    // @!@:hang_after_}{\.{\\hangafter} primitive@>
    // primitive("floatingpenalty",assign_int,int_base+floating_penalty_code);@/
    // @!@:floating_penalty_}{\.{\\floatingpenalty} primitive@>
    // primitive("globaldefs",assign_int,int_base+global_defs_code);@/
    // @!@:global_defs_}{\.{\\globaldefs} primitive@>
    // primitive("fam",assign_int,int_base+cur_fam_code);@/
    // @!@:fam_}{\.{\\fam} primitive@>
    // primitive("escapechar",assign_int,int_base+escape_char_code);@/
    primitive(globals, strpool_str!("escapechar"), assign_int, (int_base + escape_char_code as word) as _);
    // @!@:escape_char_}{\.{\\escapechar} primitive@>
    // primitive("defaulthyphenchar",assign_int,int_base+default_hyphen_char_code);@/
    // @!@:default_hyphen_char_}{\.{\\defaulthyphenchar} primitive@>
    // primitive("defaultskewchar",assign_int,int_base+default_skew_char_code);@/
    // @!@:default_skew_char_}{\.{\\defaultskewchar} primitive@>
    // primitive("endlinechar",assign_int,int_base+end_line_char_code);@/
    // @!@:end_line_char_}{\.{\\endlinechar} primitive@>
    // primitive("newlinechar",assign_int,int_base+new_line_char_code);@/
    // @!@:new_line_char_}{\.{\\newlinechar} primitive@>
    // primitive("language",assign_int,int_base+language_code);@/
    // @!@:language_}{\.{\\language} primitive@>
    // primitive("lefthyphenmin",assign_int,int_base+left_hyphen_min_code);@/
    // @!@:left_hyphen_min_}{\.{\\lefthyphenmin} primitive@>
    // primitive("righthyphenmin",assign_int,int_base+right_hyphen_min_code);@/
    // @!@:right_hyphen_min_}{\.{\\righthyphenmin} primitive@>
    // primitive("holdinginserts",assign_int,int_base+holding_inserts_code);@/
    // @!@:holding_inserts_}{\.{\\holdinginserts} primitive@>
    // primitive("errorcontextlines",assign_int,int_base+error_context_lines_code);@/
    // @!@:error_context_lines_}{\.{\\errorcontextlines} primitive@>
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0209::assign_int;
use crate::section_0230::int_base;
use crate::section_0236::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
