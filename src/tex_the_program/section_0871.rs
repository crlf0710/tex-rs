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
