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
