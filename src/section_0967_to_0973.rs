//! @* \[44] Breaking vertical lists into pages.
//! The |vsplit| procedure, which implements \TeX's \.{\\vsplit} operation,
//! is considerably simpler than |line_break| because it doesn't have to
//! worry about hyphenation, and because its mission is to discover a single
//! break instead of an optimum sequence of breakpoints.  But before we get
//! into the details of |vsplit|, we need to consider a few more basic things.
//!
//! @ A subroutine called |prune_page_top| takes a pointer to a vlist and
//! returns a pointer to a modified vlist in which all glue, kern, and penalty nodes
//! have been deleted before the first box or rule node. However, the first
//! box or rule is actually preceded by a newly created glue node designed so that
//! the topmost baseline will be at distance |split_top_skip| from the top,
//! whenever this is possible without backspacing.
//!
//! In this routine and those that follow, we make use of the fact that a
//! vertical list contains no character nodes, hence the |type| field exists
//! for each node in the list.
//! @^data structure assumptions@>
//!
//! @p function prune_page_top(@!p:pointer):pointer; {adjust top after page break}
//! var prev_p:pointer; {lags one step behind |p|}
//! @!q:pointer; {temporary variable for list manipulation}
//! begin prev_p:=temp_head; link(temp_head):=p;
//! while p<>null do
//!   case type(p) of
//!   hlist_node,vlist_node,rule_node:@<Insert glue for |split_top_skip|
//!     and set~|p:=null|@>;
//!   whatsit_node,mark_node,ins_node: begin prev_p:=p; p:=link(prev_p);
//!     end;
//!   glue_node,kern_node,penalty_node: begin q:=p; p:=link(q); link(q):=null;
//!     link(prev_p):=p; flush_node_list(q);
//!     end;
//!   othercases confusion("pruning")
//! @:this can't happen pruning}{\quad pruning@>
//!   endcases;
//! prune_page_top:=link(temp_head);
//! end;
//!
//! @ @<Insert glue for |split_top_skip|...@>=
//! begin q:=new_skip_param(split_top_skip_code); link(prev_p):=q; link(q):=p;
//!   {now |temp_ptr=glue_ptr(q)|}
//! if width(temp_ptr)>height(p) then width(temp_ptr):=width(temp_ptr)-height(p)
//! else width(temp_ptr):=0;
//! p:=null;
//! end
//!
//! @ The next subroutine finds the best place to break a given vertical list
//! so as to obtain a box of height~|h|, with maximum depth~|d|.
//! A pointer to the beginning of the vertical list is given,
//! and a pointer to the optimum breakpoint is returned. The list is effectively
//! followed by a forced break, i.e., a penalty node with the |eject_penalty|;
//! if the best break occurs at this artificial node, the value |null| is returned.
//!
//! An array of six |scaled| distances is used to keep track of the height
//! from the beginning of the list to the current place, just as in |line_break|.
//! In fact, we use one of the same arrays, only changing its name to reflect
//! its new significance.
//!
//! @d active_height==active_width {new name for the six distance variables}
//! @d cur_height==active_height[1] {the natural height}
//! @d set_height_zero(#)==active_height[#]:=0 {initialize the height to zero}
//! @#
//! @d update_heights=90 {go here to record glue in the |active_height| table}
//!
//! @p function vert_break(@!p:pointer; @!h,@!d:scaled):pointer;
//!   {finds optimum page break}
//! label done,not_found,update_heights;
//! var prev_p:pointer; {if |p| is a glue node, |type(prev_p)| determines
//!   whether |p| is a legal breakpoint}
//! @!q,@!r:pointer; {glue specifications}
//! @!pi:integer; {penalty value}
//! @!b:integer; {badness at a trial breakpoint}
//! @!least_cost:integer; {the smallest badness plus penalties found so far}
//! @!best_place:pointer; {the most recent break that leads to |least_cost|}
//! @!prev_dp:scaled; {depth of previous box in the list}
//! @!t:small_number; {|type| of the node following a kern}
//! begin prev_p:=p; {an initial glue node is not a legal breakpoint}
//! least_cost:=awful_bad; do_all_six(set_height_zero); prev_dp:=0;
//! loop@+  begin @<If node |p| is a legal breakpoint, check if this break is
//!     the best known, and |goto done| if |p| is null or
//!     if the page-so-far is already too full to accept more stuff@>;
//!   prev_p:=p; p:=link(prev_p);
//!   end;
//! done: vert_break:=best_place;
//! end;
//!
//! @ A global variable |best_height_plus_depth| will be set to the natural size
//! of the box that corresponds to the optimum breakpoint found by |vert_break|.
//! (This value is used by the insertion-splitting algorithm of the page builder.)
//!
//! @<Glob...@>=
//! @!best_height_plus_depth:scaled; {height of the best box, without stretching or
//!   shrinking}
//!
//! @ A subtle point to be noted here is that the maximum depth~|d| might be
//! negative, so |cur_height| and |prev_dp| might need to be corrected even
//! after a glue or kern node.
//!
//! @<If node |p| is a legal breakpoint, check...@>=
//! if p=null then pi:=eject_penalty
//! else  @<Use node |p| to update the current height and depth measurements;
//!     if this node is not a legal breakpoint, |goto not_found|
//!     or |update_heights|,
//!     otherwise set |pi| to the associated penalty at the break@>;
//! @<Check if node |p| is a new champion breakpoint; then \(go)|goto done|
//!   if |p| is a forced break or if the page-so-far is already too full@>;
//! if (type(p)<glue_node)or(type(p)>kern_node) then goto not_found;
//! update_heights: @<Update the current height and depth measurements with
//!   respect to a glue or kern node~|p|@>;
//! not_found: if prev_dp>d then
//!     begin cur_height:=cur_height+prev_dp-d;
//!     prev_dp:=d;
//!     end;
//!
//! @ @<Use node |p| to update the current height and depth measurements...@>=
//! case type(p) of
//! hlist_node,vlist_node,rule_node: begin@t@>@;@/
//!   cur_height:=cur_height+prev_dp+height(p); prev_dp:=depth(p);
//!   goto not_found;
//!   end;
//! whatsit_node:@<Process whatsit |p| in |vert_break| loop, |goto not_found|@>;
//! glue_node: if precedes_break(prev_p) then pi:=0
//!   else goto update_heights;
//! kern_node: begin if link(p)=null then t:=penalty_node
//!   else t:=type(link(p));
//!   if t=glue_node then pi:=0@+else goto update_heights;
//!   end;
//! penalty_node: pi:=penalty(p);
//! mark_node,ins_node: goto not_found;
//! othercases confusion("vertbreak")
//! @:this can't happen vertbreak}{\quad vertbreak@>
//! endcases
//!
