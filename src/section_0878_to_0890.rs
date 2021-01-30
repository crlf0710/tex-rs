//! @ The job of reversing links in a list is conveniently regarded as the job
//! of taking items off one stack and putting them on another. In this case we
//! take them off a stack pointed to by |q| and having |prev_break| fields;
//! we put them on a stack pointed to by |cur_p| and having |next_break| fields.
//! Node |r| is the passive node being moved from stack to stack.
//!
//! @<Reverse the links of the relevant passive nodes...@>=
//! q:=break_node(best_bet); cur_p:=null;
//! repeat r:=q; q:=prev_break(q); next_break(r):=cur_p; cur_p:=r;
//! until q=null
//!
//! @ Glue and penalty and kern and math nodes are deleted at the beginning of
//! a line, except in the anomalous case that the node to be deleted is actually
//! one of the chosen breakpoints. Otherwise
//! the pruning done here is designed to match
//! the lookahead computation in |try_break|, where the |break_width| values
//! are computed for non-discretionary breakpoints.
//!
//! @<Prune unwanted nodes at the beginning of the next line@>=
//! begin r:=temp_head;
//! loop@+  begin q:=link(r);
//!   if q=cur_break(cur_p) then goto done1;
//!     {|cur_break(cur_p)| is the next breakpoint}
//!   {now |q| cannot be |null|}
//!   if is_char_node(q) then goto done1;
//!   if non_discardable(q) then goto done1;
//!   if type(q)=kern_node then if subtype(q)<>explicit then goto done1;
//!   r:=q; {now |type(q)=glue_node|, |kern_node|, |math_node|, or |penalty_node|}
//!   end;
//! done1: if r<>temp_head then
//!   begin link(r):=null; flush_node_list(link(temp_head));
//!   link(temp_head):=q;
//!   end;
//! end
//!
//! @ The current line to be justified appears in a horizontal list starting
//! at |link(temp_head)| and ending at |cur_break(cur_p)|. If |cur_break(cur_p)| is
//! a glue node, we reset the glue to equal the |right_skip| glue; otherwise
//! we append the |right_skip| glue at the right. If |cur_break(cur_p)| is a
//! discretionary node, we modify the list so that the discretionary break
//! is compulsory, and we set |disc_break| to |true|. We also append
//! the |left_skip| glue at the left of the line, unless it is zero.
//!
//! @<Justify the line ending at breakpoint |cur_p|, and append it...@>=
//! @<Modify the end of the line to reflect the nature of the break and to include
//!   \.{\\rightskip}; also set the proper value of |disc_break|@>;
//! @<Put the \(l)\.{\\leftskip} glue at the left and detach this line@>;
//! @<Call the packaging subroutine, setting |just_box| to the justified box@>;
//! @<Append the new box to the current vertical list, followed by the list of
//!   special nodes taken out of the box by the packager@>;
//! @<Append a penalty node, if a nonzero penalty is appropriate@>
//!
//! @ At the end of the following code, |q| will point to the final node on the
//! list about to be justified.
//!
//! @<Modify the end of the line...@>=
//! q:=cur_break(cur_p); disc_break:=false; post_disc_break:=false;
//! if q<>null then {|q| cannot be a |char_node|}
//!   if type(q)=glue_node then
//!     begin delete_glue_ref(glue_ptr(q));
//!     glue_ptr(q):=right_skip;
//!     subtype(q):=right_skip_code+1; add_glue_ref(right_skip);
//!     goto done;
//!     end
//!   else  begin if type(q)=disc_node then
//!       @<Change discretionary to compulsory and set
//!         |disc_break:=true|@>
//!     else if (type(q)=math_node)or(type(q)=kern_node) then width(q):=0;
//!     end
//! else  begin q:=temp_head;
//!   while link(q)<>null do q:=link(q);
//!   end;
//! @<Put the \(r)\.{\\rightskip} glue after node |q|@>;
//! done:
//!
//! @ @<Change discretionary to compulsory...@>=
//! begin t:=replace_count(q);
//! @<Destroy the |t| nodes following |q|, and
//!    make |r| point to the following node@>;
//! if post_break(q)<>null then @<Transplant the post-break list@>;
//! if pre_break(q)<>null then @<Transplant the pre-break list@>;
//! link(q):=r; disc_break:=true;
//! end
//!
//! @ @<Destroy the |t| nodes following |q|...@>=
//! if t=0 then r:=link(q)
//! else  begin r:=q;
//!   while t>1 do
//!     begin r:=link(r); decr(t);
//!     end;
//!   s:=link(r);
//!   r:=link(s); link(s):=null;
//!   flush_node_list(link(q)); replace_count(q):=0;
//!   end
//!
//! @ We move the post-break list from inside node |q| to the main list by
//! re\-attaching it just before the present node |r|, then resetting |r|.
//!
//! @<Transplant the post-break list@>=
//! begin s:=post_break(q);
//! while link(s)<>null do s:=link(s);
//! link(s):=r; r:=post_break(q); post_break(q):=null; post_disc_break:=true;
//! end
//!
//! @ We move the pre-break list from inside node |q| to the main list by
//! re\-attaching it just after the present node |q|, then resetting |q|.
//!
//! @<Transplant the pre-break list@>=
//! begin s:=pre_break(q); link(q):=s;
//! while link(s)<>null do s:=link(s);
//! pre_break(q):=null; q:=s;
//! end
//!
//! @ @<Put the \(r)\.{\\rightskip} glue after node |q|@>=
//! r:=new_param_glue(right_skip_code); link(r):=link(q); link(q):=r; q:=r
//!
//! @ The following code begins with |q| at the end of the list to be
//! justified. It ends with |q| at the beginning of that list, and with
//! |link(temp_head)| pointing to the remainder of the paragraph, if any.
//!
//! @<Put the \(l)\.{\\leftskip} glue at the left...@>=
//! r:=link(q); link(q):=null; q:=link(temp_head); link(temp_head):=r;
//! if left_skip<>zero_glue then
//!   begin r:=new_param_glue(left_skip_code);
//!   link(r):=q; q:=r;
//!   end
//!
//! @ @<Append the new box to the current vertical list...@>=
//! append_to_vlist(just_box);
//! if adjust_head<>adjust_tail then
//!   begin link(tail):=link(adjust_head); tail:=adjust_tail;
//!    end;
//! adjust_tail:=null
//!
//! @ Now |q| points to the hlist that represents the current line of the
//! paragraph. We need to compute the appropriate line width, pack the
//! line into a box of this size, and shift the box by the appropriate
//! amount of indentation.
//!
//! @<Call the packaging subroutine...@>=
//! if cur_line>last_special_line then
//!   begin cur_width:=second_width; cur_indent:=second_indent;
//!   end
//! else if par_shape_ptr=null then
//!   begin cur_width:=first_width; cur_indent:=first_indent;
//!   end
//! else  begin cur_width:=mem[par_shape_ptr+2*cur_line].sc;
//!   cur_indent:=mem[par_shape_ptr+2*cur_line-1].sc;
//!   end;
//! adjust_tail:=adjust_head; just_box:=hpack(q,cur_width,exactly);
//! shift_amount(just_box):=cur_indent
//!
//! @ Penalties between the lines of a paragraph come from club and widow lines,
//! from the |inter_line_penalty| parameter, and from lines that end at
//! discretionary breaks.  Breaking between lines of a two-line paragraph gets
//! both club-line and widow-line penalties. The local variable |pen| will
//! be set to the sum of all relevant penalties for the current line, except
//! that the final line is never penalized.
//!
//! @<Append a penalty node, if a nonzero penalty is appropriate@>=
//! if cur_line+1<>best_line then
//!   begin pen:=inter_line_penalty;
//!   if cur_line=prev_graf+1 then pen:=pen+club_penalty;
//!   if cur_line+2=best_line then pen:=pen+final_widow_penalty;
//!   if disc_break then pen:=pen+broken_penalty;
//!   if pen<>0 then
//!     begin r:=new_penalty(pen);
//!     link(tail):=r; tail:=r;
//!     end;
//!   end
//!
