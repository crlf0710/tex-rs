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
