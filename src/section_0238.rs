//! @ The integer parameter names must be entered into the hash table.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0238(globals: &mut TeXGlobals) {
    // primitive("pretolerance",assign_int,int_base+pretolerance_code);@/
    primitive(globals, strpool_str!("pretolerance"), assign_int, (int_base + pretolerance_code as word) as _);
    // @!@:pretolerance_}{\.{\\pretolerance} primitive@>
    // primitive("tolerance",assign_int,int_base+tolerance_code);@/
    primitive(globals, strpool_str!("tolerance"), assign_int, (int_base + tolerance_code as word) as _);
    // @!@:tolerance_}{\.{\\tolerance} primitive@>
    // primitive("linepenalty",assign_int,int_base+line_penalty_code);@/
    primitive(globals, strpool_str!("linepenalty"), assign_int, (int_base + line_penalty_code as word) as _);
    // @!@:line_penalty_}{\.{\\linepenalty} primitive@>
    // primitive("hyphenpenalty",assign_int,int_base+hyphen_penalty_code);@/
    primitive(globals, strpool_str!("hyphenpenalty"), assign_int, (int_base + hyphen_penalty_code as word) as _);
    // @!@:hyphen_penalty_}{\.{\\hyphenpenalty} primitive@>
    // primitive("exhyphenpenalty",assign_int,int_base+ex_hyphen_penalty_code);@/
    primitive(globals, strpool_str!("exhyphenpenalty"), assign_int, (int_base + ex_hyphen_penalty_code as word) as _);
    // @!@:ex_hyphen_penalty_}{\.{\\exhyphenpenalty} primitive@>
    // primitive("clubpenalty",assign_int,int_base+club_penalty_code);@/
    primitive(globals, strpool_str!("clubpenalty"), assign_int, (int_base + club_penalty_code as word) as _);
    // @!@:club_penalty_}{\.{\\clubpenalty} primitive@>
    // primitive("widowpenalty",assign_int,int_base+widow_penalty_code);@/
    primitive(globals, strpool_str!("widowpenalty"), assign_int, (int_base + widow_penalty_code as word) as _);
    // @!@:widow_penalty_}{\.{\\widowpenalty} primitive@>
    // primitive("displaywidowpenalty",
    //   assign_int,int_base+display_widow_penalty_code);@/
    primitive(globals, strpool_str!("displaywidowpenalty"), assign_int, (int_base + display_widow_penalty_code as word) as _);
    // @!@:display_widow_penalty_}{\.{\\displaywidowpenalty} primitive@>
    // primitive("brokenpenalty",assign_int,int_base+broken_penalty_code);@/
    primitive(globals, strpool_str!("brokenpenalty"), assign_int, (int_base + broken_penalty_code as word) as _);
    // @!@:broken_penalty_}{\.{\\brokenpenalty} primitive@>
    // primitive("binoppenalty",assign_int,int_base+bin_op_penalty_code);@/
    primitive(globals, strpool_str!("binoppenalty"), assign_int, (int_base + bin_op_penalty_code as word) as _);
    // @!@:bin_op_penalty_}{\.{\\binoppenalty} primitive@>
    // primitive("relpenalty",assign_int,int_base+rel_penalty_code);@/
    primitive(globals, strpool_str!("relpenalty"), assign_int, (int_base + rel_penalty_code as word) as _);
    // @!@:rel_penalty_}{\.{\\relpenalty} primitive@>
    // primitive("predisplaypenalty",assign_int,int_base+pre_display_penalty_code);@/
    primitive(globals, strpool_str!("predisplaypenalty"), assign_int, (int_base + pre_display_penalty_code as word) as _);
    // @!@:pre_display_penalty_}{\.{\\predisplaypenalty} primitive@>
    // primitive("postdisplaypenalty",assign_int,int_base+post_display_penalty_code);@/
    primitive(globals, strpool_str!("postdisplaypenalty"), assign_int, (int_base + post_display_penalty_code as word) as _);
    // @!@:post_display_penalty_}{\.{\\postdisplaypenalty} primitive@>
    // primitive("interlinepenalty",assign_int,int_base+inter_line_penalty_code);@/
    primitive(globals, strpool_str!("interlinepenalty"), assign_int, (int_base + inter_line_penalty_code as word) as _);
    // @!@:inter_line_penalty_}{\.{\\interlinepenalty} primitive@>
    // primitive("doublehyphendemerits",
    //   assign_int,int_base+double_hyphen_demerits_code);@/
    primitive(globals, strpool_str!("doublehyphendemerits"), assign_int, (int_base + double_hyphen_demerits_code as word) as _);
    // @!@:double_hyphen_demerits_}{\.{\\doublehyphendemerits} primitive@>
    // primitive("finalhyphendemerits",
    //   assign_int,int_base+final_hyphen_demerits_code);@/
    primitive(globals, strpool_str!("finalhyphendemerits"), assign_int, (int_base + final_hyphen_demerits_code as word) as _);
    // @!@:final_hyphen_demerits_}{\.{\\finalhyphendemerits} primitive@>
    // primitive("adjdemerits",assign_int,int_base+adj_demerits_code);@/
    primitive(globals, strpool_str!("adjdemerits"), assign_int, (int_base + adj_demerits_code as word) as _);
    // @!@:adj_demerits_}{\.{\\adjdemerits} primitive@>
    // primitive("mag",assign_int,int_base+mag_code);@/
    primitive(globals, strpool_str!("mag"), assign_int, (int_base + mag_code as word) as _);
    // @!@:mag_}{\.{\\mag} primitive@>
    // primitive("delimiterfactor",assign_int,int_base+delimiter_factor_code);@/
    primitive(globals, strpool_str!("delimiterfactor"), assign_int, (int_base + delimiter_factor_code as word) as _);
    // @!@:delimiter_factor_}{\.{\\delimiterfactor} primitive@>
    // primitive("looseness",assign_int,int_base+looseness_code);@/
    primitive(globals, strpool_str!("looseness"), assign_int, (int_base + looseness_code as word) as _);
    // @!@:looseness_}{\.{\\looseness} primitive@>
    // primitive("time",assign_int,int_base+time_code);@/
    primitive(globals, strpool_str!("time"), assign_int, (int_base + time_code as word) as _);
    // @!@:time_}{\.{\\time} primitive@>
    // primitive("day",assign_int,int_base+day_code);@/
    primitive(globals, strpool_str!("day"), assign_int, (int_base + day_code as word) as _);
    // @!@:day_}{\.{\\day} primitive@>
    // primitive("month",assign_int,int_base+month_code);@/
    primitive(globals, strpool_str!("month"), assign_int, (int_base + month_code as word) as _);
    // @!@:month_}{\.{\\month} primitive@>
    // primitive("year",assign_int,int_base+year_code);@/
    primitive(globals, strpool_str!("year"), assign_int, (int_base + year_code as word) as _);
    // @!@:year_}{\.{\\year} primitive@>
    // primitive("showboxbreadth",assign_int,int_base+show_box_breadth_code);@/
    primitive(globals, strpool_str!("showboxbreadth"), assign_int, (int_base + show_box_breadth_code as word) as _);
    // @!@:show_box_breadth_}{\.{\\showboxbreadth} primitive@>
    // primitive("showboxdepth",assign_int,int_base+show_box_depth_code);@/
    primitive(globals, strpool_str!("showboxdepth"), assign_int, (int_base + show_box_depth_code as word) as _);
    // @!@:show_box_depth_}{\.{\\showboxdepth} primitive@>
    // primitive("hbadness",assign_int,int_base+hbadness_code);@/
    primitive(globals, strpool_str!("hbadness"), assign_int, (int_base + hbadness_code as word) as _);
    // @!@:hbadness_}{\.{\\hbadness} primitive@>
    // primitive("vbadness",assign_int,int_base+vbadness_code);@/
    primitive(globals, strpool_str!("vbadness"), assign_int, (int_base + vbadness_code as word) as _);
    // @!@:vbadness_}{\.{\\vbadness} primitive@>
    // primitive("pausing",assign_int,int_base+pausing_code);@/
    primitive(globals, strpool_str!("pausing"), assign_int, (int_base + pausing_code as word) as _);
    // @!@:pausing_}{\.{\\pausing} primitive@>
    // primitive("tracingonline",assign_int,int_base+tracing_online_code);@/
    primitive(globals, strpool_str!("tracingonline"), assign_int, (int_base + tracing_online_code as word) as _);
    // @!@:tracing_online_}{\.{\\tracingonline} primitive@>
    // primitive("tracingmacros",assign_int,int_base+tracing_macros_code);@/
    primitive(globals, strpool_str!("tracingmacros"), assign_int, (int_base + tracing_macros_code as word) as _);
    // @!@:tracing_macros_}{\.{\\tracingmacros} primitive@>
    // primitive("tracingstats",assign_int,int_base+tracing_stats_code);@/
    primitive(globals, strpool_str!("tracingstats"), assign_int, (int_base + tracing_stats_code as word) as _);
    // @!@:tracing_stats_}{\.{\\tracingstats} primitive@>
    // primitive("tracingparagraphs",assign_int,int_base+tracing_paragraphs_code);@/
    primitive(globals, strpool_str!("tracingparagraphs"), assign_int, (int_base + tracing_paragraphs_code as word) as _);
    // @!@:tracing_paragraphs_}{\.{\\tracingparagraphs} primitive@>
    // primitive("tracingpages",assign_int,int_base+tracing_pages_code);@/
    primitive(globals, strpool_str!("tracingpages"), assign_int, (int_base + tracing_pages_code as word) as _);
    // @!@:tracing_pages_}{\.{\\tracingpages} primitive@>
    // primitive("tracingoutput",assign_int,int_base+tracing_output_code);@/
    primitive(globals, strpool_str!("tracingoutput"), assign_int, (int_base + tracing_output_code as word) as _);
    // @!@:tracing_output_}{\.{\\tracingoutput} primitive@>
    // primitive("tracinglostchars",assign_int,int_base+tracing_lost_chars_code);@/
    primitive(globals, strpool_str!("tracinglostchars"), assign_int, (int_base + tracing_lost_chars_code as word) as _);
    // @!@:tracing_lost_chars_}{\.{\\tracinglostchars} primitive@>
    // primitive("tracingcommands",assign_int,int_base+tracing_commands_code);@/
    primitive(globals, strpool_str!("tracingcommands"), assign_int, (int_base + tracing_commands_code as word) as _);
    // @!@:tracing_commands_}{\.{\\tracingcommands} primitive@>
    // primitive("tracingrestores",assign_int,int_base+tracing_restores_code);@/
    primitive(globals, strpool_str!("tracingrestores"), assign_int, (int_base + tracing_restores_code as word) as _);
    // @!@:tracing_restores_}{\.{\\tracingrestores} primitive@>
    // primitive("uchyph",assign_int,int_base+uc_hyph_code);@/
    primitive(globals, strpool_str!("uchyph"), assign_int, (int_base + uc_hyph_code as word) as _);
    // @!@:uc_hyph_}{\.{\\uchyph} primitive@>
    // primitive("outputpenalty",assign_int,int_base+output_penalty_code);@/
    primitive(globals, strpool_str!("outputpenalty"), assign_int, (int_base + output_penalty_code as word) as _);
    // @!@:output_penalty_}{\.{\\outputpenalty} primitive@>
    // primitive("maxdeadcycles",assign_int,int_base+max_dead_cycles_code);@/
    primitive(globals, strpool_str!("maxdeadcycles"), assign_int, (int_base + max_dead_cycles_code as word) as _);
    // @!@:max_dead_cycles_}{\.{\\maxdeadcycles} primitive@>
    // primitive("hangafter",assign_int,int_base+hang_after_code);@/
    primitive(globals, strpool_str!("hangafter"), assign_int, (int_base + hang_after_code as word) as _);
    // @!@:hang_after_}{\.{\\hangafter} primitive@>
    // primitive("floatingpenalty",assign_int,int_base+floating_penalty_code);@/
    primitive(globals, strpool_str!("floatingpenalty"), assign_int, (int_base + floating_penalty_code as word) as _);
    // @!@:floating_penalty_}{\.{\\floatingpenalty} primitive@>
    // primitive("globaldefs",assign_int,int_base+global_defs_code);@/
    primitive(globals, strpool_str!("globaldefs"), assign_int, (int_base + global_defs_code as word) as _);
    // @!@:global_defs_}{\.{\\globaldefs} primitive@>
    // primitive("fam",assign_int,int_base+cur_fam_code);@/
    primitive(globals, strpool_str!("fam"), assign_int, (int_base + cur_fam_code as word) as _);
    // @!@:fam_}{\.{\\fam} primitive@>
    // primitive("escapechar",assign_int,int_base+escape_char_code);@/
    primitive(globals, strpool_str!("escapechar"), assign_int, (int_base + escape_char_code as word) as _);
    // @!@:escape_char_}{\.{\\escapechar} primitive@>
    // primitive("defaulthyphenchar",assign_int,int_base+default_hyphen_char_code);@/
    primitive(globals, strpool_str!("defaulthyphenchar"), assign_int, (int_base + default_hyphen_char_code as word) as _);
    // @!@:default_hyphen_char_}{\.{\\defaulthyphenchar} primitive@>
    // primitive("defaultskewchar",assign_int,int_base+default_skew_char_code);@/
    primitive(globals, strpool_str!("defaultskewchar"), assign_int, (int_base + default_skew_char_code as word) as _);
    // @!@:default_skew_char_}{\.{\\defaultskewchar} primitive@>
    // primitive("endlinechar",assign_int,int_base+end_line_char_code);@/
    primitive(globals, strpool_str!("endlinechar"), assign_int, (int_base + end_line_char_code as word) as _);
    // @!@:end_line_char_}{\.{\\endlinechar} primitive@>
    // primitive("newlinechar",assign_int,int_base+new_line_char_code);@/
    primitive(globals, strpool_str!("newlinechar"), assign_int, (int_base + new_line_char_code as word) as _);
    // @!@:new_line_char_}{\.{\\newlinechar} primitive@>
    // primitive("language",assign_int,int_base+language_code);@/
    primitive(globals, strpool_str!("language"), assign_int, (int_base + language_code as word) as _);
    // @!@:language_}{\.{\\language} primitive@>
    // primitive("lefthyphenmin",assign_int,int_base+left_hyphen_min_code);@/
    primitive(globals, strpool_str!("lefthyphenmin"), assign_int, (int_base + left_hyphen_min_code as word) as _);
    // @!@:left_hyphen_min_}{\.{\\lefthyphenmin} primitive@>
    // primitive("righthyphenmin",assign_int,int_base+right_hyphen_min_code);@/
    primitive(globals, strpool_str!("righthyphenmin"), assign_int, (int_base + right_hyphen_min_code as word) as _);
    // @!@:right_hyphen_min_}{\.{\\righthyphenmin} primitive@>
    // primitive("holdinginserts",assign_int,int_base+holding_inserts_code);@/
    primitive(globals, strpool_str!("holdinginserts"), assign_int, (int_base + holding_inserts_code as word) as _);
    // @!@:holding_inserts_}{\.{\\holdinginserts} primitive@>
    // primitive("errorcontextlines",assign_int,int_base+error_context_lines_code);@/
    primitive(globals, strpool_str!("errorcontextlines"), assign_int, (int_base + error_context_lines_code as word) as _);
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
