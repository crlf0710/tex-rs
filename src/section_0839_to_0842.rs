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
