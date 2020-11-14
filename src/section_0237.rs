//! @ We can print the symbolic name of an integer parameter as follows.
//!
//! @p procedure print_param(@!n:integer);
//! begin case n of
//! pretolerance_code:print_esc("pretolerance");
//! tolerance_code:print_esc("tolerance");
//! line_penalty_code:print_esc("linepenalty");
//! hyphen_penalty_code:print_esc("hyphenpenalty");
//! ex_hyphen_penalty_code:print_esc("exhyphenpenalty");
//! club_penalty_code:print_esc("clubpenalty");
//! widow_penalty_code:print_esc("widowpenalty");
//! display_widow_penalty_code:print_esc("displaywidowpenalty");
//! broken_penalty_code:print_esc("brokenpenalty");
//! bin_op_penalty_code:print_esc("binoppenalty");
//! rel_penalty_code:print_esc("relpenalty");
//! pre_display_penalty_code:print_esc("predisplaypenalty");
//! post_display_penalty_code:print_esc("postdisplaypenalty");
//! inter_line_penalty_code:print_esc("interlinepenalty");
//! double_hyphen_demerits_code:print_esc("doublehyphendemerits");
//! final_hyphen_demerits_code:print_esc("finalhyphendemerits");
//! adj_demerits_code:print_esc("adjdemerits");
//! mag_code:print_esc("mag");
//! delimiter_factor_code:print_esc("delimiterfactor");
//! looseness_code:print_esc("looseness");
//! time_code:print_esc("time");
//! day_code:print_esc("day");
//! month_code:print_esc("month");
//! year_code:print_esc("year");
//! show_box_breadth_code:print_esc("showboxbreadth");
//! show_box_depth_code:print_esc("showboxdepth");
//! hbadness_code:print_esc("hbadness");
//! vbadness_code:print_esc("vbadness");
//! pausing_code:print_esc("pausing");
//! tracing_online_code:print_esc("tracingonline");
//! tracing_macros_code:print_esc("tracingmacros");
//! tracing_stats_code:print_esc("tracingstats");
//! tracing_paragraphs_code:print_esc("tracingparagraphs");
//! tracing_pages_code:print_esc("tracingpages");
//! tracing_output_code:print_esc("tracingoutput");
//! tracing_lost_chars_code:print_esc("tracinglostchars");
//! tracing_commands_code:print_esc("tracingcommands");
//! tracing_restores_code:print_esc("tracingrestores");
//! uc_hyph_code:print_esc("uchyph");
//! output_penalty_code:print_esc("outputpenalty");
//! max_dead_cycles_code:print_esc("maxdeadcycles");
//! hang_after_code:print_esc("hangafter");
//! floating_penalty_code:print_esc("floatingpenalty");
//! global_defs_code:print_esc("globaldefs");
//! cur_fam_code:print_esc("fam");
//! escape_char_code:print_esc("escapechar");
//! default_hyphen_char_code:print_esc("defaulthyphenchar");
//! default_skew_char_code:print_esc("defaultskewchar");
//! end_line_char_code:print_esc("endlinechar");
//! new_line_char_code:print_esc("newlinechar");
//! language_code:print_esc("language");
//! left_hyphen_min_code:print_esc("lefthyphenmin");
//! right_hyphen_min_code:print_esc("righthyphenmin");
//! holding_inserts_code:print_esc("holdinginserts");
//! error_context_lines_code:print_esc("errorcontextlines");
//! othercases print("[unknown integer parameter!]")
//! endcases;
//! end;
//!

