//! @ The heart of the line-breaking procedure is `|try_break|', a subroutine
//! that tests if the current breakpoint |cur_p| is feasible, by running
//! through the active list to see what lines of text can be made from active
//! nodes to~|cur_p|.  If feasible breaks are possible, new break nodes are
//! created.  If |cur_p| is too far from an active node, that node is
//! deactivated.
//!
//! The parameter |pi| to |try_break| is the penalty associated
//! with a break at |cur_p|; we have |pi=eject_penalty| if the break is forced,
//! and |pi=inf_penalty| if the break is illegal.
//!
//! The other parameter, |break_type|, is set to |hyphenated| or |unhyphenated|,
//! depending on whether or not the current break is at a |disc_node|. The
//! end of a paragraph is also regarded as `|hyphenated|'; this case is
//! distinguishable by the condition |cur_p=null|.
//!
//! @d copy_to_cur_active(#)==cur_active_width[#]:=active_width[#]
//! @d deactivate=60 {go here when node |r| should be deactivated}
//!
//! @<Declare subprocedures for |line_break|@>=
//! procedure try_break(@!pi:integer;@!break_type:small_number);
//! label exit,done,done1,continue,deactivate;
//! var r:pointer; {runs through the active list}
//! @!prev_r:pointer; {stays a step behind |r|}
//! @!old_l:halfword; {maximum line number in current equivalence class of lines}
//! @!no_break_yet:boolean; {have we found a feasible break at |cur_p|?}
//! @<Other local variables for |try_break|@>@;
//! begin @<Make sure that |pi| is in the proper range@>;
//! no_break_yet:=true; prev_r:=active; old_l:=0;
//! do_all_six(copy_to_cur_active);
//! loop@+  begin continue: r:=link(prev_r);
//!   @<If node |r| is of type |delta_node|, update |cur_active_width|,
//!     set |prev_r| and |prev_prev_r|, then |goto continue|@>;
//!   @<If a line number class has ended, create new active nodes for
//!     the best feasible breaks in that class; then |return|
//!     if |r=last_active|, otherwise compute the new |line_width|@>;
//!   @<Consider the demerits for a line from |r| to |cur_p|;
//!     deactivate node |r| if it should no longer be active;
//!     then |goto continue| if a line from |r| to |cur_p| is infeasible,
//!     otherwise record a new feasible break@>;
//!   end;
//! exit: @!stat @<Update the value of |printed_node| for
//!   symbolic displays@>@+tats@;
//! end;
//!
//! @ @<Other local variables for |try_break|@>=
//! @!prev_prev_r:pointer; {a step behind |prev_r|, if |type(prev_r)=delta_node|}
//! @!s:pointer; {runs through nodes ahead of |cur_p|}
//! @!q:pointer; {points to a new node being created}
//! @!v:pointer; {points to a glue specification or a node ahead of |cur_p|}
//! @!t:integer; {node count, if |cur_p| is a discretionary node}
//! @!f:internal_font_number; {used in character width calculation}
//! @!l:halfword; {line number of current active node}
//! @!node_r_stays_active:boolean; {should node |r| remain in the active list?}
//! @!line_width:scaled; {the current line will be justified to this width}
//! @!fit_class:very_loose_fit..tight_fit; {possible fitness class of test line}
//! @!b:halfword; {badness of test line}
//! @!d:integer; {demerits of test line}
//! @!artificial_demerits:boolean; {has |d| been forced to zero?}
//! @!save_link:pointer; {temporarily holds value of |link(cur_p)|}
//! @!shortfall:scaled; {used in badness calculations}
//!
//! @ @<Make sure that |pi| is in the proper range@>=
//! if abs(pi)>=inf_penalty then
//!   if pi>0 then return {this breakpoint is inhibited by infinite penalty}
//!   else pi:=eject_penalty {this breakpoint will be forced}
//!
//! @ The following code uses the fact that |type(last_active)<>delta_node|.
//!
//! @d update_width(#)==@|
//!   cur_active_width[#]:=cur_active_width[#]+mem[r+#].sc
//!
//! @<If node |r|...@>=
//! @^inner loop@>
//! if type(r)=delta_node then
//!   begin do_all_six(update_width);
//!   prev_prev_r:=prev_r; prev_r:=r; goto continue;
//!   end
//!
//! @ As we consider various ways to end a line at |cur_p|, in a given line number
//! class, we keep track of the best total demerits known, in an array with
//! one entry for each of the fitness classifications. For example,
//! |minimal_demerits[tight_fit]| contains the fewest total demerits of feasible
//! line breaks ending at |cur_p| with a |tight_fit| line; |best_place[tight_fit]|
//! points to the passive node for the break before~|cur_p| that achieves such
//! an optimum; and |best_pl_line[tight_fit]| is the |line_number| field in the
//! active node corresponding to |best_place[tight_fit]|. When no feasible break
//! sequence is known, the |minimal_demerits| entries will be equal to
//! |awful_bad|, which is $2^{30}-1$. Another variable, |minimum_demerits|,
//! keeps track of the smallest value in the |minimal_demerits| array.
//!
//! @d awful_bad==@'7777777777 {more than a billion demerits}
//!
//! @<Global...@>=
//! @!minimal_demerits:array[very_loose_fit..tight_fit] of integer; {best total
//!   demerits known for current line class and position, given the fitness}
//! @!minimum_demerits:integer; {best total demerits known for current line class
//!   and position}
//! @!best_place:array[very_loose_fit..tight_fit] of pointer; {how to achieve
//!   |minimal_demerits|}
//! @!best_pl_line:array[very_loose_fit..tight_fit] of halfword; {corresponding
//!   line number}
//!
//! @ @<Get ready to start...@>=
//! minimum_demerits:=awful_bad;
//! minimal_demerits[tight_fit]:=awful_bad;
//! minimal_demerits[decent_fit]:=awful_bad;
//! minimal_demerits[loose_fit]:=awful_bad;
//! minimal_demerits[very_loose_fit]:=awful_bad;
//!
//! @ The first part of the following code is part of \TeX's inner loop, so
//! we don't want to waste any time. The current active node, namely node |r|,
//! contains the line number that will be considered next. At the end of the
//! list we have arranged the data structure so that |r=last_active| and
//! |line_number(last_active)>old_l|.
//! @^inner loop@>
//!
//! @<If a line number class...@>=
//! begin l:=line_number(r);
//! if l>old_l then
//!   begin {now we are no longer in the inner loop}
//!   if (minimum_demerits<awful_bad)and@|
//!       ((old_l<>easy_line)or(r=last_active)) then
//!     @<Create new active nodes for the best feasible breaks
//!       just found@>;
//!   if r=last_active then return;
//!   @<Compute the new line width@>;
//!   end;
//! end
//!
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
//! @ The length of lines depends on whether the user has specified
//! \.{\\parshape} or \.{\\hangindent}. If |par_shape_ptr| is not null, it
//! points to a $(2n+1)$-word record in |mem|, where the |info| in the first
//! word contains the value of |n|, and the other $2n$ words contain the left
//! margins and line lengths for the first |n| lines of the paragraph; the
//! specifications for line |n| apply to all subsequent lines. If
//! |par_shape_ptr=null|, the shape of the paragraph depends on the value of
//! |n=hang_after|; if |n>=0|, hanging indentation takes place on lines |n+1|,
//! |n+2|, \dots, otherwise it takes place on lines 1, \dots, $\vert
//! n\vert$. When hanging indentation is active, the left margin is
//! |hang_indent|, if |hang_indent>=0|, else it is 0; the line length is
//! $|hsize|-\vert|hang_indent|\vert$. The normal setting is
//! |par_shape_ptr=null|, |hang_after=1|, and |hang_indent=0|.
//! Note that if |hang_indent=0|, the value of |hang_after| is irrelevant.
//! @^length of lines@> @^hanging indentation@>
//!
//! @<Glob...@>=
//! @!easy_line:halfword; {line numbers |>easy_line| are equivalent in break nodes}
//! @!last_special_line:halfword; {line numbers |>last_special_line| all have
//!   the same width}
//! @!first_width:scaled; {the width of all lines |<=last_special_line|, if
//!   no \.{\\parshape} has been specified}
//! @!second_width:scaled; {the width of all lines |>last_special_line|}
//! @!first_indent:scaled; {left margin to go with |first_width|}
//! @!second_indent:scaled; {left margin to go with |second_width|}
//!
//! @ We compute the values of |easy_line| and the other local variables relating
//! to line length when the |line_break| procedure is initializing itself.
//!
//! @<Get ready to start...@>=
//! if par_shape_ptr=null then
//!   if hang_indent=0 then
//!     begin last_special_line:=0; second_width:=hsize;
//!     second_indent:=0;
//!     end
//!   else @<Set line length parameters in preparation for hanging indentation@>
//! else  begin last_special_line:=info(par_shape_ptr)-1;
//!   second_width:=mem[par_shape_ptr+2*(last_special_line+1)].sc;
//!   second_indent:=mem[par_shape_ptr+2*last_special_line+1].sc;
//!   end;
//! if looseness=0 then easy_line:=last_special_line
//! else easy_line:=max_halfword
//!
//! @ @<Set line length parameters in preparation for hanging indentation@>=
//! begin last_special_line:=abs(hang_after);
//! if hang_after<0 then
//!   begin first_width:=hsize-abs(hang_indent);
//!   if hang_indent>=0 then first_indent:=hang_indent
//!   else first_indent:=0;
//!   second_width:=hsize; second_indent:=0;
//!   end
//! else  begin first_width:=hsize; first_indent:=0;
//!   second_width:=hsize-abs(hang_indent);
//!   if hang_indent>=0 then second_indent:=hang_indent
//!   else second_indent:=0;
//!   end;
//! end
//!
//! @ When we come to the following code, we have just encountered the first
//! active node~|r| whose |line_number| field contains |l|. Thus we want to
//! compute the length of the $l\mskip1mu$th line of the current paragraph. Furthermore,
//! we want to set |old_l| to the last number in the class of line numbers
//! equivalent to~|l|.
//!
//! @<Compute the new line width@>=
//! if l>easy_line then
//!   begin line_width:=second_width; old_l:=max_halfword-1;
//!   end
//! else  begin old_l:=l;
//!   if l>last_special_line then line_width:=second_width
//!   else if par_shape_ptr=null then line_width:=first_width
//!   else line_width:=mem[par_shape_ptr+2*l@,].sc;
//!   end
//!
//! @ The remaining part of |try_break| deals with the calculation of
//! demerits for a break from |r| to |cur_p|.
//!
//! The first thing to do is calculate the badness, |b|. This value will always
//! be between zero and |inf_bad+1|; the latter value occurs only in the
//! case of lines from |r| to |cur_p| that cannot shrink enough to fit the necessary
//! width. In such cases, node |r| will be deactivated.
//! We also deactivate node~|r| when a break at~|cur_p| is forced, since future
//! breaks must go through a forced break.
//!
//! @<Consider the demerits for a line from |r| to |cur_p|...@>=
//! begin artificial_demerits:=false;@/
//! @^inner loop@>
//! shortfall:=line_width-cur_active_width[1]; {we're this much too short}
//! if shortfall>0 then
//!   @<Set the value of |b| to the badness for stretching the line,
//!     and compute the corresponding |fit_class|@>
//! else @<Set the value of |b| to the badness for shrinking the line,
//!     and compute the corresponding |fit_class|@>;
//! if (b>inf_bad)or(pi=eject_penalty) then
//!   @<Prepare to deactivate node~|r|, and |goto deactivate| unless
//!     there is a reason to consider lines of text from |r| to |cur_p|@>
//! else  begin prev_r:=r;
//!   if b>threshold then goto continue;
//!   node_r_stays_active:=true;
//!   end;
//! @<Record a new feasible break@>;
//! if node_r_stays_active then goto continue; {|prev_r| has been set to |r|}
//! deactivate: @<Deactivate node |r|@>;
//! end
//!
//! @ When a line must stretch, the available stretchability can be found in the
//! subarray |cur_active_width[2..5]|, in units of points, fil, fill, and filll.
//!
//! The present section is part of \TeX's inner loop, and it is most often performed
//! when the badness is infinite; therefore it is worth while to make a quick
//! test for large width excess and small stretchability, before calling the
//! |badness| subroutine.
//! @^inner loop@>
//!
//! @<Set the value of |b| to the badness for stretching...@>=
//! if (cur_active_width[3]<>0)or(cur_active_width[4]<>0)or@|
//!   (cur_active_width[5]<>0) then
//!   begin b:=0; fit_class:=decent_fit; {infinite stretch}
//!   end
//! else  begin if shortfall>7230584 then if cur_active_width[2]<1663497 then
//!     begin b:=inf_bad; fit_class:=very_loose_fit; goto done1;
//!     end;
//!   b:=badness(shortfall,cur_active_width[2]);
//!   if b>12 then
//!     if b>99 then fit_class:=very_loose_fit
//!     else fit_class:=loose_fit
//!   else fit_class:=decent_fit;
//!   done1:
//!   end
//!
//! @ Shrinkability is never infinite in a paragraph;
//! we can shrink the line from |r| to |cur_p| by at most |cur_active_width[6]|.
//!
//! @<Set the value of |b| to the badness for shrinking...@>=
//! begin if -shortfall>cur_active_width[6] then b:=inf_bad+1
//! else b:=badness(-shortfall,cur_active_width[6]);
//! if b>12 then fit_class:=tight_fit@+else fit_class:=decent_fit;
//! end
//!
//! @ During the final pass, we dare not lose all active nodes, lest we lose
//! touch with the line breaks already found. The code shown here makes sure
//! that such a catastrophe does not happen, by permitting overfull boxes as
//! a last resort. This particular part of \TeX\ was a source of several subtle
//! bugs before the correct program logic was finally discovered; readers
//! who seek to ``improve'' \TeX\ should therefore think thrice before daring
//! to make any changes here.
//! @^overfull boxes@>
//!
//! @<Prepare to deactivate node~|r|, and |goto deactivate| unless...@>=
//! begin if final_pass and (minimum_demerits=awful_bad) and@|
//!    (link(r)=last_active) and
//!    (prev_r=active) then
//!   artificial_demerits:=true {set demerits zero, this break is forced}
//! else if b>threshold then goto deactivate;
//! node_r_stays_active:=false;
//! end
//!
//! @ When we get to this part of the code, the line from |r| to |cur_p| is
//! feasible, its badness is~|b|, and its fitness classification is |fit_class|.
//! We don't want to make an active node for this break yet, but we will
//! compute the total demerits and record them in the |minimal_demerits| array,
//! if such a break is the current champion among all ways to get to |cur_p|
//! in a given line-number class and fitness class.
//!
//! @<Record a new feasible break@>=
//! if artificial_demerits then d:=0
//! else @<Compute the demerits, |d|, from |r| to |cur_p|@>;
//! @!stat if tracing_paragraphs>0 then
//!   @<Print a symbolic description of this feasible break@>;
//! tats@;@/
//! d:=d+total_demerits(r); {this is the minimum total demerits
//!   from the beginning to |cur_p| via |r|}
//! if d<=minimal_demerits[fit_class] then
//!   begin minimal_demerits[fit_class]:=d;
//!   best_place[fit_class]:=break_node(r); best_pl_line[fit_class]:=l;
//!   if d<minimum_demerits then minimum_demerits:=d;
//!   end
//!
//! @ @<Print a symbolic description of this feasible break@>=
//! begin if printed_node<>cur_p then
//!   @<Print the list between |printed_node| and |cur_p|,
//!     then set |printed_node:=cur_p|@>;
//! print_nl("@@");
//! @.\AT!@>
//! if cur_p=null then print_esc("par")
//! else if type(cur_p)<>glue_node then
//!   begin if type(cur_p)=penalty_node then print_esc("penalty")
//!   else if type(cur_p)=disc_node then print_esc("discretionary")
//!   else if type(cur_p)=kern_node then print_esc("kern")
//!   else print_esc("math");
//!   end;
//! print(" via @@@@");
//! if break_node(r)=null then print_char("0")
//! else print_int(serial(break_node(r)));
//! print(" b=");
//! if b>inf_bad then print_char("*")@+else print_int(b);
//! @.*\relax@>
//! print(" p="); print_int(pi); print(" d=");
//! if artificial_demerits then print_char("*")@+else print_int(d);
//! end
//!
//! @ @<Print the list between |printed_node| and |cur_p|...@>=
//! begin print_nl("");
//! if cur_p=null then short_display(link(printed_node))
//! else  begin save_link:=link(cur_p);
//!   link(cur_p):=null; print_nl(""); short_display(link(printed_node));
//!   link(cur_p):=save_link;
//!   end;
//! printed_node:=cur_p;
//! end
//!
//! @ When the data for a discretionary break is being displayed, we will have
//! printed the |pre_break| and |post_break| lists; we want to skip over the
//! third list, so that the discretionary data will not appear twice.  The
//! following code is performed at the very end of |try_break|.
//!
//! @<Update the value of |printed_node|...@>=
//! if cur_p=printed_node then if cur_p<>null then if type(cur_p)=disc_node then
//!   begin t:=replace_count(cur_p);
//!   while t>0 do
//!     begin decr(t); printed_node:=link(printed_node);
//!     end;
//!   end
//!
//! @ @<Compute the demerits, |d|, from |r| to |cur_p|@>=
//! begin d:=line_penalty+b;
//! if abs(d)>=10000 then d:=100000000@+else d:=d*d;
//! if pi<>0 then
//!   if pi>0 then d:=d+pi*pi
//!   else if pi>eject_penalty then d:=d-pi*pi;
//! if (break_type=hyphenated)and(type(r)=hyphenated) then
//!   if cur_p<>null then d:=d+double_hyphen_demerits
//!   else d:=d+final_hyphen_demerits;
//! if abs(fit_class-fitness(r))>1 then d:=d+adj_demerits;
//! end
//!
//! @ When an active node disappears, we must delete an adjacent delta node if the
//! active node was at the beginning or the end of the active list, or if it
//! was surrounded by delta nodes. We also must preserve the property that
//! |cur_active_width| represents the length of material from |link(prev_r)|
//! to~|cur_p|.
//!
//! @d combine_two_deltas(#)==@|mem[prev_r+#].sc:=mem[prev_r+#].sc+mem[r+#].sc
//! @d downdate_width(#)==@|cur_active_width[#]:=cur_active_width[#]-
//!   mem[prev_r+#].sc
//!
//! @<Deactivate node |r|@>=
//! link(prev_r):=link(r); free_node(r,active_node_size);
//! if prev_r=active then @<Update the active widths, since the first active
//!   node has been deleted@>
//! else if type(prev_r)=delta_node then
//!   begin r:=link(prev_r);
//!   if r=last_active then
//!     begin do_all_six(downdate_width);
//!     link(prev_prev_r):=last_active;
//!     free_node(prev_r,delta_node_size); prev_r:=prev_prev_r;
//!     end
//!   else if type(r)=delta_node then
//!     begin do_all_six(update_width);
//!     do_all_six(combine_two_deltas);
//!     link(prev_r):=link(r); free_node(r,delta_node_size);
//!     end;
//!   end
//!
//! @ The following code uses the fact that |type(last_active)<>delta_node|. If the
//! active list has just become empty, we do not need to update the
//! |active_width| array, since it will be initialized when an active
//! node is next inserted.
//!
//! @d update_active(#)==active_width[#]:=active_width[#]+mem[r+#].sc
//!
//! @<Update the active widths,...@>=
//! begin r:=link(active);
//! if type(r)=delta_node then
//!   begin do_all_six(update_active);
//!   do_all_six(copy_to_cur_active);
//!   link(active):=link(r); free_node(r,delta_node_size);
//!   end;
//! end
//!
