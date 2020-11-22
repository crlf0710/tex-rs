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
