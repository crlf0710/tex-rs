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
