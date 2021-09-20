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
