//! @* \[39] Breaking paragraphs into lines, continued.
//! So far we have gotten a little way into the |line_break| routine, having
//! covered its important |try_break| subroutine. Now let's consider the
//! rest of the process.
//!
//! The main loop of |line_break| traverses the given hlist,
//! starting at |link(temp_head)|, and calls |try_break| at each legal
//! breakpoint. A variable called |auto_breaking| is set to true except
//! within math formulas, since glue nodes are not legal breakpoints when
//! they appear in formulas.
//!
//! The current node of interest in the hlist is pointed to by |cur_p|. Another
//! variable, |prev_p|, is usually one step behind |cur_p|, but the real
//! meaning of |prev_p| is this: If |type(cur_p)=glue_node| then |cur_p| is a legal
//! breakpoint if and only if |auto_breaking| is true and |prev_p| does not
//! point to a glue node, penalty node, explicit kern node, or math node.
//!
//! The following declarations provide for a few other local variables that are
//! used in special calculations.
//!
//! @<Local variables for line breaking@>=
//! @!auto_breaking:boolean; {is node |cur_p| outside a formula?}
//! @!prev_p:pointer; {helps to determine when glue nodes are breakpoints}
//! @!q,@!r,@!s,@!prev_s:pointer; {miscellaneous nodes of temporary interest}
//! @!f:internal_font_number; {used when calculating character widths}
//!
//! @ The `\ignorespaces|loop|\unskip' in the following code is performed at most
//! thrice per call of |line_break|, since it is actually a pass over the
//! entire paragraph.
//!
//! @<Find optimal breakpoints@>=
//! threshold:=pretolerance;
//! if threshold>=0 then
//!   begin @!stat if tracing_paragraphs>0 then
//!     begin begin_diagnostic; print_nl("@@firstpass");@+end;@;@+tats@;@/
//!   second_pass:=false; final_pass:=false;
//!   end
//! else  begin threshold:=tolerance; second_pass:=true;
//!   final_pass:=(emergency_stretch<=0);
//!   @!stat if tracing_paragraphs>0 then begin_diagnostic;@+tats@;
//!   end;
//! loop@+  begin if threshold>inf_bad then threshold:=inf_bad;
//!   if second_pass then @<Initialize for hyphenating a paragraph@>;
//!   @<Create an active breakpoint representing the beginning of the paragraph@>;
//!   cur_p:=link(temp_head); auto_breaking:=true;@/
//!   prev_p:=cur_p; {glue at beginning is not a legal breakpoint}
//!   while (cur_p<>null)and(link(active)<>last_active) do
//!     @<Call |try_break| if |cur_p| is a legal breakpoint;
//!     on the second pass, also try to hyphenate the next
//!     word, if |cur_p| is a glue node;
//!     then advance |cur_p| to the next node of the paragraph
//!     that could possibly be a legal breakpoint@>;
//!   if cur_p=null then
//!     @<Try the final line break at the end of the paragraph,
//!     and |goto done| if the desired breakpoints have been found@>;
//!   @<Clean up the memory by removing the break nodes@>;
//!   if not second_pass then
//!     begin@!stat if tracing_paragraphs>0 then print_nl("@@secondpass");@;@+tats@/
//!     threshold:=tolerance; second_pass:=true; final_pass:=(emergency_stretch<=0);
//!     end {if at first you don't succeed, \dots}
//!   else begin @!stat if tracing_paragraphs>0 then
//!       print_nl("@@emergencypass");@;@+tats@/
//!     background[2]:=background[2]+emergency_stretch; final_pass:=true;
//!     end;
//!   end;
//! done: @!stat if tracing_paragraphs>0 then
//!   begin end_diagnostic(true); normalize_selector;
//!   end;@+tats@/
//!
//! @ The active node that represents the starting point does not need a
//! corresponding passive node.
//!
//! @d store_background(#)==active_width[#]:=background[#]
//!
//! @<Create an active breakpoint representing the beginning of the paragraph@>=
//! q:=get_node(active_node_size);
//! type(q):=unhyphenated; fitness(q):=decent_fit;
//! link(q):=last_active; break_node(q):=null;
//! line_number(q):=prev_graf+1; total_demerits(q):=0; link(active):=q;
//! do_all_six(store_background);@/
//! passive:=null; printed_node:=temp_head; pass_number:=0;
//! font_in_short_display:=null_font
//!
//! @ @<Clean...@>=
//! q:=link(active);
//! while q<>last_active do
//!   begin cur_p:=link(q);
//!   if type(q)=delta_node then free_node(q,delta_node_size)
//!   else free_node(q,active_node_size);
//!   q:=cur_p;
//!   end;
//! q:=passive;
//! while q<>null do
//!   begin cur_p:=link(q);
//!   free_node(q,passive_node_size);
//!   q:=cur_p;
//!   end
//!
//! @ Here is the main switch in the |line_break| routine, where legal breaks
//! are determined. As we move through the hlist, we need to keep the |active_width|
//! array up to date, so that the badness of individual lines is readily calculated
//! by |try_break|. It is convenient to use the short name |act_width| for
//! the component of active width that represents real width as opposed to glue.
//!
//! @d act_width==active_width[1] {length from first active node to current node}
//! @d kern_break==begin if not is_char_node(link(cur_p)) and auto_breaking then
//!     if type(link(cur_p))=glue_node then try_break(0,unhyphenated);
//!   act_width:=act_width+width(cur_p);
//!   end
//!
//! @<Call |try_break| if |cur_p| is a legal breakpoint...@>=
//! begin if is_char_node(cur_p) then
//!   @<Advance \(c)|cur_p| to the node following the present
//!     string of characters@>;
//! case type(cur_p) of
//! hlist_node,vlist_node,rule_node: act_width:=act_width+width(cur_p);
//! whatsit_node: @<Advance \(p)past a whatsit node in the \(l)|line_break| loop@>;
//! glue_node: begin @<If node |cur_p| is a legal breakpoint, call |try_break|;
//!   then update the active widths by including the glue in |glue_ptr(cur_p)|@>;
//!   if second_pass and auto_breaking then
//!     @<Try to hyphenate the following word@>;
//!   end;
//! kern_node: if subtype(cur_p)=explicit then kern_break
//!   else act_width:=act_width+width(cur_p);
//! ligature_node: begin f:=font(lig_char(cur_p));
//!   act_width:=act_width+char_width(f)(char_info(f)(character(lig_char(cur_p))));
//!   end;
//! disc_node: @<Try to break after a discretionary fragment, then |goto done5|@>;
//! math_node: begin auto_breaking:=(subtype(cur_p)=after); kern_break;
//!   end;
//! penalty_node: try_break(penalty(cur_p),unhyphenated);
//! mark_node,ins_node,adjust_node: do_nothing;
//! othercases confusion("paragraph")
//! @:this can't happen paragraph}{\quad paragraph@>
//! endcases;@/
//! prev_p:=cur_p; cur_p:=link(cur_p);
//! done5:end
//!
//! @ The code that passes over the characters of words in a paragraph is
//! part of \TeX's inner loop, so it has been streamlined for speed. We use
//! the fact that `\.{\\parfillskip}' glue appears at the end of each paragraph;
//! it is therefore unnecessary to check if |link(cur_p)=null| when |cur_p| is a
//! character node.
//! @^inner loop@>
//!
//! @<Advance \(c)|cur_p| to the node following the present string...@>=
//! begin prev_p:=cur_p;
//! repeat f:=font(cur_p);
//! act_width:=act_width+char_width(f)(char_info(f)(character(cur_p)));
//! cur_p:=link(cur_p);
//! until not is_char_node(cur_p);
//! end
//!
//! @ When node |cur_p| is a glue node, we look at |prev_p| to see whether or not
//! a breakpoint is legal at |cur_p|, as explained above.
//!
//! @<If node |cur_p| is a legal breakpoint, call...@>=
//! if auto_breaking then
//!   begin if is_char_node(prev_p) then try_break(0,unhyphenated)
//!   else if precedes_break(prev_p) then try_break(0,unhyphenated)
//!   else if (type(prev_p)=kern_node)and(subtype(prev_p)<>explicit) then
//!     try_break(0,unhyphenated);
//!   end;
//! check_shrinkage(glue_ptr(cur_p)); q:=glue_ptr(cur_p);
//! act_width:=act_width+width(q);@|
//! active_width[2+stretch_order(q)]:=@|
//!   active_width[2+stretch_order(q)]+stretch(q);@/
//! active_width[6]:=active_width[6]+shrink(q)
//!
//! @ The following code knows that discretionary texts contain
//! only character nodes, kern nodes, box nodes, rule nodes, and ligature nodes.
//!
//! @<Try to break after a discretionary fragment...@>=
//! begin s:=pre_break(cur_p); disc_width:=0;
//! if s=null then try_break(ex_hyphen_penalty,hyphenated)
//! else  begin repeat @<Add the width of node |s| to |disc_width|@>;
//!     s:=link(s);
//!   until s=null;
//!   act_width:=act_width+disc_width;
//!   try_break(hyphen_penalty,hyphenated);
//!   act_width:=act_width-disc_width;
//!   end;
//! r:=replace_count(cur_p); s:=link(cur_p);
//! while r>0 do
//!   begin @<Add the width of node |s| to |act_width|@>;
//!   decr(r); s:=link(s);
//!   end;
//! prev_p:=cur_p; cur_p:=s; goto done5;
//! end
//!
//! @ @<Add the width of node |s| to |disc_width|@>=
//! if is_char_node(s) then
//!   begin f:=font(s);
//!   disc_width:=disc_width+char_width(f)(char_info(f)(character(s)));
//!   end
//! else  case type(s) of
//!   ligature_node: begin f:=font(lig_char(s));
//!     disc_width:=disc_width+
//!       char_width(f)(char_info(f)(character(lig_char(s))));
//!     end;
//!   hlist_node,vlist_node,rule_node,kern_node:
//!     disc_width:=disc_width+width(s);
//!   othercases confusion("disc3")
//! @:this can't happen disc3}{\quad disc3@>
//!   endcases
//!
//! @ @<Add the width of node |s| to |act_width|@>=
//! if is_char_node(s) then
//!   begin f:=font(s);
//!   act_width:=act_width+char_width(f)(char_info(f)(character(s)));
//!   end
//! else  case type(s) of
//!   ligature_node: begin f:=font(lig_char(s));
//!     act_width:=act_width+
//!       char_width(f)(char_info(f)(character(lig_char(s))));
//!     end;
//!   hlist_node,vlist_node,rule_node,kern_node:
//!     act_width:=act_width+width(s);
//!   othercases confusion("disc4")
//! @:this can't happen disc4}{\quad disc4@>
//!   endcases
//!
//! @ The forced line break at the paragraph's end will reduce the list of
//! breakpoints so that all active nodes represent breaks at |cur_p=null|.
//! On the first pass, we insist on finding an active node that has the
//! correct ``looseness.'' On the final pass, there will be at least one active
//! node, and we will match the desired looseness as well as we can.
//!
//! The global variable |best_bet| will be set to the active node for the best
//! way to break the paragraph, and a few other variables are used to
//! help determine what is best.
//!
//! @<Glob...@>=
//! @!best_bet:pointer; {use this passive node and its predecessors}
//! @!fewest_demerits:integer; {the demerits associated with |best_bet|}
//! @!best_line:halfword; {line number following the last line of the new paragraph}
//! @!actual_looseness:integer; {the difference between |line_number(best_bet)|
//!   and the optimum |best_line|}
//! @!line_diff:integer; {the difference between the current line number and
//!   the optimum |best_line|}
//!
//! @ @<Try the final line break at the end of the paragraph...@>=
//! begin try_break(eject_penalty,hyphenated);
//! if link(active)<>last_active then
//!   begin @<Find an active node with fewest demerits@>;
//!   if looseness=0 then goto done;
//!   @<Find the best active node for the desired looseness@>;
//!   if (actual_looseness=looseness)or final_pass then goto done;
//!   end;
//! end
//!
//! @ @<Find an active node...@>=
//! r:=link(active); fewest_demerits:=awful_bad;
//! repeat if type(r)<>delta_node then if total_demerits(r)<fewest_demerits then
//!   begin fewest_demerits:=total_demerits(r); best_bet:=r;
//!   end;
//! r:=link(r);
//! until r=last_active;
//! best_line:=line_number(best_bet)
//!
//! @ The adjustment for a desired looseness is a slightly more complicated
//! version of the loop just considered. Note that if a paragraph is broken
//! into segments by displayed equations, each segment will be subject to the
//! looseness calculation, independently of the other segments.
//!
//! @<Find the best active node...@>=
//! begin r:=link(active); actual_looseness:=0;
//! repeat if type(r)<>delta_node then
//!   begin line_diff:=line_number(r)-best_line;
//!   if ((line_diff<actual_looseness)and(looseness<=line_diff))or@|
//!   ((line_diff>actual_looseness)and(looseness>=line_diff)) then
//!     begin best_bet:=r; actual_looseness:=line_diff;
//!     fewest_demerits:=total_demerits(r);
//!     end
//!   else if (line_diff=actual_looseness)and@|
//!     (total_demerits(r)<fewest_demerits) then
//!     begin best_bet:=r; fewest_demerits:=total_demerits(r);
//!     end;
//!   end;
//! r:=link(r);
//! until r=last_active;
//! best_line:=line_number(best_bet);
//! end
//!
