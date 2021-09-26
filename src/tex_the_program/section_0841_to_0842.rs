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
