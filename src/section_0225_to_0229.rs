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
//! @ The symbolic names for glue parameters are put into \TeX's hash table
//! by using the routine called |primitive|, defined below. Let us enter them
//! now, so that we don't have to list all those parameter names anywhere else.
//!
//! @<Put each of \TeX's primitives into the hash table@>=
//! primitive("lineskip",assign_glue,glue_base+line_skip_code);@/
//! @!@:line_skip_}{\.{\\lineskip} primitive@>
//! primitive("baselineskip",assign_glue,glue_base+baseline_skip_code);@/
//! @!@:baseline_skip_}{\.{\\baselineskip} primitive@>
//! primitive("parskip",assign_glue,glue_base+par_skip_code);@/
//! @!@:par_skip_}{\.{\\parskip} primitive@>
//! primitive("abovedisplayskip",assign_glue,glue_base+above_display_skip_code);@/
//! @!@:above_display_skip_}{\.{\\abovedisplayskip} primitive@>
//! primitive("belowdisplayskip",assign_glue,glue_base+below_display_skip_code);@/
//! @!@:below_display_skip_}{\.{\\belowdisplayskip} primitive@>
//! primitive("abovedisplayshortskip",
//!   assign_glue,glue_base+above_display_short_skip_code);@/
//! @!@:above_display_short_skip_}{\.{\\abovedisplayshortskip} primitive@>
//! primitive("belowdisplayshortskip",
//!   assign_glue,glue_base+below_display_short_skip_code);@/
//! @!@:below_display_short_skip_}{\.{\\belowdisplayshortskip} primitive@>
//! primitive("leftskip",assign_glue,glue_base+left_skip_code);@/
//! @!@:left_skip_}{\.{\\leftskip} primitive@>
//! primitive("rightskip",assign_glue,glue_base+right_skip_code);@/
//! @!@:right_skip_}{\.{\\rightskip} primitive@>
//! primitive("topskip",assign_glue,glue_base+top_skip_code);@/
//! @!@:top_skip_}{\.{\\topskip} primitive@>
//! primitive("splittopskip",assign_glue,glue_base+split_top_skip_code);@/
//! @!@:split_top_skip_}{\.{\\splittopskip} primitive@>
//! primitive("tabskip",assign_glue,glue_base+tab_skip_code);@/
//! @!@:tab_skip_}{\.{\\tabskip} primitive@>
//! primitive("spaceskip",assign_glue,glue_base+space_skip_code);@/
//! @!@:space_skip_}{\.{\\spaceskip} primitive@>
//! primitive("xspaceskip",assign_glue,glue_base+xspace_skip_code);@/
//! @!@:xspace_skip_}{\.{\\xspaceskip} primitive@>
//! primitive("parfillskip",assign_glue,glue_base+par_fill_skip_code);@/
//! @!@:par_fill_skip_}{\.{\\parfillskip} primitive@>
//! primitive("thinmuskip",assign_mu_glue,glue_base+thin_mu_skip_code);@/
//! @!@:thin_mu_skip_}{\.{\\thinmuskip} primitive@>
//! primitive("medmuskip",assign_mu_glue,glue_base+med_mu_skip_code);@/
//! @!@:med_mu_skip_}{\.{\\medmuskip} primitive@>
//! primitive("thickmuskip",assign_mu_glue,glue_base+thick_mu_skip_code);@/
//! @!@:thick_mu_skip_}{\.{\\thickmuskip} primitive@>
//!
//! @ @<Cases of |print_cmd_chr| for symbolic printing of primitives@>=
//! assign_glue,assign_mu_glue: if chr_code<skip_base then
//!     print_skip_param(chr_code-glue_base)
//!   else if chr_code<mu_skip_base then
//!     begin print_esc("skip"); print_int(chr_code-skip_base);
//!     end
//!   else  begin print_esc("muskip"); print_int(chr_code-mu_skip_base);
//!     end;
//!
//! @ All glue parameters and registers are initially `\.{0pt plus0pt minus0pt}'.
//!
//! @<Initialize table entries...@>=
//! equiv(glue_base):=zero_glue; eq_level(glue_base):=level_one;
//! eq_type(glue_base):=glue_ref;
//! for k:=glue_base+1 to local_base-1 do eqtb[k]:=eqtb[glue_base];
//! glue_ref_count(zero_glue):=glue_ref_count(zero_glue)+local_base-glue_base;
//!
//! @ @<Show equivalent |n|, in region 3@>=
//! if n<skip_base then
//!   begin print_skip_param(n-glue_base); print_char("=");
//!   if n<glue_base+thin_mu_skip_code then print_spec(equiv(n),"pt")
//!   else print_spec(equiv(n),"mu");
//!   end
//! else if n<mu_skip_base then
//!   begin print_esc("skip"); print_int(n-skip_base); print_char("=");
//!   print_spec(equiv(n),"pt");
//!   end
//! else  begin print_esc("muskip"); print_int(n-mu_skip_base); print_char("=");
//!   print_spec(equiv(n),"mu");
//!   end
//!
