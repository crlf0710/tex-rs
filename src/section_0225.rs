//! @ Sometimes we need to convert \TeX's internal code numbers into symbolic
//! form. The |print_skip_param| routine gives the symbolic name of a glue
//! parameter.
//!
//! @<Declare the procedure called |print_skip_param|@>=
//! procedure print_skip_param(@!n:integer);
//! begin case n of
//! line_skip_code: print_esc("lineskip");
//! baseline_skip_code: print_esc("baselineskip");
//! par_skip_code: print_esc("parskip");
//! above_display_skip_code: print_esc("abovedisplayskip");
//! below_display_skip_code: print_esc("belowdisplayskip");
//! above_display_short_skip_code: print_esc("abovedisplayshortskip");
//! below_display_short_skip_code: print_esc("belowdisplayshortskip");
//! left_skip_code: print_esc("leftskip");
//! right_skip_code: print_esc("rightskip");
//! top_skip_code: print_esc("topskip");
//! split_top_skip_code: print_esc("splittopskip");
//! tab_skip_code: print_esc("tabskip");
//! space_skip_code: print_esc("spaceskip");
//! xspace_skip_code: print_esc("xspaceskip");
//! par_fill_skip_code: print_esc("parfillskip");
//! thin_mu_skip_code: print_esc("thinmuskip");
//! med_mu_skip_code: print_esc("medmuskip");
//! thick_mu_skip_code: print_esc("thickmuskip");
//! othercases print("[unknown glue parameter!]")
//! endcases;
//! end;
//!
