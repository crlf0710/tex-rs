//! @ The |align_state| and |preamble| variables are initialized elsewhere.
//!
//! @<Set init...@>=
//! align_ptr:=null; cur_align:=null; cur_span:=null; cur_loop:=null;
//! cur_head:=null; cur_tail:=null;
//!
//! @ Alignment stack maintenance is handled by a pair of trivial routines
//! called |push_alignment| and |pop_alignment|.
//!
//! @p procedure push_alignment;
//! var p:pointer; {the new alignment stack node}
//! begin p:=get_node(align_stack_node_size);
//! link(p):=align_ptr; info(p):=cur_align;
//! llink(p):=preamble; rlink(p):=cur_span;
//! mem[p+2].int:=cur_loop; mem[p+3].int:=align_state;
//! info(p+4):=cur_head; link(p+4):=cur_tail;
//! align_ptr:=p;
//! cur_head:=get_avail;
//! end;
//! @#
//! procedure pop_alignment;
//! var p:pointer; {the top alignment stack node}
//! begin free_avail(cur_head);
//! p:=align_ptr;
//! cur_tail:=link(p+4); cur_head:=info(p+4);
//! align_state:=mem[p+3].int; cur_loop:=mem[p+2].int;
//! cur_span:=rlink(p); preamble:=llink(p);
//! cur_align:=info(p); align_ptr:=link(p);
//! free_node(p,align_stack_node_size);
//! end;
//!
//! @ \TeX\ has eight procedures that govern alignments: |init_align| and
//! |fin_align| are used at the very beginning and the very end; |init_row| and
//! |fin_row| are used at the beginning and end of individual rows; |init_span|
//! is used at the beginning of a sequence of spanned columns (possibly involving
//! only one column); |init_col| and |fin_col| are used at the beginning and
//! end of individual columns; and |align_peek| is used after \.{\\cr} to see
//! whether the next item is \.{\\noalign}.
//!
//! We shall consider these routines in the order they are first used during
//! the course of a complete \.{\\halign}, namely |init_align|, |align_peek|,
//! |init_row|, |init_span|, |init_col|, |fin_col|, |fin_row|, |fin_align|.
//!
//! @ When \.{\\halign} or \.{\\valign} has been scanned in an appropriate
//! mode, \TeX\ calls |init_align|, whose task is to get everything off to a
//! good start. This mostly involves scanning the preamble and putting its
//! information into the preamble list.
//! @^preamble@>
//!
//! @p @t\4@>@<Declare the procedure called |get_preamble_token|@>@t@>@/
//! procedure@?align_peek; forward;@t\2@>@/
//! procedure@?normal_paragraph; forward;@t\2@>@/
//! procedure init_align;
//! label done, done1, done2, continue;
//! var save_cs_ptr:pointer; {|warning_index| value for error messages}
//! @!p:pointer; {for short-term temporary use}
//! begin save_cs_ptr:=cur_cs; {\.{\\halign} or \.{\\valign}, usually}
//! push_alignment; align_state:=-1000000; {enter a new alignment level}
//! @<Check for improper alignment in displayed math@>;
//! push_nest; {enter a new semantic level}
//! @<Change current mode to |-vmode| for \.{\\halign}, |-hmode| for \.{\\valign}@>;
//! scan_spec(align_group,false);@/
//! @<Scan the preamble and record it in the |preamble| list@>;
//! new_save_level(align_group);
//! if every_cr<>null then begin_token_list(every_cr,every_cr_text);
//! align_peek; {look for \.{\\noalign} or \.{\\omit}}
//! end;
//!
//! @ In vertical modes, |prev_depth| already has the correct value. But
//! if we are in |mmode| (displayed formula mode), we reach out to the
//! enclosing vertical mode for the |prev_depth| value that produces the
//! correct baseline calculations.
//!
//! @<Change current mode...@>=
//! if mode=mmode then
//!   begin mode:=-vmode; prev_depth:=nest[nest_ptr-2].aux_field.sc;
//!   end
//! else if mode>0 then negate(mode)
//!
//! @ When \.{\\halign} is used as a displayed formula, there should be
//! no other pieces of mlists present.
//!
//! @<Check for improper alignment in displayed math@>=
//! if (mode=mmode)and((tail<>head)or(incompleat_noad<>null)) then
//!   begin print_err("Improper "); print_esc("halign"); print(" inside $$'s");
//! @.Improper \\halign...@>
//!   help3("Displays can use special alignments (like \eqalignno)")@/
//!   ("only if nothing but the alignment itself is between $$'s.")@/
//!   ("So I've deleted the formulas that preceded this alignment.");
//!   error; flush_math;
//!   end
//!
//! @ @<Scan the preamble and record it in the |preamble| list@>=
//! preamble:=null; cur_align:=align_head; cur_loop:=null; scanner_status:=aligning;
//! warning_index:=save_cs_ptr; align_state:=-1000000;
//!   {at this point, |cur_cmd=left_brace|}
//! loop@+  begin @<Append the current tabskip glue to the preamble list@>;
//!   if cur_cmd=car_ret then goto done; {\.{\\cr} ends the preamble}
//!   @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|,
//!     looking for changes in the tabskip glue; append an
//!     alignrecord to the preamble list@>;
//!   end;
//! done: scanner_status:=normal
//!
//! @ @<Append the current tabskip glue to the preamble list@>=
//! link(cur_align):=new_param_glue(tab_skip_code);
//! cur_align:=link(cur_align)
//!
//! @ @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|...@>=
//! @<Scan the template \<u_j>, putting the resulting token list in |hold_head|@>;
//! link(cur_align):=new_null_box; cur_align:=link(cur_align); {a new alignrecord}
//! info(cur_align):=end_span; width(cur_align):=null_flag;
//! u_part(cur_align):=link(hold_head);
//! @<Scan the template \<v_j>, putting the resulting token list in |hold_head|@>;
//! v_part(cur_align):=link(hold_head)
//!
//! @ We enter `\.{\\span}' into |eqtb| with |tab_mark| as its command code,
//! and with |span_code| as the command modifier. This makes \TeX\ interpret it
//! essentially the same as an alignment delimiter like `\.\&', yet it is
//! recognizably different when we need to distinguish it from a normal delimiter.
//! It also turns out to be useful to give a special |cr_code| to `\.{\\cr}',
//! and an even larger |cr_cr_code| to `\.{\\crcr}'.
//!
//! The end of a template is represented by two ``frozen'' control sequences
//! called \.{\\endtemplate}. The first has the command code |end_template|, which
//! is |>outer_call|, so it will not easily disappear in the presence of errors.
//! The |get_x_token| routine converts the first into the second, which has |endv|
//! as its command code.
//!
//! @d span_code=256 {distinct from any character}
//! @d cr_code=257 {distinct from |span_code| and from any character}
//! @d cr_cr_code=cr_code+1 {this distinguishes \.{\\crcr} from \.{\\cr}}
//! @d end_template_token==cs_token_flag+frozen_end_template
//!
//! @<Put each of \TeX's primitives into the hash table@>=
//! primitive("span",tab_mark,span_code);@/
//! @!@:span_}{\.{\\span} primitive@>
//! primitive("cr",car_ret,cr_code);
//! @!@:cr_}{\.{\\cr} primitive@>
//! text(frozen_cr):="cr"; eqtb[frozen_cr]:=eqtb[cur_val];@/
//! primitive("crcr",car_ret,cr_cr_code);
//! @!@:cr_cr_}{\.{\\crcr} primitive@>
//! text(frozen_end_template):="endtemplate"; text(frozen_endv):="endtemplate";
//! eq_type(frozen_endv):=endv; equiv(frozen_endv):=null_list;
//! eq_level(frozen_endv):=level_one;@/
//! eqtb[frozen_end_template]:=eqtb[frozen_endv];
//! eq_type(frozen_end_template):=end_template;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! tab_mark: if chr_code=span_code then print_esc("span")
//!   else chr_cmd("alignment tab character ");
//! car_ret: if chr_code=cr_code then print_esc("cr")
//!   else print_esc("crcr");
//!
//! @ The preamble is copied directly, except that \.{\\tabskip} causes a change
//! to the tabskip glue, thereby possibly expanding macros that immediately
//! follow it. An appearance of \.{\\span} also causes such an expansion.
//!
//! Note that if the preamble contains `\.{\\global\\tabskip}', the `\.{\\global}'
//! token survives in the preamble and the `\.{\\tabskip}' defines new
//! tabskip glue (locally).
//!
//! @<Declare the procedure called |get_preamble_token|@>=
//! procedure get_preamble_token;
//! label restart;
//! begin restart: get_token;
//! while (cur_chr=span_code)and(cur_cmd=tab_mark) do
//!   begin get_token; {this token will be expanded once}
//!   if cur_cmd>max_command then
//!     begin expand; get_token;
//!     end;
//!   end;
//! if cur_cmd=endv then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! if (cur_cmd=assign_glue)and(cur_chr=glue_base+tab_skip_code) then
//!   begin scan_optional_equals; scan_glue(glue_val);
//!   if global_defs>0 then geq_define(glue_base+tab_skip_code,glue_ref,cur_val)
//!   else eq_define(glue_base+tab_skip_code,glue_ref,cur_val);
//!   goto restart;
//!   end;
//! end;
//!
//! @ Spaces are eliminated from the beginning of a template.
//!
//! @<Scan the template \<u_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin get_preamble_token;
//!   if cur_cmd=mac_param then goto done1;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!    if (p=hold_head)and(cur_loop=null)and(cur_cmd=tab_mark)
//!     then cur_loop:=cur_align
//!    else  begin print_err("Missing # inserted in alignment preamble");
//! @.Missing \# inserted...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("none, so I've put one in; maybe that will work.");
//!     back_error; goto done1;
//!     end
//!   else if (cur_cmd<>spacer)or(p<>hold_head) then
//!     begin link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!     end;
//!   end;
//! done1:
//!
//! @ @<Scan the template \<v_j>...@>=
//! p:=hold_head; link(p):=null;
//! loop@+  begin continue: get_preamble_token;
//!   if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
//!     goto done2;
//!   if cur_cmd=mac_param then
//!     begin print_err("Only one # is allowed per tab");
//! @.Only one \# is allowed...@>
//!     help3("There should be exactly one # between &'s, when an")@/
//!     ("\halign or \valign is being set up. In this case you had")@/
//!     ("more than one, so I'm ignoring all but the first.");
//!     error; goto continue;
//!     end;
//!   link(p):=get_avail; p:=link(p); info(p):=cur_tok;
//!   end;
//! done2: link(p):=get_avail; p:=link(p);
//! info(p):=end_template_token {put \.{\\endtemplate} at the end}
//!
//! @ The tricky part about alignments is getting the templates into the
//! scanner at the right time, and recovering control when a row or column
//! is finished.
//!
//! We usually begin a row after each \.{\\cr} has been sensed, unless that
//! \.{\\cr} is followed by \.{\\noalign} or by the right brace that terminates
//! the alignment. The |align_peek| routine is used to look ahead and do
//! the right thing; it either gets a new row started, or gets a \.{\\noalign}
//! started, or finishes off the alignment.
//!
//! @<Declare the procedure called |align_peek|@>=
//! procedure align_peek;
//! label restart;
//! begin restart: align_state:=1000000; @<Get the next non-blank non-call token@>;
//! if cur_cmd=no_align then
//!   begin scan_left_brace; new_save_level(no_align_group);
//!   if mode=-vmode then normal_paragraph;
//!   end
//! else if cur_cmd=right_brace then fin_align
//! else if (cur_cmd=car_ret)and(cur_chr=cr_cr_code) then
//!   goto restart {ignore \.{\\crcr}}
//! else  begin init_row; {start a new row}
//!   init_col; {start a new column and replace what we peeked at}
//!   end;
//! end;
//!
//! @ To start a row (i.e., a `row' that rhymes with `dough' but not with `bough'),
//! we enter a new semantic level, copy the first tabskip glue, and change
//! from internal vertical mode to restricted horizontal mode or vice versa.
//! The |space_factor| and |prev_depth| are not used on this semantic level,
//! but we clear them to zero just to be tidy.
//!
//! @p @t\4@>@<Declare the procedure called |init_span|@>@t@>@/
//! procedure init_row;
//! begin push_nest; mode:=(-hmode-vmode)-mode;
//! if mode=-hmode then space_factor:=0 @+else prev_depth:=0;
//! tail_append(new_glue(glue_ptr(preamble)));
//! subtype(tail):=tab_skip_code+1;@/
//! cur_align:=link(preamble); cur_tail:=cur_head; init_span(cur_align);
//! end;
//!
//! @ The parameter to |init_span| is a pointer to the alignrecord where the
//! next column or group of columns will begin. A new semantic level is
//! entered, so that the columns will generate a list for subsequent packaging.
//!
//! @<Declare the procedure called |init_span|@>=
//! procedure init_span(@!p:pointer);
//! begin push_nest;
//! if mode=-hmode then space_factor:=1000
//! else  begin prev_depth:=ignore_depth; normal_paragraph;
//!   end;
//! cur_span:=p;
//! end;
//!
//! @ When a column begins, we assume that |cur_cmd| is either |omit| or else
//! the current token should be put back into the input until the \<u_j>
//! template has been scanned.  (Note that |cur_cmd| might be |tab_mark| or
//! |car_ret|.)  We also assume that |align_state| is approximately 1000000 at
//! this time.  We remain in the same mode, and start the template if it is
//! called for.
//!
//! @p procedure init_col;
//! begin extra_info(cur_align):=cur_cmd;
//! if cur_cmd=omit then align_state:=0
//! else  begin back_input; begin_token_list(u_part(cur_align),u_template);
//!   end; {now |align_state=1000000|}
//! end;
//!
