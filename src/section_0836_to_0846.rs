//! @ It is not necessary to create new active nodes having |minimal_demerits|
//! greater than
//! |minimum_demerits+abs(adj_demerits)|, since such active nodes will never
//! be chosen in the final paragraph breaks. This observation allows us to
//! omit a substantial number of feasible breakpoints from further consideration.
//!
//! @<Create new active nodes...@>=
//! begin if no_break_yet then @<Compute the values of |break_width|@>;
//! @<Insert a delta node to prepare for breaks at |cur_p|@>;
//! if abs(adj_demerits)>=awful_bad-minimum_demerits then
//!   minimum_demerits:=awful_bad-1
//! else minimum_demerits:=minimum_demerits+abs(adj_demerits);
//! for fit_class:=very_loose_fit to tight_fit do
//!   begin if minimal_demerits[fit_class]<=minimum_demerits then
//!     @<Insert a new active node
//!       from |best_place[fit_class]| to |cur_p|@>;
//!   minimal_demerits[fit_class]:=awful_bad;
//!   end;
//! minimum_demerits:=awful_bad;
//! @<Insert a delta node to prepare for the next active node@>;
//! end
//!
//! @ When we insert a new active node for a break at |cur_p|, suppose this
//! new node is to be placed just before active node |a|; then we essentially
//! want to insert `$\delta\,|cur_p|\,\delta^\prime$' before |a|, where
//! $\delta=\alpha(a)-\alpha(|cur_p|)$ and $\delta^\prime=\alpha(|cur_p|)-\alpha(a)$
//! in the notation explained above.  The |cur_active_width| array now holds
//! $\gamma+\beta(|cur_p|)-\alpha(a)$; so $\delta$ can be obtained by
//! subtracting |cur_active_width| from the quantity $\gamma+\beta(|cur_p|)-
//! \alpha(|cur_p|)$. The latter quantity can be regarded as the length of a
//! line ``from |cur_p| to |cur_p|''; we call it the |break_width| at |cur_p|.
//!
//! The |break_width| is usually negative, since it consists of the background
//! (which is normally zero) minus the width of nodes following~|cur_p| that are
//! eliminated after a break. If, for example, node |cur_p| is a glue node, the
//! width of this glue is subtracted from the background; and we also look
//! ahead to eliminate all subsequent glue and penalty and kern and math
//! nodes, subtracting their widths as well.
//!
//! Kern nodes do not disappear at a line break unless they are |explicit|.
//!
//! @d set_break_width_to_background(#)==break_width[#]:=background[#]
//!
//! @<Compute the values of |break...@>=
//! begin no_break_yet:=false; do_all_six(set_break_width_to_background);
//! s:=cur_p;
//! if break_type>unhyphenated then if cur_p<>null then
//!   @<Compute the discretionary |break_width| values@>;
//! while s<>null do
//!   begin if is_char_node(s) then goto done;
//!   case type(s) of
//!   glue_node:@<Subtract glue from |break_width|@>;
//!   penalty_node: do_nothing;
//!   math_node: break_width[1]:=break_width[1]-width(s);
//!   kern_node: if subtype(s)<>explicit then goto done
//!     else break_width[1]:=break_width[1]-width(s);
//!   othercases goto done
//!   endcases;@/
//!   s:=link(s);
//!   end;
//! done: end
//!
//! @ @<Subtract glue from |break...@>=
//! begin v:=glue_ptr(s); break_width[1]:=break_width[1]-width(v);
//! break_width[2+stretch_order(v)]:=break_width[2+stretch_order(v)]-stretch(v);
//! break_width[6]:=break_width[6]-shrink(v);
//! end
//!
//! @ When |cur_p| is a discretionary break, the length of a line ``from |cur_p| to
//! |cur_p|'' has to be defined properly so that the other calculations work out.
//! Suppose that the pre-break text at |cur_p| has length $l_0$, the post-break
//! text has length $l_1$, and the replacement text has length |l|. Suppose
//! also that |q| is the node following the replacement text. Then length of a
//! line from |cur_p| to |q| will be computed as $\gamma+\beta(q)-\alpha(|cur_p|)$,
//! where $\beta(q)=\beta(|cur_p|)-l_0+l$. The actual length will be the background
//! plus $l_1$, so the length from |cur_p| to |cur_p| should be $\gamma+l_0+l_1-l$.
//! If the post-break text of the discretionary is empty, a break may also
//! discard~|q|; in that unusual case we subtract the length of~|q| and any
//! other nodes that will be discarded after the discretionary break.
//!
//! The value of $l_0$ need not be computed, since |line_break| will put
//! it into the global variable |disc_width| before calling |try_break|.
//!
//! @<Glob...@>=
//! @!disc_width:scaled; {the length of discretionary material preceding a break}
//!
//! @ @<Compute the discretionary |break...@>=
//! begin t:=replace_count(cur_p); v:=cur_p; s:=post_break(cur_p);
//! while t>0 do
//!   begin decr(t); v:=link(v);
//!   @<Subtract the width of node |v| from |break_width|@>;
//!   end;
//! while s<>null do
//!   begin @<Add the width of node |s| to |break_width|@>;
//!   s:=link(s);
//!   end;
//! break_width[1]:=break_width[1]+disc_width;
//! if post_break(cur_p)=null then s:=link(v);
//!           {nodes may be discardable after the break}
//! end
//!
//! @ Replacement texts and discretionary texts are supposed to contain
//! only character nodes, kern nodes, ligature nodes, and box or rule nodes.
//!
//! @<Subtract the width of node |v|...@>=
//! if is_char_node(v) then
//!   begin f:=font(v);
//!   break_width[1]:=break_width[1]-char_width(f)(char_info(f)(character(v)));
//!   end
//! else  case type(v) of
//!   ligature_node: begin f:=font(lig_char(v));@/
//!     break_width[1]:=@|break_width[1]-
//!       char_width(f)(char_info(f)(character(lig_char(v))));
//!     end;
//!   hlist_node,vlist_node,rule_node,kern_node:
//!     break_width[1]:=break_width[1]-width(v);
//!   othercases confusion("disc1")
//! @:this can't happen disc1}{\quad disc1@>
//!   endcases
//!
//! @ @<Add the width of node |s| to |b...@>=
//! if is_char_node(s) then
//!   begin f:=font(s);
//!   break_width[1]:=@|break_width[1]+char_width(f)(char_info(f)(character(s)));
//!   end
//! else  case type(s) of
//!   ligature_node: begin f:=font(lig_char(s));
//!     break_width[1]:=break_width[1]+
//!       char_width(f)(char_info(f)(character(lig_char(s))));
//!     end;
//!   hlist_node,vlist_node,rule_node,kern_node:
//!     break_width[1]:=break_width[1]+width(s);
//!   othercases confusion("disc2")
//! @:this can't happen disc2}{\quad disc2@>
//!   endcases
//!
//! @ We use the fact that |type(active)<>delta_node|.
//!
//! @d convert_to_break_width(#)==@|
//!   mem[prev_r+#].sc:=@|@t\hskip10pt@>mem[prev_r+#].sc
//!   -cur_active_width[#]+break_width[#]
//! @d store_break_width(#)==active_width[#]:=break_width[#]
//! @d new_delta_to_break_width(#)==@|
//!   mem[q+#].sc:=break_width[#]-cur_active_width[#]
//!
//! @<Insert a delta node to prepare for breaks at |cur_p|@>=
//! if type(prev_r)=delta_node then {modify an existing delta node}
//!   begin do_all_six(convert_to_break_width);
//!   end
//! else if prev_r=active then {no delta node needed at the beginning}
//!   begin do_all_six(store_break_width);
//!   end
//! else  begin q:=get_node(delta_node_size); link(q):=r; type(q):=delta_node;@/
//!   subtype(q):=0; {the |subtype| is not used}
//!   do_all_six(new_delta_to_break_width);
//!   link(prev_r):=q; prev_prev_r:=prev_r; prev_r:=q;
//!   end
//!
//! @ When the following code is performed, we will have just inserted at
//! least one active node before |r|, so |type(prev_r)<>delta_node|.
//!
//! @d new_delta_from_break_width(#)==@|mem[q+#].sc:=
//!     cur_active_width[#]-break_width[#]
//!
//! @<Insert a delta node to prepare for the next active node@>=
//! if r<>last_active then
//!   begin q:=get_node(delta_node_size); link(q):=r; type(q):=delta_node;@/
//!   subtype(q):=0; {the |subtype| is not used}
//!   do_all_six(new_delta_from_break_width);
//!   link(prev_r):=q; prev_prev_r:=prev_r; prev_r:=q;
//!   end
//!
//! @ When we create an active node, we also create the corresponding
//! passive node.
//!
//! @<Insert a new active node from |best_place[fit_class]| to |cur_p|@>=
//! begin q:=get_node(passive_node_size);
//! link(q):=passive; passive:=q; cur_break(q):=cur_p;
//! @!stat incr(pass_number); serial(q):=pass_number;@+tats@;@/
//! prev_break(q):=best_place[fit_class];@/
//! q:=get_node(active_node_size); break_node(q):=passive;
//! line_number(q):=best_pl_line[fit_class]+1;
//! fitness(q):=fit_class; type(q):=break_type;
//! total_demerits(q):=minimal_demerits[fit_class];
//! link(q):=r; link(prev_r):=q; prev_r:=q;
//! @!stat if tracing_paragraphs>0 then
//!   @<Print a symbolic description of the new break node@>;
//! tats@;@/
//! end
//!
//! @ @<Print a symbolic description of the new break node@>=
//! begin print_nl("@@@@"); print_int(serial(passive));
//! @.\AT!\AT!@>
//! print(": line "); print_int(line_number(q)-1);
//! print_char("."); print_int(fit_class);
//! if break_type=hyphenated then print_char("-");
//! print(" t="); print_int(total_demerits(q));
//! print(" -> @@@@");
//! if prev_break(passive)=null then print_char("0")
//! else print_int(serial(prev_break(passive)));
//! end
//!
