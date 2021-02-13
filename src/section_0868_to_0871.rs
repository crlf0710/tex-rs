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
