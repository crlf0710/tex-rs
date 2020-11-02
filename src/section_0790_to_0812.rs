//! @ The token list |omit_template| just referred to is a constant token
//! list that contains the special control sequence \.{\\endtemplate} only.
//!
//! @<Initialize the special...@>=
//! info(omit_template):=end_template_token; {|link(omit_template)=null|}
//!
//! @ When the |endv| command at the end of a \<v_j> template comes through the
//! scanner, things really start to happen; and it is the |fin_col| routine
//! that makes them happen. This routine returns |true| if a row as well as a
//! column has been finished.
//!
//! @p function fin_col:boolean;
//! label exit;
//! var p:pointer; {the alignrecord after the current one}
//! @!q,@!r:pointer; {temporary pointers for list manipulation}
//! @!s:pointer; {a new span node}
//! @!u:pointer; {a new unset box}
//! @!w:scaled; {natural width}
//! @!o:glue_ord; {order of infinity}
//! @!n:halfword; {span counter}
//! begin if cur_align=null then confusion("endv");
//! q:=link(cur_align);@+if q=null then confusion("endv");
//! @:this can't happen endv}{\quad endv@>
//! if align_state<500000 then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//! p:=link(q);
//! @<If the preamble list has been traversed, check that the row has ended@>;
//! if extra_info(cur_align)<>span_code then
//!   begin unsave; new_save_level(align_group);@/
//!   @<Package an unset box for the current column and record its width@>;
//!   @<Copy the tabskip glue between columns@>;
//!   if extra_info(cur_align)>=cr_code then
//!     begin fin_col:=true; return;
//!     end;
//!   init_span(p);
//!   end;
//! align_state:=1000000; @<Get the next non-blank non-call token@>;
//! cur_align:=p;
//! init_col; fin_col:=false;
//! exit: end;
//!
//! @ @<If the preamble list has been traversed, check that the row has ended@>=
//! if (p=null)and(extra_info(cur_align)<cr_code) then
//!  if cur_loop<>null then @<Lengthen the preamble periodically@>
//!  else  begin print_err("Extra alignment tab has been changed to ");
//! @.Extra alignment tab...@>
//!   print_esc("cr");
//!   help3("You have given more \span or & marks than there were")@/
//!   ("in the preamble to the \halign or \valign now in progress.")@/
//!   ("So I'll assume that you meant to type \cr instead.");
//!   extra_info(cur_align):=cr_code; error;
//!   end
//!
//! @ @<Lengthen the preamble...@>=
//! begin link(q):=new_null_box; p:=link(q); {a new alignrecord}
//! info(p):=end_span; width(p):=null_flag; cur_loop:=link(cur_loop);
//! @<Copy the templates from node |cur_loop| into node |p|@>;
//! cur_loop:=link(cur_loop);
//! link(p):=new_glue(glue_ptr(cur_loop));
//! end
//!
//! @ @<Copy the templates from node |cur_loop| into node |p|@>=
//! q:=hold_head; r:=u_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; u_part(p):=link(hold_head);
//! q:=hold_head; r:=v_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; v_part(p):=link(hold_head)
//!
//! @ @<Copy the tabskip glue...@>=
//! tail_append(new_glue(glue_ptr(link(cur_align))));
//! subtype(tail):=tab_skip_code+1
//!
//! @ @<Package an unset...@>=
//! begin if mode=-hmode then
//!   begin adjust_tail:=cur_tail; u:=hpack(link(head),natural); w:=width(u);
//!   cur_tail:=adjust_tail; adjust_tail:=null;
//!   end
//! else  begin u:=vpackage(link(head),natural,0); w:=height(u);
//!   end;
//! n:=min_quarterword; {this represents a span count of 1}
//! if cur_span<>cur_align then @<Update width entry for spanned columns@>
//! else if w>width(cur_align) then width(cur_align):=w;
//! type(u):=unset_node; span_count(u):=n;@/
//! @<Determine the stretch order@>;
//! glue_order(u):=o; glue_stretch(u):=total_stretch[o];@/
//! @<Determine the shrink order@>;
//! glue_sign(u):=o; glue_shrink(u):=total_shrink[o];@/
//! pop_nest; link(tail):=u; tail:=u;
//! end
//!
//! @ A span node is a 2-word record containing |width|, |info|, and |link|
//! fields. The |link| field is not really a link, it indicates the number of
//! spanned columns; the |info| field points to a span node for the same
//! starting column, having a greater extent of spanning, or to |end_span|,
//! which has the largest possible |link| field; the |width| field holds the
//! largest natural width corresponding to a particular set of spanned columns.
//!
//! A list of the maximum widths so far, for spanned columns starting at a
//! given column, begins with the |info| field of the alignrecord for that
//! column.
//!
//! @d span_node_size=2 {number of |mem| words for a span node}
//!
//! @<Initialize the special list heads...@>=
//! link(end_span):=max_quarterword+1; info(end_span):=null;
//!
//! @ @<Update width entry for spanned columns@>=
//! begin q:=cur_span;
//! repeat incr(n); q:=link(link(q));
//! until q=cur_align;
//! if n>max_quarterword then confusion("256 spans"); {this can happen, but won't}
//! @^system dependencies@>
//! @:this can't happen 256 spans}{\quad 256 spans@>
//! q:=cur_span; while link(info(q))<n do q:=info(q);
//! if link(info(q))>n then
//!   begin s:=get_node(span_node_size); info(s):=info(q); link(s):=n;
//!   info(q):=s; width(s):=w;
//!   end
//! else if width(info(q))<w then width(info(q)):=w;
//! end
//!
//! @ At the end of a row, we append an unset box to the current vlist (for
//! \.{\\halign}) or the current hlist (for \.{\\valign}). This unset box
//! contains the unset boxes for the columns, separated by the tabskip glue.
//! Everything will be set later.
//!
//! @p procedure fin_row;
//! var p:pointer; {the new unset box}
//! begin if mode=-hmode then
//!   begin p:=hpack(link(head),natural);
//!   pop_nest; append_to_vlist(p);
//!   if cur_head<>cur_tail then
//!     begin link(tail):=link(cur_head); tail:=cur_tail;
//!     end;
//!   end
//! else  begin p:=vpack(link(head),natural); pop_nest;
//!   link(tail):=p; tail:=p; space_factor:=1000;
//!   end;
//! type(p):=unset_node; glue_stretch(p):=0;
//! if every_cr<>null then begin_token_list(every_cr,every_cr_text);
//! align_peek;
//! end; {note that |glue_shrink(p)=0| since |glue_shrink==shift_amount|}
//!
//! @ Finally, we will reach the end of the alignment, and we can breathe a
//! sigh of relief that memory hasn't overflowed. All the unset boxes will now be
//! set so that the columns line up, taking due account of spanned columns.
//!
//! @p procedure@?do_assignments; forward;@t\2@>@/
//! procedure@?resume_after_display; forward;@t\2@>@/
//! procedure@?build_page; forward;@t\2@>@/
//! procedure fin_align;
//! var @!p,@!q,@!r,@!s,@!u,@!v: pointer; {registers for the list operations}
//! @!t,@!w:scaled; {width of column}
//! @!o:scaled; {shift offset for unset boxes}
//! @!n:halfword; {matching span amount}
//! @!rule_save:scaled; {temporary storage for |overfull_rule|}
//! @!aux_save:memory_word; {temporary storage for |aux|}
//! begin if cur_group<>align_group then confusion("align1");
//! @:this can't happen align}{\quad align@>
//! unsave; {that |align_group| was for individual entries}
//! if cur_group<>align_group then confusion("align0");
//! unsave; {that |align_group| was for the whole alignment}
//! if nest[nest_ptr-1].mode_field=mmode then o:=display_indent
//!   else o:=0;
//! @<Go through the preamble list, determining the column widths and
//!   changing the alignrecords to dummy unset boxes@>;
//! @<Package the preamble list, to determine the actual tabskip glue amounts,
//!   and let |p| point to this prototype box@>;
//! @<Set the glue in all the unset boxes of the current list@>;
//! flush_node_list(p); pop_alignment;
//! @<Insert the \(c)current list into its environment@>;
//! end;@/
//! @t\4@>@<Declare the procedure called |align_peek|@>
//!
//! @ It's time now to dismantle the preamble list and to compute the column
//! widths. Let $w_{ij}$ be the maximum of the natural widths of all entries
//! that span columns $i$ through $j$, inclusive. The alignrecord for column~$i$
//! contains $w_{ii}$ in its |width| field, and there is also a linked list of
//! the nonzero $w_{ij}$ for increasing $j$, accessible via the |info| field;
//! these span nodes contain the value $j-i+|min_quarterword|$ in their
//! |link| fields. The values of $w_{ii}$ were initialized to |null_flag|, which
//! we regard as $-\infty$.
//!
//! The final column widths are defined by the formula
//! $$w_j=\max_{1\L i\L j}\biggl( w_{ij}-\sum_{i\L k<j}(t_k+w_k)\biggr),$$
//! where $t_k$ is the natural width of the tabskip glue between columns
//! $k$ and~$k+1$. However, if $w_{ij}=-\infty$ for all |i| in the range
//! |1<=i<=j| (i.e., if every entry that involved column~|j| also involved
//! column~|j+1|), we let $w_j=0$, and we zero out the tabskip glue after
//! column~|j|.
//!
//! \TeX\ computes these values by using the following scheme: First $w_1=w_{11}$.
//! Then replace $w_{2j}$ by $\max(w_{2j},w_{1j}-t_1-w_1)$, for all $j>1$.
//! Then $w_2=w_{22}$. Then replace $w_{3j}$ by $\max(w_{3j},w_{2j}-t_2-w_2)$
//! for all $j>2$; and so on. If any $w_j$ turns out to be $-\infty$, its
//! value is changed to zero and so is the next tabskip.
//!
//! @<Go through the preamble list,...@>=
//! q:=link(preamble);
//! repeat flush_list(u_part(q)); flush_list(v_part(q));
//! p:=link(link(q));
//! if width(q)=null_flag then
//!   @<Nullify |width(q)| and the tabskip glue following this column@>;
//! if info(q)<>end_span then
//!   @<Merge the widths in the span nodes of |q| with those of |p|,
//!     destroying the span nodes of |q|@>;
//! type(q):=unset_node; span_count(q):=min_quarterword; height(q):=0;
//! depth(q):=0; glue_order(q):=normal; glue_sign(q):=normal;
//! glue_stretch(q):=0; glue_shrink(q):=0; q:=p;
//! until q=null
//!
//! @ @<Nullify |width(q)| and the tabskip glue following this column@>=
//! begin width(q):=0; r:=link(q); s:=glue_ptr(r);
//! if s<>zero_glue then
//!   begin add_glue_ref(zero_glue); delete_glue_ref(s);
//!   glue_ptr(r):=zero_glue;
//!   end;
//! end
//!
//! @ Merging of two span-node lists is a typical exercise in the manipulation of
//! linearly linked data structures. The essential invariant in the following
//! |repeat| loop is that we want to dispense with node |r|, in |q|'s list,
//! and |u| is its successor; all nodes of |p|'s list up to and including |s|
//! have been processed, and the successor of |s| matches |r| or precedes |r|
//! or follows |r|, according as |link(r)=n| or |link(r)>n| or |link(r)<n|.
//!
//! @<Merge the widths...@>=
//! begin t:=width(q)+width(glue_ptr(link(q)));
//! r:=info(q); s:=end_span; info(s):=p; n:=min_quarterword+1;
//! repeat width(r):=width(r)-t; u:=info(r);
//! while link(r)>n do
//!   begin s:=info(s); n:=link(info(s))+1;
//!   end;
//! if link(r)<n then
//!   begin info(r):=info(s); info(s):=r; decr(link(r)); s:=r;
//!   end
//! else  begin if width(r)>width(info(s)) then width(info(s)):=width(r);
//!   free_node(r,span_node_size);
//!   end;
//! r:=u;
//! until r=end_span;
//! end
//!
//! @ Now the preamble list has been converted to a list of alternating unset
//! boxes and tabskip glue, where the box widths are equal to the final
//! column sizes. In case of \.{\\valign}, we change the widths to heights,
//! so that a correct error message will be produced if the alignment is
//! overfull or underfull.
//!
//! @<Package the preamble list...@>=
//! save_ptr:=save_ptr-2; pack_begin_line:=-mode_line;
//! if mode=-vmode then
//!   begin rule_save:=overfull_rule;
//!   overfull_rule:=0; {prevent rule from being packaged}
//!   p:=hpack(preamble,saved(1),saved(0)); overfull_rule:=rule_save;
//!   end
//! else  begin q:=link(preamble);
//!   repeat height(q):=width(q); width(q):=0; q:=link(link(q));
//!   until q=null;
//!   p:=vpack(preamble,saved(1),saved(0));
//!   q:=link(preamble);
//!   repeat width(q):=height(q); height(q):=0; q:=link(link(q));
//!   until q=null;
//!   end;
//! pack_begin_line:=0
//!
//! @ @<Set the glue in all the unset...@>=
//! q:=link(head); s:=head;
//! while q<>null do
//!   begin if not is_char_node(q) then
//!     if type(q)=unset_node then
//!       @<Set the unset box |q| and the unset boxes in it@>
//!     else if type(q)=rule_node then
//!       @<Make the running dimensions in rule |q| extend to the
//!         boundaries of the alignment@>;
//!   s:=q; q:=link(q);
//!   end
//!
//! @ @<Make the running dimensions in rule |q| extend...@>=
//! begin if is_running(width(q)) then width(q):=width(p);
//! if is_running(height(q)) then height(q):=height(p);
//! if is_running(depth(q)) then depth(q):=depth(p);
//! if o<>0 then
//!   begin r:=link(q); link(q):=null; q:=hpack(q,natural);
//!   shift_amount(q):=o; link(q):=r; link(s):=q;
//!   end;
//! end
//!
//! @ The unset box |q| represents a row that contains one or more unset boxes,
//! depending on how soon \.{\\cr} occurred in that row.
//!
//! @<Set the unset box |q| and the unset boxes in it@>=
//! begin if mode=-vmode then
//!   begin type(q):=hlist_node; width(q):=width(p);
//!   end
//! else  begin type(q):=vlist_node; height(q):=height(p);
//!   end;
//! glue_order(q):=glue_order(p); glue_sign(q):=glue_sign(p);
//! glue_set(q):=glue_set(p); shift_amount(q):=o;
//! r:=link(list_ptr(q)); s:=link(list_ptr(p));
//! repeat @<Set the glue in node |r| and change it from an unset node@>;
//! r:=link(link(r)); s:=link(link(s));
//! until r=null;
//! end
//!
//! @ A box made from spanned columns will be followed by tabskip glue nodes and
//! by empty boxes as if there were no spanning. This permits perfect alignment
//! of subsequent entries, and it prevents values that depend on floating point
//! arithmetic from entering into the dimensions of any boxes.
//!
//! @<Set the glue in node |r|...@>=
//! n:=span_count(r); t:=width(s); w:=t; u:=hold_head;
//! while n>min_quarterword do
//!   begin decr(n);
//!   @<Append tabskip glue and an empty box to list |u|,
//!     and update |s| and |t| as the prototype nodes are passed@>;
//!   end;
//! if mode=-vmode then
//!   @<Make the unset node |r| into an |hlist_node| of width |w|,
//!     setting the glue as if the width were |t|@>
//! else @<Make the unset node |r| into a |vlist_node| of height |w|,
//!     setting the glue as if the height were |t|@>;
//! shift_amount(r):=0;
//! if u<>hold_head then {append blank boxes to account for spanned nodes}
//!   begin link(u):=link(r); link(r):=link(hold_head); r:=u;
//!   end
//!
//! @ @<Append tabskip glue and an empty box to list |u|...@>=
//! s:=link(s); v:=glue_ptr(s); link(u):=new_glue(v); u:=link(u);
//! subtype(u):=tab_skip_code+1; t:=t+width(v);
//! if glue_sign(p)=stretching then
//!   begin if stretch_order(v)=glue_order(p) then
//!     t:=t+round(float(glue_set(p))*stretch(v));
//! @^real multiplication@>
//!   end
//! else if glue_sign(p)=shrinking then
//!   begin if shrink_order(v)=glue_order(p) then
//!     t:=t-round(float(glue_set(p))*shrink(v));
//!   end;
//! s:=link(s); link(u):=new_null_box; u:=link(u); t:=t+width(s);
//! if mode=-vmode then width(u):=width(s)@+else
//!   begin type(u):=vlist_node; height(u):=width(s);
//!   end
//!
//! @ @<Make the unset node |r| into an |hlist_node| of width |w|...@>=
//! begin height(r):=height(q); depth(r):=depth(q);
//! if t=width(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>width(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-width(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(width(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((width(r)-t)/glue_shrink(r));
//!   end;
//! width(r):=w; type(r):=hlist_node;
//! end
//!
//! @ @<Make the unset node |r| into a |vlist_node| of height |w|...@>=
//! begin width(r):=width(q);
//! if t=height(r) then
//!   begin glue_sign(r):=normal; glue_order(r):=normal;
//!   set_glue_ratio_zero(glue_set(r));
//!   end
//! else if t>height(r) then
//!   begin glue_sign(r):=stretching;
//!   if glue_stretch(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else glue_set(r):=unfloat((t-height(r))/glue_stretch(r));
//! @^real division@>
//!   end
//! else  begin glue_order(r):=glue_sign(r); glue_sign(r):=shrinking;
//!   if glue_shrink(r)=0 then set_glue_ratio_zero(glue_set(r))
//!   else if (glue_order(r)=normal)and(height(r)-t>glue_shrink(r)) then
//!     set_glue_ratio_one(glue_set(r))
//!   else glue_set(r):=unfloat((height(r)-t)/glue_shrink(r));
//!   end;
//! height(r):=w; type(r):=vlist_node;
//! end
//!
//! @ We now have a completed alignment, in the list that starts at |head|
//! and ends at |tail|. This list will be merged with the one that encloses
//! it. (In case the enclosing mode is |mmode|, for displayed formulas,
//! we will need to insert glue before and after the display; that part of the
//! program will be deferred until we're more familiar with such operations.)
//!
//! In restricted horizontal mode, the |clang| part of |aux| is undefined;
//! an over-cautious \PASCAL\ runtime system may complain about this.
//! @^dirty \PASCAL@>
//!
//! @<Insert the \(c)current list into its environment@>=
//! aux_save:=aux; p:=link(head); q:=tail; pop_nest;
//! if mode=mmode then @<Finish an alignment in a display@>
//! else  begin aux:=aux_save; link(tail):=p;
//!   if p<>null then tail:=q;
//!   if mode=vmode then build_page;
//!   end
//!
