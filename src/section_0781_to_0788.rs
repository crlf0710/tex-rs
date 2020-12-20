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
