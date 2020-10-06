//! @* \[17] The table of equivalents.
//! Now that we have studied the data structures for \TeX's semantic routines,
//! we ought to consider the data structures used by its syntactic routines. In
//! other words, our next concern will be
//! the tables that \TeX\ looks at when it is scanning
//! what the user has written.
//!
//! The biggest and most important such table is called |eqtb|. It holds the
//! current ``equivalents'' of things; i.e., it explains what things mean
//! or what their current values are, for all quantities that are subject to
//! the nesting structure provided by \TeX's grouping mechanism. There are six
//! parts to |eqtb|:
//!
//! \yskip\hangg 1) |eqtb[active_base..(hash_base-1)]| holds the current
//! equivalents of single-character control sequences.
//!
//! \yskip\hangg 2) |eqtb[hash_base..(glue_base-1)]| holds the current
//! equivalents of multiletter control sequences.
//!
//! \yskip\hangg 3) |eqtb[glue_base..(local_base-1)]| holds the current
//! equivalents of glue parameters like the current baselineskip.
//!
//! \yskip\hangg 4) |eqtb[local_base..(int_base-1)]| holds the current
//! equivalents of local halfword quantities like the current box registers,
//! the current ``catcodes,'' the current font, and a pointer to the current
//! paragraph shape.
//!
//! \yskip\hangg 5) |eqtb[int_base..(dimen_base-1)]| holds the current
//! equivalents of fullword integer parameters like the current hyphenation
//! penalty.
//!
//! \yskip\hangg 6) |eqtb[dimen_base..eqtb_size]| holds the current equivalents
//! of fullword dimension parameters like the current hsize or amount of
//! hanging indentation.
//!
//! \yskip\noindent Note that, for example, the current amount of
//! baselineskip glue is determined by the setting of a particular location
//! in region~3 of |eqtb|, while the current meaning of the control sequence
//! `\.{\\baselineskip}' (which might have been changed by \.{\\def} or
//! \.{\\let}) appears in region~2.
//!
//! @ Each entry in |eqtb| is a |memory_word|. Most of these words are of type
//! |two_halves|, and subdivided into three fields:
//!
//! \yskip\hangg 1) The |eq_level| (a quarterword) is the level of grouping at
//! which this equivalent was defined. If the level is |level_zero|, the
//! equivalent has never been defined; |level_one| refers to the outer level
//! (outside of all groups), and this level is also used for global
//! definitions that never go away. Higher levels are for equivalents that
//! will disappear at the end of their group.  @^global definitions@>
//!
//! \yskip\hangg 2) The |eq_type| (another quarterword) specifies what kind of
//! entry this is. There are many types, since each \TeX\ primitive like
//! \.{\\hbox}, \.{\\def}, etc., has its own special code. The list of
//! command codes above includes all possible settings of the |eq_type| field.
//!
//! \yskip\hangg 3) The |equiv| (a halfword) is the current equivalent value.
//! This may be a font number, a pointer into |mem|, or a variety of other
//! things.
//!
//! @d eq_level_field(#)==#.hh.b1
//! @d eq_type_field(#)==#.hh.b0
//! @d equiv_field(#)==#.hh.rh
//! @d eq_level(#)==eq_level_field(eqtb[#]) {level of definition}
//! @d eq_type(#)==eq_type_field(eqtb[#]) {command code for equivalent}
//! @d equiv(#)==equiv_field(eqtb[#]) {equivalent value}
//! @d level_zero=min_quarterword {level for undefined quantities}
//! @d level_one=level_zero+1 {outermost level for defined quantities}
//!
//! @ Many locations in |eqtb| have symbolic names. The purpose of the next
//! paragraphs is to define these names, and to set up the initial values of the
//! equivalents.
//!
//! In the first region we have 256 equivalents for ``active characters'' that
//! act as control sequences, followed by 256 equivalents for single-character
//! control sequences.
//!
//! Then comes region~2, which corresponds to the hash table that we will
//! define later.  The maximum address in this region is used for a dummy
//! control sequence that is perpetually undefined. There also are several
//! locations for control sequences that are perpetually defined
//! (since they are used in error recovery).
//!
//! @d active_base=1 {beginning of region 1, for active character equivalents}
//! @d single_base=active_base+256 {equivalents of one-character control sequences}
//! @d null_cs=single_base+256 {equivalent of \.{\\csname\\endcsname}}
//! @d hash_base=null_cs+1 {beginning of region 2, for the hash table}
//! @d frozen_control_sequence=hash_base+hash_size {for error recovery}
//! @d frozen_protection=frozen_control_sequence {inaccessible but definable}
//! @d frozen_cr=frozen_control_sequence+1 {permanent `\.{\\cr}'}
//! @d frozen_end_group=frozen_control_sequence+2 {permanent `\.{\\endgroup}'}
//! @d frozen_right=frozen_control_sequence+3 {permanent `\.{\\right}'}
//! @d frozen_fi=frozen_control_sequence+4 {permanent `\.{\\fi}'}
//! @d frozen_end_template=frozen_control_sequence+5 {permanent `\.{\\endtemplate}'}
//! @d frozen_endv=frozen_control_sequence+6 {second permanent `\.{\\endtemplate}'}
//! @d frozen_relax=frozen_control_sequence+7 {permanent `\.{\\relax}'}
//! @d end_write=frozen_control_sequence+8 {permanent `\.{\\endwrite}'}
//! @d frozen_dont_expand=frozen_control_sequence+9
//!   {permanent `\.{\\notexpanded:}'}
//! @d frozen_null_font=frozen_control_sequence+10
//!   {permanent `\.{\\nullfont}'}
//! @d font_id_base=frozen_null_font-font_base
//!   {begins table of 257 permanent font identifiers}
//! @d undefined_control_sequence=frozen_null_font+257 {dummy location}
//! @d glue_base=undefined_control_sequence+1 {beginning of region 3}
//!
//! @<Initialize table entries...@>=
//! eq_type(undefined_control_sequence):=undefined_cs;
//! equiv(undefined_control_sequence):=null;
//! eq_level(undefined_control_sequence):=level_zero;
//! for k:=active_base to undefined_control_sequence-1 do
//!   eqtb[k]:=eqtb[undefined_control_sequence];
//!
//! @ Here is a routine that displays the current meaning of an |eqtb| entry
//! in region 1 or~2. (Similar routines for the other regions will appear
//! below.)
//!
//! @<Show equivalent |n|, in region 1 or 2@>=
//! begin sprint_cs(n); print_char("="); print_cmd_chr(eq_type(n),equiv(n));
//! if eq_type(n)>=call then
//!   begin print_char(":"); show_token_list(link(equiv(n)),null,32);
//!   end;
//! end
//!
//! @ Region 3 of |eqtb| contains the 256 \.{\\skip} registers, as well as the
//! glue parameters defined here. It is important that the ``muskip''
//! parameters have larger numbers than the others.
//!
//! @d line_skip_code=0 {interline glue if |baseline_skip| is infeasible}
//! @d baseline_skip_code=1 {desired glue between baselines}
//! @d par_skip_code=2 {extra glue just above a paragraph}
//! @d above_display_skip_code=3 {extra glue just above displayed math}
//! @d below_display_skip_code=4 {extra glue just below displayed math}
//! @d above_display_short_skip_code=5
//!   {glue above displayed math following short lines}
//! @d below_display_short_skip_code=6
//!   {glue below displayed math following short lines}
//! @d left_skip_code=7 {glue at left of justified lines}
//! @d right_skip_code=8 {glue at right of justified lines}
//! @d top_skip_code=9 {glue at top of main pages}
//! @d split_top_skip_code=10 {glue at top of split pages}
//! @d tab_skip_code=11 {glue between aligned entries}
//! @d space_skip_code=12 {glue between words (if not |zero_glue|)}
//! @d xspace_skip_code=13 {glue after sentences (if not |zero_glue|)}
//! @d par_fill_skip_code=14 {glue on last line of paragraph}
//! @d thin_mu_skip_code=15 {thin space in math formula}
//! @d med_mu_skip_code=16 {medium space in math formula}
//! @d thick_mu_skip_code=17 {thick space in math formula}
//! @d glue_pars=18 {total number of glue parameters}
//! @d skip_base=glue_base+glue_pars {table of 256 ``skip'' registers}
//! @d mu_skip_base=skip_base+256 {table of 256 ``muskip'' registers}
//! @d local_base=mu_skip_base+256 {beginning of region 4}
//! @#
//! @d skip(#)==equiv(skip_base+#) {|mem| location of glue specification}
//! @d mu_skip(#)==equiv(mu_skip_base+#) {|mem| location of math glue spec}
//! @d glue_par(#)==equiv(glue_base+#) {|mem| location of glue specification}
//! @d line_skip==glue_par(line_skip_code)
//! @d baseline_skip==glue_par(baseline_skip_code)
//! @d par_skip==glue_par(par_skip_code)
//! @d above_display_skip==glue_par(above_display_skip_code)
//! @d below_display_skip==glue_par(below_display_skip_code)
//! @d above_display_short_skip==glue_par(above_display_short_skip_code)
//! @d below_display_short_skip==glue_par(below_display_short_skip_code)
//! @d left_skip==glue_par(left_skip_code)
//! @d right_skip==glue_par(right_skip_code)
//! @d top_skip==glue_par(top_skip_code)
//! @d split_top_skip==glue_par(split_top_skip_code)
//! @d tab_skip==glue_par(tab_skip_code)
//! @d space_skip==glue_par(space_skip_code)
//! @d xspace_skip==glue_par(xspace_skip_code)
//! @d par_fill_skip==glue_par(par_fill_skip_code)
//! @d thin_mu_skip==glue_par(thin_mu_skip_code)
//! @d med_mu_skip==glue_par(med_mu_skip_code)
//! @d thick_mu_skip==glue_par(thick_mu_skip_code)
//!
//! @<Current |mem| equivalent of glue parameter number |n|@>=glue_par(n)
//!
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
//! @ Region 4 of |eqtb| contains the local quantities defined here. The
//! bulk of this region is taken up by five tables that are indexed by eight-bit
//! characters; these tables are important to both the syntactic and semantic
//! portions of \TeX. There are also a bunch of special things like font and
//! token parameters, as well as the tables of \.{\\toks} and \.{\\box}
//! registers.
//!
//! @d par_shape_loc=local_base {specifies paragraph shape}
//! @d output_routine_loc=local_base+1 {points to token list for \.{\\output}}
//! @d every_par_loc=local_base+2 {points to token list for \.{\\everypar}}
//! @d every_math_loc=local_base+3 {points to token list for \.{\\everymath}}
//! @d every_display_loc=local_base+4 {points to token list for \.{\\everydisplay}}
//! @d every_hbox_loc=local_base+5 {points to token list for \.{\\everyhbox}}
//! @d every_vbox_loc=local_base+6 {points to token list for \.{\\everyvbox}}
//! @d every_job_loc=local_base+7 {points to token list for \.{\\everyjob}}
//! @d every_cr_loc=local_base+8 {points to token list for \.{\\everycr}}
//! @d err_help_loc=local_base+9 {points to token list for \.{\\errhelp}}
//! @d toks_base=local_base+10 {table of 256 token list registers}
//! @d box_base=toks_base+256 {table of 256 box registers}
//! @d cur_font_loc=box_base+256 {internal font number outside math mode}
//! @d math_font_base=cur_font_loc+1 {table of 48 math font numbers}
//! @d cat_code_base=math_font_base+48
//!   {table of 256 command codes (the ``catcodes'')}
//! @d lc_code_base=cat_code_base+256 {table of 256 lowercase mappings}
//! @d uc_code_base=lc_code_base+256 {table of 256 uppercase mappings}
//! @d sf_code_base=uc_code_base+256 {table of 256 spacefactor mappings}
//! @d math_code_base=sf_code_base+256 {table of 256 math mode mappings}
//! @d int_base=math_code_base+256 {beginning of region 5}
//! @#
//! @d par_shape_ptr==equiv(par_shape_loc)
//! @d output_routine==equiv(output_routine_loc)
//! @d every_par==equiv(every_par_loc)
//! @d every_math==equiv(every_math_loc)
//! @d every_display==equiv(every_display_loc)
//! @d every_hbox==equiv(every_hbox_loc)
//! @d every_vbox==equiv(every_vbox_loc)
//! @d every_job==equiv(every_job_loc)
//! @d every_cr==equiv(every_cr_loc)
//! @d err_help==equiv(err_help_loc)
//! @d toks(#)==equiv(toks_base+#)
//! @d box(#)==equiv(box_base+#)
//! @d cur_font==equiv(cur_font_loc)
//! @d fam_fnt(#)==equiv(math_font_base+#)
//! @d cat_code(#)==equiv(cat_code_base+#)
//! @d lc_code(#)==equiv(lc_code_base+#)
//! @d uc_code(#)==equiv(uc_code_base+#)
//! @d sf_code(#)==equiv(sf_code_base+#)
//! @d math_code(#)==equiv(math_code_base+#)
//!   {Note: |math_code(c)| is the true math code plus |min_halfword|}
//!
//! @<Put each...@>=
//! primitive("output",assign_toks,output_routine_loc);
//! @!@:output_}{\.{\\output} primitive@>
//! primitive("everypar",assign_toks,every_par_loc);
//! @!@:every_par_}{\.{\\everypar} primitive@>
//! primitive("everymath",assign_toks,every_math_loc);
//! @!@:every_math_}{\.{\\everymath} primitive@>
//! primitive("everydisplay",assign_toks,every_display_loc);
//! @!@:every_display_}{\.{\\everydisplay} primitive@>
//! primitive("everyhbox",assign_toks,every_hbox_loc);
//! @!@:every_hbox_}{\.{\\everyhbox} primitive@>
//! primitive("everyvbox",assign_toks,every_vbox_loc);
//! @!@:every_vbox_}{\.{\\everyvbox} primitive@>
//! primitive("everyjob",assign_toks,every_job_loc);
//! @!@:every_job_}{\.{\\everyjob} primitive@>
//! primitive("everycr",assign_toks,every_cr_loc);
//! @!@:every_cr_}{\.{\\everycr} primitive@>
//! primitive("errhelp",assign_toks,err_help_loc);
//! @!@:err_help_}{\.{\\errhelp} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_toks: if chr_code>=toks_base then
//!   begin print_esc("toks"); print_int(chr_code-toks_base);
//!   end
//! else  case chr_code of
//!   output_routine_loc: print_esc("output");
//!   every_par_loc: print_esc("everypar");
//!   every_math_loc: print_esc("everymath");
//!   every_display_loc: print_esc("everydisplay");
//!   every_hbox_loc: print_esc("everyhbox");
//!   every_vbox_loc: print_esc("everyvbox");
//!   every_job_loc: print_esc("everyjob");
//!   every_cr_loc: print_esc("everycr");
//!   othercases print_esc("errhelp")
//!   endcases;
//!
//! @ We initialize most things to null or undefined values. An undefined font
//! is represented by the internal code |font_base|.
//!
//! However, the character code tables are given initial values based on the
//! conventional interpretation of ASCII code. These initial values should
//! not be changed when \TeX\ is adapted for use with non-English languages;
//! all changes to the initialization conventions should be made in format
//! packages, not in \TeX\ itself, so that global interchange of formats is
//! possible.
//!
//! @d null_font==font_base
//! @d var_code==@'70000 {math code meaning ``use the current family''}
//!
//! @<Initialize table entries...@>=
//! par_shape_ptr:=null; eq_type(par_shape_loc):=shape_ref;
//! eq_level(par_shape_loc):=level_one;@/
//! for k:=output_routine_loc to toks_base+255 do
//!   eqtb[k]:=eqtb[undefined_control_sequence];
//! box(0):=null; eq_type(box_base):=box_ref; eq_level(box_base):=level_one;
//! for k:=box_base+1 to box_base+255 do eqtb[k]:=eqtb[box_base];
//! cur_font:=null_font; eq_type(cur_font_loc):=data;
//! eq_level(cur_font_loc):=level_one;@/
//! for k:=math_font_base to math_font_base+47 do eqtb[k]:=eqtb[cur_font_loc];
//! equiv(cat_code_base):=0; eq_type(cat_code_base):=data;
//! eq_level(cat_code_base):=level_one;@/
//! for k:=cat_code_base+1 to int_base-1 do eqtb[k]:=eqtb[cat_code_base];
//! for k:=0 to 255 do
//!   begin cat_code(k):=other_char; math_code(k):=hi(k); sf_code(k):=1000;
//!   end;
//! cat_code(carriage_return):=car_ret; cat_code(" "):=spacer;
//! cat_code("\"):=escape; cat_code("%"):=comment;
//! cat_code(invalid_code):=invalid_char; cat_code(null_code):=ignore;
//! for k:="0" to "9" do math_code(k):=hi(k+var_code);
//! for k:="A" to "Z" do
//!   begin cat_code(k):=letter; cat_code(k+"a"-"A"):=letter;@/
//!   math_code(k):=hi(k+var_code+@"100);
//!   math_code(k+"a"-"A"):=hi(k+"a"-"A"+var_code+@"100);@/
//!   lc_code(k):=k+"a"-"A"; lc_code(k+"a"-"A"):=k+"a"-"A";@/
//!   uc_code(k):=k; uc_code(k+"a"-"A"):=k;@/
//!   sf_code(k):=999;
//!   end;
//!
//! @ @<Show equivalent |n|, in region 4@>=
//! if n=par_shape_loc then
//!   begin print_esc("parshape"); print_char("=");
//!   if par_shape_ptr=null then print_char("0")
//!   else print_int(info(par_shape_ptr));
//!   end
//! else if n<toks_base then
//!   begin print_cmd_chr(assign_toks,n); print_char("=");
//!   if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
//!   end
//! else if n<box_base then
//!   begin print_esc("toks"); print_int(n-toks_base); print_char("=");
//!   if equiv(n)<>null then show_token_list(link(equiv(n)),null,32);
//!   end
//! else if n<cur_font_loc then
//!   begin print_esc("box"); print_int(n-box_base); print_char("=");
//!   if equiv(n)=null then print("void")
//!   else  begin depth_threshold:=0; breadth_max:=1; show_node_list(equiv(n));
//!     end;
//!   end
//! else if n<cat_code_base then @<Show the font identifier in |eqtb[n]|@>
//! else @<Show the halfword code in |eqtb[n]|@>
//!
//! @ @<Show the font identifier in |eqtb[n]|@>=
//! begin if n=cur_font_loc then print("current font")
//! else if n<math_font_base+16 then
//!   begin print_esc("textfont"); print_int(n-math_font_base);
//!   end
//! else if n<math_font_base+32 then
//!   begin print_esc("scriptfont"); print_int(n-math_font_base-16);
//!   end
//! else  begin print_esc("scriptscriptfont"); print_int(n-math_font_base-32);
//!   end;
//! print_char("=");@/
//! print_esc(hash[font_id_base+equiv(n)].rh);
//!   {that's |font_id_text(equiv(n))|}
//! end
//!
//! @ @<Show the halfword code in |eqtb[n]|@>=
//! if n<math_code_base then
//!   begin if n<lc_code_base then
//!     begin print_esc("catcode"); print_int(n-cat_code_base);
//!     end
//!   else if n<uc_code_base then
//!     begin print_esc("lccode"); print_int(n-lc_code_base);
//!     end
//!   else if n<sf_code_base then
//!     begin print_esc("uccode"); print_int(n-uc_code_base);
//!     end
//!   else  begin print_esc("sfcode"); print_int(n-sf_code_base);
//!     end;
//!   print_char("="); print_int(equiv(n));
//!   end
//! else  begin print_esc("mathcode"); print_int(n-math_code_base);
//!   print_char("="); print_int(ho(equiv(n)));
//!   end
//!
//! @ Region 5 of |eqtb| contains the integer parameters and registers defined
//! here, as well as the |del_code| table. The latter table differs from the
//! |cat_code..math_code| tables that precede it, since delimiter codes are
//! fullword integers while the other kinds of codes occupy at most a
//! halfword. This is what makes region~5 different from region~4. We will
//! store the |eq_level| information in an auxiliary array of quarterwords
//! that will be defined later.
//!
//! @d pretolerance_code=0 {badness tolerance before hyphenation}
//! @d tolerance_code=1 {badness tolerance after hyphenation}
//! @d line_penalty_code=2 {added to the badness of every line}
//! @d hyphen_penalty_code=3 {penalty for break after discretionary hyphen}
//! @d ex_hyphen_penalty_code=4 {penalty for break after explicit hyphen}
//! @d club_penalty_code=5 {penalty for creating a club line}
//! @d widow_penalty_code=6 {penalty for creating a widow line}
//! @d display_widow_penalty_code=7 {ditto, just before a display}
//! @d broken_penalty_code=8 {penalty for breaking a page at a broken line}
//! @d bin_op_penalty_code=9 {penalty for breaking after a binary operation}
//! @d rel_penalty_code=10 {penalty for breaking after a relation}
//! @d pre_display_penalty_code=11
//!   {penalty for breaking just before a displayed formula}
//! @d post_display_penalty_code=12
//!   {penalty for breaking just after a displayed formula}
//! @d inter_line_penalty_code=13 {additional penalty between lines}
//! @d double_hyphen_demerits_code=14 {demerits for double hyphen break}
//! @d final_hyphen_demerits_code=15 {demerits for final hyphen break}
//! @d adj_demerits_code=16 {demerits for adjacent incompatible lines}
//! @d mag_code=17 {magnification ratio}
//! @d delimiter_factor_code=18 {ratio for variable-size delimiters}
//! @d looseness_code=19 {change in number of lines for a paragraph}
//! @d time_code=20 {current time of day}
//! @d day_code=21 {current day of the month}
//! @d month_code=22 {current month of the year}
//! @d year_code=23 {current year of our Lord}
//! @d show_box_breadth_code=24 {nodes per level in |show_box|}
//! @d show_box_depth_code=25 {maximum level in |show_box|}
//! @d hbadness_code=26 {hboxes exceeding this badness will be shown by |hpack|}
//! @d vbadness_code=27 {vboxes exceeding this badness will be shown by |vpack|}
//! @d pausing_code=28 {pause after each line is read from a file}
//! @d tracing_online_code=29 {show diagnostic output on terminal}
//! @d tracing_macros_code=30 {show macros as they are being expanded}
//! @d tracing_stats_code=31 {show memory usage if \TeX\ knows it}
//! @d tracing_paragraphs_code=32 {show line-break calculations}
//! @d tracing_pages_code=33 {show page-break calculations}
//! @d tracing_output_code=34 {show boxes when they are shipped out}
//! @d tracing_lost_chars_code=35 {show characters that aren't in the font}
//! @d tracing_commands_code=36 {show command codes at |big_switch|}
//! @d tracing_restores_code=37 {show equivalents when they are restored}
//! @d uc_hyph_code=38 {hyphenate words beginning with a capital letter}
//! @d output_penalty_code=39 {penalty found at current page break}
//! @d max_dead_cycles_code=40 {bound on consecutive dead cycles of output}
//! @d hang_after_code=41 {hanging indentation changes after this many lines}
//! @d floating_penalty_code=42 {penalty for insertions heldover after a split}
//! @d global_defs_code=43 {override \.{\\global} specifications}
//! @d cur_fam_code=44 {current family}
//! @d escape_char_code=45 {escape character for token output}
//! @d default_hyphen_char_code=46 {value of \.{\\hyphenchar} when a font is loaded}
//! @d default_skew_char_code=47 {value of \.{\\skewchar} when a font is loaded}
//! @d end_line_char_code=48 {character placed at the right end of the buffer}
//! @d new_line_char_code=49 {character that prints as |print_ln|}
//! @d language_code=50 {current hyphenation table}
//! @d left_hyphen_min_code=51 {minimum left hyphenation fragment size}
//! @d right_hyphen_min_code=52 {minimum right hyphenation fragment size}
//! @d holding_inserts_code=53 {do not remove insertion nodes from \.{\\box255}}
//! @d error_context_lines_code=54 {maximum intermediate line pairs shown}
//! @d int_pars=55 {total number of integer parameters}
//! @d count_base=int_base+int_pars {256 user \.{\\count} registers}
//! @d del_code_base=count_base+256 {256 delimiter code mappings}
//! @d dimen_base=del_code_base+256 {beginning of region 6}
//! @#
//! @d del_code(#)==eqtb[del_code_base+#].int
//! @d count(#)==eqtb[count_base+#].int
//! @d int_par(#)==eqtb[int_base+#].int {an integer parameter}
//! @d pretolerance==int_par(pretolerance_code)
//! @d tolerance==int_par(tolerance_code)
//! @d line_penalty==int_par(line_penalty_code)
//! @d hyphen_penalty==int_par(hyphen_penalty_code)
//! @d ex_hyphen_penalty==int_par(ex_hyphen_penalty_code)
//! @d club_penalty==int_par(club_penalty_code)
//! @d widow_penalty==int_par(widow_penalty_code)
//! @d display_widow_penalty==int_par(display_widow_penalty_code)
//! @d broken_penalty==int_par(broken_penalty_code)
//! @d bin_op_penalty==int_par(bin_op_penalty_code)
//! @d rel_penalty==int_par(rel_penalty_code)
//! @d pre_display_penalty==int_par(pre_display_penalty_code)
//! @d post_display_penalty==int_par(post_display_penalty_code)
//! @d inter_line_penalty==int_par(inter_line_penalty_code)
//! @d double_hyphen_demerits==int_par(double_hyphen_demerits_code)
//! @d final_hyphen_demerits==int_par(final_hyphen_demerits_code)
//! @d adj_demerits==int_par(adj_demerits_code)
//! @d mag==int_par(mag_code)
//! @d delimiter_factor==int_par(delimiter_factor_code)
//! @d looseness==int_par(looseness_code)
//! @d time==int_par(time_code)
//! @d day==int_par(day_code)
//! @d month==int_par(month_code)
//! @d year==int_par(year_code)
//! @d show_box_breadth==int_par(show_box_breadth_code)
//! @d show_box_depth==int_par(show_box_depth_code)
//! @d hbadness==int_par(hbadness_code)
//! @d vbadness==int_par(vbadness_code)
//! @d pausing==int_par(pausing_code)
//! @d tracing_online==int_par(tracing_online_code)
//! @d tracing_macros==int_par(tracing_macros_code)
//! @d tracing_stats==int_par(tracing_stats_code)
//! @d tracing_paragraphs==int_par(tracing_paragraphs_code)
//! @d tracing_pages==int_par(tracing_pages_code)
//! @d tracing_output==int_par(tracing_output_code)
//! @d tracing_lost_chars==int_par(tracing_lost_chars_code)
//! @d tracing_commands==int_par(tracing_commands_code)
//! @d tracing_restores==int_par(tracing_restores_code)
//! @d uc_hyph==int_par(uc_hyph_code)
//! @d output_penalty==int_par(output_penalty_code)
//! @d max_dead_cycles==int_par(max_dead_cycles_code)
//! @d hang_after==int_par(hang_after_code)
//! @d floating_penalty==int_par(floating_penalty_code)
//! @d global_defs==int_par(global_defs_code)
//! @d cur_fam==int_par(cur_fam_code)
//! @d escape_char==int_par(escape_char_code)
//! @d default_hyphen_char==int_par(default_hyphen_char_code)
//! @d default_skew_char==int_par(default_skew_char_code)
//! @d end_line_char==int_par(end_line_char_code)
//! @d new_line_char==int_par(new_line_char_code)
//! @d language==int_par(language_code)
//! @d left_hyphen_min==int_par(left_hyphen_min_code)
//! @d right_hyphen_min==int_par(right_hyphen_min_code)
//! @d holding_inserts==int_par(holding_inserts_code)
//! @d error_context_lines==int_par(error_context_lines_code)
//!
//! @<Assign the values |depth_threshold:=show_box_depth|...@>=
//! depth_threshold:=show_box_depth;
//! breadth_max:=show_box_breadth
//!
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
//! @ The integer parameter names must be entered into the hash table.
//!
//! @<Put each...@>=
//! primitive("pretolerance",assign_int,int_base+pretolerance_code);@/
//! @!@:pretolerance_}{\.{\\pretolerance} primitive@>
//! primitive("tolerance",assign_int,int_base+tolerance_code);@/
//! @!@:tolerance_}{\.{\\tolerance} primitive@>
//! primitive("linepenalty",assign_int,int_base+line_penalty_code);@/
//! @!@:line_penalty_}{\.{\\linepenalty} primitive@>
//! primitive("hyphenpenalty",assign_int,int_base+hyphen_penalty_code);@/
//! @!@:hyphen_penalty_}{\.{\\hyphenpenalty} primitive@>
//! primitive("exhyphenpenalty",assign_int,int_base+ex_hyphen_penalty_code);@/
//! @!@:ex_hyphen_penalty_}{\.{\\exhyphenpenalty} primitive@>
//! primitive("clubpenalty",assign_int,int_base+club_penalty_code);@/
//! @!@:club_penalty_}{\.{\\clubpenalty} primitive@>
//! primitive("widowpenalty",assign_int,int_base+widow_penalty_code);@/
//! @!@:widow_penalty_}{\.{\\widowpenalty} primitive@>
//! primitive("displaywidowpenalty",
//!   assign_int,int_base+display_widow_penalty_code);@/
//! @!@:display_widow_penalty_}{\.{\\displaywidowpenalty} primitive@>
//! primitive("brokenpenalty",assign_int,int_base+broken_penalty_code);@/
//! @!@:broken_penalty_}{\.{\\brokenpenalty} primitive@>
//! primitive("binoppenalty",assign_int,int_base+bin_op_penalty_code);@/
//! @!@:bin_op_penalty_}{\.{\\binoppenalty} primitive@>
//! primitive("relpenalty",assign_int,int_base+rel_penalty_code);@/
//! @!@:rel_penalty_}{\.{\\relpenalty} primitive@>
//! primitive("predisplaypenalty",assign_int,int_base+pre_display_penalty_code);@/
//! @!@:pre_display_penalty_}{\.{\\predisplaypenalty} primitive@>
//! primitive("postdisplaypenalty",assign_int,int_base+post_display_penalty_code);@/
//! @!@:post_display_penalty_}{\.{\\postdisplaypenalty} primitive@>
//! primitive("interlinepenalty",assign_int,int_base+inter_line_penalty_code);@/
//! @!@:inter_line_penalty_}{\.{\\interlinepenalty} primitive@>
//! primitive("doublehyphendemerits",
//!   assign_int,int_base+double_hyphen_demerits_code);@/
//! @!@:double_hyphen_demerits_}{\.{\\doublehyphendemerits} primitive@>
//! primitive("finalhyphendemerits",
//!   assign_int,int_base+final_hyphen_demerits_code);@/
//! @!@:final_hyphen_demerits_}{\.{\\finalhyphendemerits} primitive@>
//! primitive("adjdemerits",assign_int,int_base+adj_demerits_code);@/
//! @!@:adj_demerits_}{\.{\\adjdemerits} primitive@>
//! primitive("mag",assign_int,int_base+mag_code);@/
//! @!@:mag_}{\.{\\mag} primitive@>
//! primitive("delimiterfactor",assign_int,int_base+delimiter_factor_code);@/
//! @!@:delimiter_factor_}{\.{\\delimiterfactor} primitive@>
//! primitive("looseness",assign_int,int_base+looseness_code);@/
//! @!@:looseness_}{\.{\\looseness} primitive@>
//! primitive("time",assign_int,int_base+time_code);@/
//! @!@:time_}{\.{\\time} primitive@>
//! primitive("day",assign_int,int_base+day_code);@/
//! @!@:day_}{\.{\\day} primitive@>
//! primitive("month",assign_int,int_base+month_code);@/
//! @!@:month_}{\.{\\month} primitive@>
//! primitive("year",assign_int,int_base+year_code);@/
//! @!@:year_}{\.{\\year} primitive@>
//! primitive("showboxbreadth",assign_int,int_base+show_box_breadth_code);@/
//! @!@:show_box_breadth_}{\.{\\showboxbreadth} primitive@>
//! primitive("showboxdepth",assign_int,int_base+show_box_depth_code);@/
//! @!@:show_box_depth_}{\.{\\showboxdepth} primitive@>
//! primitive("hbadness",assign_int,int_base+hbadness_code);@/
//! @!@:hbadness_}{\.{\\hbadness} primitive@>
//! primitive("vbadness",assign_int,int_base+vbadness_code);@/
//! @!@:vbadness_}{\.{\\vbadness} primitive@>
//! primitive("pausing",assign_int,int_base+pausing_code);@/
//! @!@:pausing_}{\.{\\pausing} primitive@>
//! primitive("tracingonline",assign_int,int_base+tracing_online_code);@/
//! @!@:tracing_online_}{\.{\\tracingonline} primitive@>
//! primitive("tracingmacros",assign_int,int_base+tracing_macros_code);@/
//! @!@:tracing_macros_}{\.{\\tracingmacros} primitive@>
//! primitive("tracingstats",assign_int,int_base+tracing_stats_code);@/
//! @!@:tracing_stats_}{\.{\\tracingstats} primitive@>
//! primitive("tracingparagraphs",assign_int,int_base+tracing_paragraphs_code);@/
//! @!@:tracing_paragraphs_}{\.{\\tracingparagraphs} primitive@>
//! primitive("tracingpages",assign_int,int_base+tracing_pages_code);@/
//! @!@:tracing_pages_}{\.{\\tracingpages} primitive@>
//! primitive("tracingoutput",assign_int,int_base+tracing_output_code);@/
//! @!@:tracing_output_}{\.{\\tracingoutput} primitive@>
//! primitive("tracinglostchars",assign_int,int_base+tracing_lost_chars_code);@/
//! @!@:tracing_lost_chars_}{\.{\\tracinglostchars} primitive@>
//! primitive("tracingcommands",assign_int,int_base+tracing_commands_code);@/
//! @!@:tracing_commands_}{\.{\\tracingcommands} primitive@>
//! primitive("tracingrestores",assign_int,int_base+tracing_restores_code);@/
//! @!@:tracing_restores_}{\.{\\tracingrestores} primitive@>
//! primitive("uchyph",assign_int,int_base+uc_hyph_code);@/
//! @!@:uc_hyph_}{\.{\\uchyph} primitive@>
//! primitive("outputpenalty",assign_int,int_base+output_penalty_code);@/
//! @!@:output_penalty_}{\.{\\outputpenalty} primitive@>
//! primitive("maxdeadcycles",assign_int,int_base+max_dead_cycles_code);@/
//! @!@:max_dead_cycles_}{\.{\\maxdeadcycles} primitive@>
//! primitive("hangafter",assign_int,int_base+hang_after_code);@/
//! @!@:hang_after_}{\.{\\hangafter} primitive@>
//! primitive("floatingpenalty",assign_int,int_base+floating_penalty_code);@/
//! @!@:floating_penalty_}{\.{\\floatingpenalty} primitive@>
//! primitive("globaldefs",assign_int,int_base+global_defs_code);@/
//! @!@:global_defs_}{\.{\\globaldefs} primitive@>
//! primitive("fam",assign_int,int_base+cur_fam_code);@/
//! @!@:fam_}{\.{\\fam} primitive@>
//! primitive("escapechar",assign_int,int_base+escape_char_code);@/
//! @!@:escape_char_}{\.{\\escapechar} primitive@>
//! primitive("defaulthyphenchar",assign_int,int_base+default_hyphen_char_code);@/
//! @!@:default_hyphen_char_}{\.{\\defaulthyphenchar} primitive@>
//! primitive("defaultskewchar",assign_int,int_base+default_skew_char_code);@/
//! @!@:default_skew_char_}{\.{\\defaultskewchar} primitive@>
//! primitive("endlinechar",assign_int,int_base+end_line_char_code);@/
//! @!@:end_line_char_}{\.{\\endlinechar} primitive@>
//! primitive("newlinechar",assign_int,int_base+new_line_char_code);@/
//! @!@:new_line_char_}{\.{\\newlinechar} primitive@>
//! primitive("language",assign_int,int_base+language_code);@/
//! @!@:language_}{\.{\\language} primitive@>
//! primitive("lefthyphenmin",assign_int,int_base+left_hyphen_min_code);@/
//! @!@:left_hyphen_min_}{\.{\\lefthyphenmin} primitive@>
//! primitive("righthyphenmin",assign_int,int_base+right_hyphen_min_code);@/
//! @!@:right_hyphen_min_}{\.{\\righthyphenmin} primitive@>
//! primitive("holdinginserts",assign_int,int_base+holding_inserts_code);@/
//! @!@:holding_inserts_}{\.{\\holdinginserts} primitive@>
//! primitive("errorcontextlines",assign_int,int_base+error_context_lines_code);@/
//! @!@:error_context_lines_}{\.{\\errorcontextlines} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_int: if chr_code<count_base then print_param(chr_code-int_base)
//!   else  begin print_esc("count"); print_int(chr_code-count_base);
//!     end;
//!
//! @ The integer parameters should really be initialized by a macro package;
//! the following initialization does the minimum to keep \TeX\ from
//! complete failure.
//! @^null delimiter@>
//!
//! @<Initialize table entries...@>=
//! for k:=int_base to del_code_base-1 do eqtb[k].int:=0;
//! mag:=1000; tolerance:=10000; hang_after:=1; max_dead_cycles:=25;
//! escape_char:="\"; end_line_char:=carriage_return;
//! for k:=0 to 255 do del_code(k):=-1;
//! del_code("."):=0; {this null delimiter is used in error recovery}
//!
//! @ The following procedure, which is called just before \TeX\ initializes its
//! input and output, establishes the initial values of the date and time.
//! @^system dependencies@>
//! Since standard \PASCAL\ cannot provide such information, something special
//! is needed. The program here simply specifies July 4, 1776, at noon; but
//! users probably want a better approximation to the truth.
//!
//! @p procedure fix_date_and_time;
//! begin time:=12*60; {minutes since midnight}
//! day:=4; {fourth day of the month}
//! month:=7; {seventh month of the year}
//! year:=1776; {Anno Domini}
//! end;
//!
//! @ @<Show equivalent |n|, in region 5@>=
//! begin if n<count_base then print_param(n-int_base)
//! else if  n<del_code_base then
//!   begin print_esc("count"); print_int(n-count_base);
//!   end
//! else  begin print_esc("delcode"); print_int(n-del_code_base);
//!   end;
//! print_char("="); print_int(eqtb[n].int);
//! end
//!
//! @ @<Set variable |c| to the current escape character@>=c:=escape_char
//!
//! @ @<Character |s| is the current new-line character@>=s=new_line_char
//!
//! @ \TeX\ is occasionally supposed to print diagnostic information that
//! goes only into the transcript file, unless |tracing_online| is positive.
//! Here are two routines that adjust the destination of print commands:
//!
//! @p procedure begin_diagnostic; {prepare to do some tracing}
//! begin old_setting:=selector;
//! if (tracing_online<=0)and(selector=term_and_log) then
//!   begin decr(selector);
//!   if history=spotless then history:=warning_issued;
//!   end;
//! end;
//! @#
//! procedure end_diagnostic(@!blank_line:boolean);
//!   {restore proper conditions after tracing}
//! begin print_nl("");
//! if blank_line then print_ln;
//! selector:=old_setting;
//! end;
//!
//! @ Of course we had better declare another global variable, if the previous
//! routines are going to work.
//!
//! @<Glob...@>=
//! @!old_setting:0..max_selector;
//!
//! @ The final region of |eqtb| contains the dimension parameters defined
//! here, and the 256 \.{\\dimen} registers.
//!
//! @d par_indent_code=0 {indentation of paragraphs}
//! @d math_surround_code=1 {space around math in text}
//! @d line_skip_limit_code=2 {threshold for |line_skip| instead of |baseline_skip|}
//! @d hsize_code=3 {line width in horizontal mode}
//! @d vsize_code=4 {page height in vertical mode}
//! @d max_depth_code=5 {maximum depth of boxes on main pages}
//! @d split_max_depth_code=6 {maximum depth of boxes on split pages}
//! @d box_max_depth_code=7 {maximum depth of explicit vboxes}
//! @d hfuzz_code=8 {tolerance for overfull hbox messages}
//! @d vfuzz_code=9 {tolerance for overfull vbox messages}
//! @d delimiter_shortfall_code=10 {maximum amount uncovered by variable delimiters}
//! @d null_delimiter_space_code=11 {blank space in null delimiters}
//! @d script_space_code=12 {extra space after subscript or superscript}
//! @d pre_display_size_code=13 {length of text preceding a display}
//! @d display_width_code=14 {length of line for displayed equation}
//! @d display_indent_code=15 {indentation of line for displayed equation}
//! @d overfull_rule_code=16 {width of rule that identifies overfull hboxes}
//! @d hang_indent_code=17 {amount of hanging indentation}
//! @d h_offset_code=18 {amount of horizontal offset when shipping pages out}
//! @d v_offset_code=19 {amount of vertical offset when shipping pages out}
//! @d emergency_stretch_code=20 {reduces badnesses on final pass of line-breaking}
//! @d dimen_pars=21 {total number of dimension parameters}
//! @d scaled_base=dimen_base+dimen_pars
//!   {table of 256 user-defined \.{\\dimen} registers}
//! @d eqtb_size=scaled_base+255 {largest subscript of |eqtb|}
//! @#
//! @d dimen(#)==eqtb[scaled_base+#].sc
//! @d dimen_par(#)==eqtb[dimen_base+#].sc {a scaled quantity}
//! @d par_indent==dimen_par(par_indent_code)
//! @d math_surround==dimen_par(math_surround_code)
//! @d line_skip_limit==dimen_par(line_skip_limit_code)
//! @d hsize==dimen_par(hsize_code)
//! @d vsize==dimen_par(vsize_code)
//! @d max_depth==dimen_par(max_depth_code)
//! @d split_max_depth==dimen_par(split_max_depth_code)
//! @d box_max_depth==dimen_par(box_max_depth_code)
//! @d hfuzz==dimen_par(hfuzz_code)
//! @d vfuzz==dimen_par(vfuzz_code)
//! @d delimiter_shortfall==dimen_par(delimiter_shortfall_code)
//! @d null_delimiter_space==dimen_par(null_delimiter_space_code)
//! @d script_space==dimen_par(script_space_code)
//! @d pre_display_size==dimen_par(pre_display_size_code)
//! @d display_width==dimen_par(display_width_code)
//! @d display_indent==dimen_par(display_indent_code)
//! @d overfull_rule==dimen_par(overfull_rule_code)
//! @d hang_indent==dimen_par(hang_indent_code)
//! @d h_offset==dimen_par(h_offset_code)
//! @d v_offset==dimen_par(v_offset_code)
//! @d emergency_stretch==dimen_par(emergency_stretch_code)
//!
//! @p procedure print_length_param(@!n:integer);
//! begin case n of
//! par_indent_code:print_esc("parindent");
//! math_surround_code:print_esc("mathsurround");
//! line_skip_limit_code:print_esc("lineskiplimit");
//! hsize_code:print_esc("hsize");
//! vsize_code:print_esc("vsize");
//! max_depth_code:print_esc("maxdepth");
//! split_max_depth_code:print_esc("splitmaxdepth");
//! box_max_depth_code:print_esc("boxmaxdepth");
//! hfuzz_code:print_esc("hfuzz");
//! vfuzz_code:print_esc("vfuzz");
//! delimiter_shortfall_code:print_esc("delimitershortfall");
//! null_delimiter_space_code:print_esc("nulldelimiterspace");
//! script_space_code:print_esc("scriptspace");
//! pre_display_size_code:print_esc("predisplaysize");
//! display_width_code:print_esc("displaywidth");
//! display_indent_code:print_esc("displayindent");
//! overfull_rule_code:print_esc("overfullrule");
//! hang_indent_code:print_esc("hangindent");
//! h_offset_code:print_esc("hoffset");
//! v_offset_code:print_esc("voffset");
//! emergency_stretch_code:print_esc("emergencystretch");
//! othercases print("[unknown dimen parameter!]")
//! endcases;
//! end;
//!
//! @ @<Put each...@>=
//! primitive("parindent",assign_dimen,dimen_base+par_indent_code);@/
//! @!@:par_indent_}{\.{\\parindent} primitive@>
//! primitive("mathsurround",assign_dimen,dimen_base+math_surround_code);@/
//! @!@:math_surround_}{\.{\\mathsurround} primitive@>
//! primitive("lineskiplimit",assign_dimen,dimen_base+line_skip_limit_code);@/
//! @!@:line_skip_limit_}{\.{\\lineskiplimit} primitive@>
//! primitive("hsize",assign_dimen,dimen_base+hsize_code);@/
//! @!@:hsize_}{\.{\\hsize} primitive@>
//! primitive("vsize",assign_dimen,dimen_base+vsize_code);@/
//! @!@:vsize_}{\.{\\vsize} primitive@>
//! primitive("maxdepth",assign_dimen,dimen_base+max_depth_code);@/
//! @!@:max_depth_}{\.{\\maxdepth} primitive@>
//! primitive("splitmaxdepth",assign_dimen,dimen_base+split_max_depth_code);@/
//! @!@:split_max_depth_}{\.{\\splitmaxdepth} primitive@>
//! primitive("boxmaxdepth",assign_dimen,dimen_base+box_max_depth_code);@/
//! @!@:box_max_depth_}{\.{\\boxmaxdepth} primitive@>
//! primitive("hfuzz",assign_dimen,dimen_base+hfuzz_code);@/
//! @!@:hfuzz_}{\.{\\hfuzz} primitive@>
//! primitive("vfuzz",assign_dimen,dimen_base+vfuzz_code);@/
//! @!@:vfuzz_}{\.{\\vfuzz} primitive@>
//! primitive("delimitershortfall",
//!   assign_dimen,dimen_base+delimiter_shortfall_code);@/
//! @!@:delimiter_shortfall_}{\.{\\delimitershortfall} primitive@>
//! primitive("nulldelimiterspace",
//!   assign_dimen,dimen_base+null_delimiter_space_code);@/
//! @!@:null_delimiter_space_}{\.{\\nulldelimiterspace} primitive@>
//! primitive("scriptspace",assign_dimen,dimen_base+script_space_code);@/
//! @!@:script_space_}{\.{\\scriptspace} primitive@>
//! primitive("predisplaysize",assign_dimen,dimen_base+pre_display_size_code);@/
//! @!@:pre_display_size_}{\.{\\predisplaysize} primitive@>
//! primitive("displaywidth",assign_dimen,dimen_base+display_width_code);@/
//! @!@:display_width_}{\.{\\displaywidth} primitive@>
//! primitive("displayindent",assign_dimen,dimen_base+display_indent_code);@/
//! @!@:display_indent_}{\.{\\displayindent} primitive@>
//! primitive("overfullrule",assign_dimen,dimen_base+overfull_rule_code);@/
//! @!@:overfull_rule_}{\.{\\overfullrule} primitive@>
//! primitive("hangindent",assign_dimen,dimen_base+hang_indent_code);@/
//! @!@:hang_indent_}{\.{\\hangindent} primitive@>
//! primitive("hoffset",assign_dimen,dimen_base+h_offset_code);@/
//! @!@:h_offset_}{\.{\\hoffset} primitive@>
//! primitive("voffset",assign_dimen,dimen_base+v_offset_code);@/
//! @!@:v_offset_}{\.{\\voffset} primitive@>
//! primitive("emergencystretch",assign_dimen,dimen_base+emergency_stretch_code);@/
//! @!@:emergency_stretch_}{\.{\\emergencystretch} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_dimen: if chr_code<scaled_base then
//!     print_length_param(chr_code-dimen_base)
//!   else  begin print_esc("dimen"); print_int(chr_code-scaled_base);
//!     end;
//!
//! @ @<Initialize table entries...@>=
//! for k:=dimen_base to eqtb_size do eqtb[k].sc:=0;
//!
//! @ @<Show equivalent |n|, in region 6@>=
//! begin if n<scaled_base then print_length_param(n-dimen_base)
//! else  begin print_esc("dimen"); print_int(n-scaled_base);
//!   end;
//! print_char("="); print_scaled(eqtb[n].sc); print("pt");
//! end
//!
//! @ Here is a procedure that displays the contents of |eqtb[n]|
//! symbolically.
//!
//! @p@t\4@>@<Declare the procedure called |print_cmd_chr|@>@;@/
//! @!stat procedure show_eqtb(@!n:pointer);
//! begin if n<active_base then print_char("?") {this can't happen}
//! else if n<glue_base then @<Show equivalent |n|, in region 1 or 2@>
//! else if n<local_base then @<Show equivalent |n|, in region 3@>
//! else if n<int_base then @<Show equivalent |n|, in region 4@>
//! else if n<dimen_base then @<Show equivalent |n|, in region 5@>
//! else if n<=eqtb_size then @<Show equivalent |n|, in region 6@>
//! else print_char("?"); {this can't happen either}
//! end;
//! tats
//!
//! @ The last two regions of |eqtb| have fullword values instead of the
//! three fields |eq_level|, |eq_type|, and |equiv|. An |eq_type| is unnecessary,
//! but \TeX\ needs to store the |eq_level| information in another array
//! called |xeq_level|.
//!
//! @<Glob...@>=
//! @!eqtb:array[active_base..eqtb_size] of memory_word;
//! @!xeq_level:array[int_base..eqtb_size] of quarterword;
//!
//! @ @<Set init...@>=
//! for k:=int_base to eqtb_size do xeq_level[k]:=level_one;
//!
//! @ When the debugging routine |search_mem| is looking for pointers having a
//! given value, it is interested only in regions 1 to~3 of~|eqtb|, and in the
//! first part of region~4.
//!
//! @<Search |eqtb| for equivalents equal to |p|@>=
//! for q:=active_base to box_base+255 do
//!   begin if equiv(q)=p then
//!     begin print_nl("EQUIV("); print_int(q); print_char(")");
//!     end;
//!   end
//!
