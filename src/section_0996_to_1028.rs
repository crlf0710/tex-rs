//! @ @<Update the values of |last_glue|...@>=
//! if last_glue<>max_halfword then delete_glue_ref(last_glue);
//! last_penalty:=0; last_kern:=0;
//! if type(p)=glue_node then
//!   begin last_glue:=glue_ptr(p); add_glue_ref(last_glue);
//!   end
//! else  begin last_glue:=max_halfword;
//!   if type(p)=penalty_node then last_penalty:=penalty(p)
//!   else if type(p)=kern_node then last_kern:=width(p);
//!   end
//!
//! @ The code here is an example of a many-way switch into routines that
//! merge together in different places. Some people call this unstructured
//! programming, but the author doesn't see much wrong with it, as long as
//! @^Knuth, Donald Ervin@>
//! the various labels have a well-understood meaning.
//!
//! @<Move node |p| to the current page; ...@>=
//! @<If the current page is empty and node |p| is to be deleted, |goto done1|;
//!   otherwise use node |p| to update the state of the current page;
//!   if this node is an insertion, |goto contribute|; otherwise if this node
//!   is not a legal breakpoint, |goto contribute| or |update_heights|;
//!   otherwise set |pi| to the penalty associated with this breakpoint@>;
//! @<Check if node |p| is a new champion breakpoint; then \(if)if it is time for
//!   a page break, prepare for output, and either fire up the user's
//!   output routine and |return| or ship out the page and |goto done|@>;
//! if (type(p)<glue_node)or(type(p)>kern_node) then goto contribute;
//! update_heights:@<Update the current page measurements with respect to the
//!   glue or kern specified by node~|p|@>;
//! contribute: @<Make sure that |page_max_depth| is not exceeded@>;
//! @<Link node |p| into the current page and |goto done|@>;
//! done1:@<Recycle node |p|@>;
//! done:
//!
//! @ @<Link node |p| into the current page and |goto done|@>=
//! link(page_tail):=p; page_tail:=p;
//! link(contrib_head):=link(p); link(p):=null; goto done
//!
//! @ @<Recycle node |p|@>=
//! link(contrib_head):=link(p); link(p):=null; flush_node_list(p)
//!
//! @ The title of this section is already so long, it seems best to avoid
//! making it more accurate but still longer, by mentioning the fact that a
//! kern node at the end of the contribution list will not be contributed until
//! we know its successor.
//!
//! @<If the current page is empty...@>=
//! case type(p) of
//! hlist_node,vlist_node,rule_node: if page_contents<box_there then
//!     @<Initialize the current page, insert the \.{\\topskip} glue
//!       ahead of |p|, and |goto continue|@>
//!   else @<Prepare to move a box or rule node to the current page,
//!     then |goto contribute|@>;
//! whatsit_node: @<Prepare to move whatsit |p| to the current page,
//!   then |goto contribute|@>;
//! glue_node: if page_contents<box_there then goto done1
//!   else if precedes_break(page_tail) then pi:=0
//!   else goto update_heights;
//! kern_node: if page_contents<box_there then goto done1
//!   else if link(p)=null then return
//!   else if type(link(p))=glue_node then pi:=0
//!   else goto update_heights;
//! penalty_node: if page_contents<box_there then goto done1@+else pi:=penalty(p);
//! mark_node: goto contribute;
//! ins_node: @<Append an insertion to the current page and |goto contribute|@>;
//! othercases confusion("page")
//! @:this can't happen page}{\quad page@>
//! endcases
//!
//! @ @<Initialize the current page, insert the \.{\\topskip} glue...@>=
//! begin if page_contents=empty then freeze_page_specs(box_there)
//! else page_contents:=box_there;
//! q:=new_skip_param(top_skip_code); {now |temp_ptr=glue_ptr(q)|}
//! if width(temp_ptr)>height(p) then width(temp_ptr):=width(temp_ptr)-height(p)
//! else width(temp_ptr):=0;
//! link(q):=p; link(contrib_head):=q; goto continue;
//! end
//!
//! @ @<Prepare to move a box or rule node to the current page...@>=
//! begin page_total:=page_total+page_depth+height(p);
//! page_depth:=depth(p);
//! goto contribute;
//! end
//!
//! @ @<Make sure that |page_max_depth| is not exceeded@>=
//! if page_depth>page_max_depth then
//!   begin page_total:=@|
//!     page_total+page_depth-page_max_depth;@/
//!   page_depth:=page_max_depth;
//!   end;
//!
//! @ @<Update the current page measurements with respect to the glue...@>=
//! if type(p)=kern_node then q:=p
//! else begin q:=glue_ptr(p);
//!   page_so_far[2+stretch_order(q)]:=@|
//!     page_so_far[2+stretch_order(q)]+stretch(q);@/
//!   page_shrink:=page_shrink+shrink(q);
//!   if (shrink_order(q)<>normal)and(shrink(q)<>0) then
//!     begin@t@>@;@/
//!     print_err("Infinite glue shrinkage found on current page");@/
//! @.Infinite glue shrinkage...@>
//!     help4("The page about to be output contains some infinitely")@/
//!       ("shrinkable glue, e.g., `\vss' or `\vskip 0pt minus 1fil'.")@/
//!       ("Such glue doesn't belong there; but you can safely proceed,")@/
//!       ("since the offensive shrinkability has been made finite.");
//!     error;
//!     r:=new_spec(q); shrink_order(r):=normal; delete_glue_ref(q);
//!     glue_ptr(p):=r; q:=r;
//!     end;
//!   end;
//! page_total:=page_total+page_depth+width(q); page_depth:=0
//!
//! @ @<Check if node |p| is a new champion breakpoint; then \(if)...@>=
//! if pi<inf_penalty then
//!   begin @<Compute the badness, |b|, of the current page,
//!     using |awful_bad| if the box is too full@>;
//!   if b<awful_bad then
//!     if pi<=eject_penalty then c:=pi
//!     else  if b<inf_bad then c:=b+pi+insert_penalties
//!       else c:=deplorable
//!   else c:=b;
//!   if insert_penalties>=10000 then c:=awful_bad;
//!   @!stat if tracing_pages>0 then @<Display the page break cost@>;@+tats@;@/
//!   if c<=least_page_cost then
//!     begin best_page_break:=p; best_size:=page_goal;
//!     least_page_cost:=c;
//!     r:=link(page_ins_head);
//!     while r<>page_ins_head do
//!       begin best_ins_ptr(r):=last_ins_ptr(r);
//!       r:=link(r);
//!       end;
//!     end;
//!   if (c=awful_bad)or(pi<=eject_penalty) then
//!     begin fire_up(p); {output the current page at the best place}
//!     if output_active then return; {user's output routine will act}
//!     goto done; {the page has been shipped out by default output routine}
//!     end;
//!   end
//!
//! @ @<Display the page break cost@>=
//! begin begin_diagnostic; print_nl("%");
//! print(" t="); print_totals;@/
//! print(" g="); print_scaled(page_goal);@/
//! print(" b=");
//! if b=awful_bad then print_char("*")@+else print_int(b);
//! @.*\relax@>
//! print(" p="); print_int(pi);
//! print(" c=");
//! if c=awful_bad then print_char("*")@+else print_int(c);
//! if c<=least_page_cost then print_char("#");
//! end_diagnostic(false);
//! end
//!
//! @ @<Compute the badness, |b|, of the current page...@>=
//! if page_total<page_goal then
//!   if (page_so_far[3]<>0) or (page_so_far[4]<>0) or@|
//!     (page_so_far[5]<>0) then b:=0
//!   else b:=badness(page_goal-page_total,page_so_far[2])
//! else if page_total-page_goal>page_shrink then b:=awful_bad
//! else b:=badness(page_total-page_goal,page_shrink)
//!
//! @ @<Append an insertion to the current page and |goto contribute|@>=
//! begin if page_contents=empty then freeze_page_specs(inserts_only);
//! n:=subtype(p); r:=page_ins_head;
//! while n>=subtype(link(r)) do r:=link(r);
//! n:=qo(n);
//! if subtype(r)<>qi(n) then
//!   @<Create a page insertion node with |subtype(r)=qi(n)|, and
//!     include the glue correction for box |n| in the
//!     current page state@>;
//! if type(r)=split_up then insert_penalties:=insert_penalties+float_cost(p)
//! else  begin last_ins_ptr(r):=p;
//!   delta:=page_goal-page_total-page_depth+page_shrink;
//!     {this much room is left if we shrink the maximum}
//!   if count(n)=1000 then h:=height(p)
//!   else h:=x_over_n(height(p),1000)*count(n); {this much room is needed}
//!   if ((h<=0)or(h<=delta))and(height(p)+height(r)<=dimen(n)) then
//!     begin page_goal:=page_goal-h; height(r):=height(r)+height(p);
//!     end
//!   else @<Find the best way to split the insertion, and change
//!     |type(r)| to |split_up|@>;
//!   end;
//! goto contribute;
//! end
//!
//! @ We take note of the value of \.{\\skip} |n| and the height plus depth
//! of \.{\\box}~|n| only when the first \.{\\insert}~|n| node is
//! encountered for a new page. A user who changes the contents of \.{\\box}~|n|
//! after that first \.{\\insert}~|n| had better be either extremely careful
//! or extremely lucky, or both.
//!
//! @<Create a page insertion node...@>=
//! begin q:=get_node(page_ins_node_size); link(q):=link(r); link(r):=q; r:=q;
//! subtype(r):=qi(n); type(r):=inserting; ensure_vbox(n);
//! if box(n)=null then height(r):=0
//! else height(r):=height(box(n))+depth(box(n));
//! best_ins_ptr(r):=null;@/
//! q:=skip(n);
//! if count(n)=1000 then h:=height(r)
//! else h:=x_over_n(height(r),1000)*count(n);
//! page_goal:=page_goal-h-width(q);@/
//! page_so_far[2+stretch_order(q)]:=@|page_so_far[2+stretch_order(q)]+stretch(q);@/
//! page_shrink:=page_shrink+shrink(q);
//! if (shrink_order(q)<>normal)and(shrink(q)<>0) then
//!   begin print_err("Infinite glue shrinkage inserted from "); print_esc("skip");
//! @.Infinite glue shrinkage...@>
//!   print_int(n);
//!   help3("The correction glue for page breaking with insertions")@/
//!     ("must have finite shrinkability. But you may proceed,")@/
//!     ("since the offensive shrinkability has been made finite.");
//!   error;
//!   end;
//! end
//!
//! @ Here is the code that will split a long footnote between pages, in an
//! emergency. The current situation deserves to be recapitulated: Node |p|
//! is an insertion into box |n|; the insertion will not fit, in its entirety,
//! either because it would make the total contents of box |n| greater than
//! \.{\\dimen} |n|, or because it would make the incremental amount of growth
//! |h| greater than the available space |delta|, or both. (This amount |h| has
//! been weighted by the insertion scaling factor, i.e., by \.{\\count} |n|
//! over 1000.) Now we will choose the best way to break the vlist of the
//! insertion, using the same criteria as in the \.{\\vsplit} operation.
//!
//! @<Find the best way to split the insertion...@>=
//! begin if count(n)<=0 then w:=max_dimen
//! else  begin w:=page_goal-page_total-page_depth;
//!   if count(n)<>1000 then w:=x_over_n(w,count(n))*1000;
//!   end;
//! if w>dimen(n)-height(r) then w:=dimen(n)-height(r);
//! q:=vert_break(ins_ptr(p),w,depth(p));
//! height(r):=height(r)+best_height_plus_depth;
//! @!stat if tracing_pages>0 then @<Display the insertion split cost@>;@+tats@;@/
//! if count(n)<>1000 then
//!   best_height_plus_depth:=x_over_n(best_height_plus_depth,1000)*count(n);
//! page_goal:=page_goal-best_height_plus_depth;
//! type(r):=split_up; broken_ptr(r):=q; broken_ins(r):=p;
//! if q=null then insert_penalties:=insert_penalties+eject_penalty
//! else if type(q)=penalty_node then insert_penalties:=insert_penalties+penalty(q);
//! end
//!
//! @ @<Display the insertion split cost@>=
//! begin begin_diagnostic; print_nl("% split"); print_int(n);
//! @.split@>
//! print(" to "); print_scaled(w);
//! print_char(","); print_scaled(best_height_plus_depth);@/
//! print(" p=");
//! if q=null then print_int(eject_penalty)
//! else if type(q)=penalty_node then print_int(penalty(q))
//! else print_char("0");
//! end_diagnostic(false);
//! end
//!
//! @ When the page builder has looked at as much material as could appear before
//! the next page break, it makes its decision. The break that gave minimum
//! badness will be used to put a completed ``page'' into box 255, with insertions
//! appended to their other boxes.
//!
//! We also set the values of |top_mark|, |first_mark|, and |bot_mark|. The
//! program uses the fact that |bot_mark<>null| implies |first_mark<>null|;
//! it also knows that |bot_mark=null| implies |top_mark=first_mark=null|.
//!
//! The |fire_up| subroutine prepares to output the current page at the best
//! place; then it fires up the user's output routine, if there is one,
//! or it simply ships out the page. There is one parameter, |c|, which represents
//! the node that was being contributed to the page when the decision to
//! force an output was made.
//!
//! @<Declare the procedure called |fire_up|@>=
//! procedure fire_up(@!c:pointer);
//! label exit;
//! var p,@!q,@!r,@!s:pointer; {nodes being examined and/or changed}
//! @!prev_p:pointer; {predecessor of |p|}
//! @!n:min_quarterword..255; {insertion box number}
//! @!wait:boolean; {should the present insertion be held over?}
//! @!save_vbadness:integer; {saved value of |vbadness|}
//! @!save_vfuzz: scaled; {saved value of |vfuzz|}
//! @!save_split_top_skip: pointer; {saved value of |split_top_skip|}
//! begin @<Set the value of |output_penalty|@>;
//! if bot_mark<>null then
//!   begin if top_mark<>null then delete_token_ref(top_mark);
//!   top_mark:=bot_mark; add_token_ref(top_mark);
//!   delete_token_ref(first_mark); first_mark:=null;
//!   end;
//! @<Put the \(o)optimal current page into box 255, update |first_mark| and
//!   |bot_mark|, append insertions to their boxes, and put the
//!   remaining nodes back on the contribution list@>;
//! if (top_mark<>null)and(first_mark=null) then
//!   begin first_mark:=top_mark; add_token_ref(top_mark);
//!   end;
//! if output_routine<>null then
//!   if dead_cycles>=max_dead_cycles then
//!     @<Explain that too many dead cycles have occurred in a row@>
//!   else @<Fire up the user's output routine and |return|@>;
//! @<Perform the default output routine@>;
//! exit:end;
//!
//! @ @<Set the value of |output_penalty|@>=
//! if type(best_page_break)=penalty_node then
//!   begin geq_word_define(int_base+output_penalty_code,penalty(best_page_break));
//!   penalty(best_page_break):=inf_penalty;
//!   end
//! else geq_word_define(int_base+output_penalty_code,inf_penalty)
//!
//! @ As the page is finally being prepared for output,
//! pointer |p| runs through the vlist, with |prev_p| trailing behind;
//! pointer |q| is the tail of a list of insertions that
//! are being held over for a subsequent page.
//!
//! @<Put the \(o)optimal current page into box 255...@>=
//! if c=best_page_break then best_page_break:=null; {|c| not yet linked in}
//! @<Ensure that box 255 is empty before output@>;
//! insert_penalties:=0; {this will count the number of insertions held over}
//! save_split_top_skip:=split_top_skip;
//! if holding_inserts<=0 then
//!   @<Prepare all the boxes involved in insertions to act as queues@>;
//! q:=hold_head; link(q):=null; prev_p:=page_head; p:=link(prev_p);
//! while p<>best_page_break do
//!   begin if type(p)=ins_node then
//!     begin if holding_inserts<=0 then
//!        @<Either insert the material specified by node |p| into the
//!          appropriate box, or hold it for the next page;
//!          also delete node |p| from the current page@>;
//!     end
//!   else if type(p)=mark_node then @<Update the values of
//!     |first_mark| and |bot_mark|@>;
//!   prev_p:=p; p:=link(prev_p);
//!   end;
//! split_top_skip:=save_split_top_skip;
//! @<Break the current page at node |p|, put it in box~255,
//!   and put the remaining nodes on the contribution list@>;
//! @<Delete \(t)the page-insertion nodes@>
//!
//! @ @<Ensure that box 255 is empty before output@>=
//! if box(255)<>null then
//!   begin print_err(""); print_esc("box"); print("255 is not void");
//! @:box255}{\.{\\box255 is not void}@>
//!   help2("You shouldn't use \box255 except in \output routines.")@/
//!     ("Proceed, and I'll discard its present contents.");
//!   box_error(255);
//!   end
//!
//! @ @<Update the values of |first_mark| and |bot_mark|@>=
//! begin if first_mark=null then
//!   begin first_mark:=mark_ptr(p);
//!   add_token_ref(first_mark);
//!   end;
//! if bot_mark<>null then delete_token_ref(bot_mark);
//! bot_mark:=mark_ptr(p); add_token_ref(bot_mark);
//! end
//!
//! @ When the following code is executed, the current page runs from node
//! |link(page_head)| to node |prev_p|, and the nodes from |p| to |page_tail|
//! are to be placed back at the front of the contribution list. Furthermore
//! the heldover insertions appear in a list from |link(hold_head)| to |q|; we
//! will put them into the current page list for safekeeping while the user's
//! output routine is active.  We might have |q=hold_head|; and |p=null| if
//! and only if |prev_p=page_tail|. Error messages are suppressed within
//! |vpackage|, since the box might appear to be overfull or underfull simply
//! because the stretch and shrink from the \.{\\skip} registers for inserts
//! are not actually present in the box.
//!
//! @<Break the current page at node |p|, put it...@>=
//! if p<>null then
//!   begin if link(contrib_head)=null then
//!     if nest_ptr=0 then tail:=page_tail
//!     else contrib_tail:=page_tail;
//!   link(page_tail):=link(contrib_head);
//!   link(contrib_head):=p;
//!   link(prev_p):=null;
//!   end;
//! save_vbadness:=vbadness; vbadness:=inf_bad;
//! save_vfuzz:=vfuzz; vfuzz:=max_dimen; {inhibit error messages}
//! box(255):=vpackage(link(page_head),best_size,exactly,page_max_depth);
//! vbadness:=save_vbadness; vfuzz:=save_vfuzz;
//! if last_glue<>max_halfword then delete_glue_ref(last_glue);
//! @<Start a new current page@>; {this sets |last_glue:=max_halfword|}
//! if q<>hold_head then
//!   begin link(page_head):=link(hold_head); page_tail:=q;
//!   end
//!
//! @ If many insertions are supposed to go into the same box, we want to know
//! the position of the last node in that box, so that we don't need to waste time
//! when linking further information into it. The |last_ins_ptr| fields of the
//! page insertion nodes are therefore used for this purpose during the
//! packaging phase.
//!
//! @<Prepare all the boxes involved in insertions to act as queues@>=
//! begin r:=link(page_ins_head);
//! while r<>page_ins_head do
//!   begin if best_ins_ptr(r)<>null then
//!     begin n:=qo(subtype(r)); ensure_vbox(n);
//!     if box(n)=null then box(n):=new_null_box;
//!     p:=box(n)+list_offset;
//!     while link(p)<>null do p:=link(p);
//!     last_ins_ptr(r):=p;
//!     end;
//!   r:=link(r);
//!   end;
//! end
//!
//! @ @<Delete \(t)the page-insertion nodes@>=
//! r:=link(page_ins_head);
//! while r<>page_ins_head do
//!   begin q:=link(r); free_node(r,page_ins_node_size); r:=q;
//!   end;
//! link(page_ins_head):=page_ins_head
//!
//! @ We will set |best_ins_ptr:=null| and package the box corresponding to
//! insertion node~|r|, just after making the final insertion into that box.
//! If this final insertion is `|split_up|', the remainder after splitting
//! and pruning (if any) will be carried over to the next page.
//!
//! @<Either insert the material specified by node |p| into...@>=
//! begin r:=link(page_ins_head);
//! while subtype(r)<>subtype(p) do r:=link(r);
//! if best_ins_ptr(r)=null then wait:=true
//! else  begin wait:=false; s:=last_ins_ptr(r); link(s):=ins_ptr(p);
//!   if best_ins_ptr(r)=p then
//!     @<Wrap up the box specified by node |r|, splitting node |p| if
//!     called for; set |wait:=true| if node |p| holds a remainder after
//!     splitting@>
//!   else  begin while link(s)<>null do s:=link(s);
//!     last_ins_ptr(r):=s;
//!     end;
//!   end;
//! @<Either append the insertion node |p| after node |q|, and remove it
//!   from the current page, or delete |node(p)|@>;
//! end
//!
//! @ @<Wrap up the box specified by node |r|, splitting node |p| if...@>=
//! begin if type(r)=split_up then
//!   if (broken_ins(r)=p)and(broken_ptr(r)<>null) then
//!     begin while link(s)<>broken_ptr(r) do s:=link(s);
//!     link(s):=null;
//!     split_top_skip:=split_top_ptr(p);
//!     ins_ptr(p):=prune_page_top(broken_ptr(r));
//!     if ins_ptr(p)<>null then
//!       begin temp_ptr:=vpack(ins_ptr(p),natural);
//!       height(p):=height(temp_ptr)+depth(temp_ptr);
//!       free_node(temp_ptr,box_node_size); wait:=true;
//!       end;
//!     end;
//! best_ins_ptr(r):=null;
//! n:=qo(subtype(r));
//! temp_ptr:=list_ptr(box(n));
//! free_node(box(n),box_node_size);
//! box(n):=vpack(temp_ptr,natural);
//! end
//!
//! @ @<Either append the insertion node |p|...@>=
//! link(prev_p):=link(p); link(p):=null;
//! if wait then
//!   begin link(q):=p; q:=p; incr(insert_penalties);
//!   end
//! else  begin delete_glue_ref(split_top_ptr(p));
//!   free_node(p,ins_node_size);
//!   end;
//! p:=prev_p
//!
//! @ The list of heldover insertions, running from |link(page_head)| to
//! |page_tail|, must be moved to the contribution list when the user has
//! specified no output routine.
//!
//! @<Perform the default output routine@>=
//! begin if link(page_head)<>null then
//!   begin if link(contrib_head)=null then
//!     if nest_ptr=0 then tail:=page_tail@+else contrib_tail:=page_tail
//!   else link(page_tail):=link(contrib_head);
//!   link(contrib_head):=link(page_head);
//!   link(page_head):=null; page_tail:=page_head;
//!   end;
//! ship_out(box(255)); box(255):=null;
//! end
//!
//! @ @<Explain that too many dead cycles have occurred in a row@>=
//! begin print_err("Output loop---"); print_int(dead_cycles);
//! @.Output loop...@>
//! print(" consecutive dead cycles");
//! help3("I've concluded that your \output is awry; it never does a")@/
//! ("\shipout, so I'm shipping \box255 out myself. Next time")@/
//! ("increase \maxdeadcycles if you want me to be more patient!"); error;
//! end
//!
//! @ @<Fire up the user's output routine and |return|@>=
//! begin output_active:=true;
//! incr(dead_cycles);
//! push_nest; mode:=-vmode; prev_depth:=ignore_depth; mode_line:=-line;
//! begin_token_list(output_routine,output_text);
//! new_save_level(output_group); normal_paragraph;
//! scan_left_brace;
//! return;
//! end
//!
//! @ When the user's output routine finishes, it has constructed a vlist
//! in internal vertical mode, and \TeX\ will do the following:
//!
//! @<Resume the page builder after an output routine has come to an end@>=
//! begin if (loc<>null) or
//!  ((token_type<>output_text)and(token_type<>backed_up)) then
//!   @<Recover from an unbalanced output routine@>;
//! end_token_list; {conserve stack space in case more outputs are triggered}
//! end_graf; unsave; output_active:=false; insert_penalties:=0;@/
//! @<Ensure that box 255 is empty after output@>;
//! if tail<>head then {current list goes after heldover insertions}
//!   begin link(page_tail):=link(head);
//!   page_tail:=tail;
//!   end;
//! if link(page_head)<>null then {and both go before heldover contributions}
//!   begin if link(contrib_head)=null then contrib_tail:=page_tail;
//!   link(page_tail):=link(contrib_head);
//!   link(contrib_head):=link(page_head);
//!   link(page_head):=null; page_tail:=page_head;
//!   end;
//! pop_nest; build_page;
//! end
//!
//! @ @<Recover from an unbalanced output routine@>=
//! begin print_err("Unbalanced output routine");
//! @.Unbalanced output routine@>
//! help2("Your sneaky output routine has problematic {'s and/or }'s.")@/
//! ("I can't handle that very well; good luck."); error;
//! repeat get_token;
//! until loc=null;
//! end {loops forever if reading from a file, since |null=min_halfword<=0|}
//!
//! @ @<Ensure that box 255 is empty after output@>=
//! if box(255)<>null then
//!   begin print_err("Output routine didn't use all of ");
//!   print_esc("box"); print_int(255);
//! @.Output routine didn't use...@>
//!   help3("Your \output commands should empty \box255,")@/
//!     ("e.g., by saying `\shipout\box255'.")@/
//!     ("Proceed; I'll discard its present contents.");
//!   box_error(255);
//!   end
//!
