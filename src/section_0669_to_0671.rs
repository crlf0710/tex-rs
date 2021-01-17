//! @ @<Examine node |p| in the vlist, taking account of its effect...@>=
//! begin if is_char_node(p) then confusion("vpack")
//! @:this can't happen vpack}{\quad vpack@>
//! else  case type(p) of
//!   hlist_node,vlist_node,rule_node,unset_node:
//!     @<Incorporate box dimensions into the dimensions of
//!       the vbox that will contain~it@>;
//!   whatsit_node:@<Incorporate a whatsit node into a vbox@>;
//!   glue_node: @<Incorporate glue into the vertical totals@>;
//!   kern_node: begin x:=x+d+width(p); d:=0;
//!     end;
//!   othercases do_nothing
//!   endcases;
//! p:=link(p);
//! end
//!
//! @ @<Incorporate box dimensions into the dimensions of the vbox...@>=
//! begin x:=x+d+height(p); d:=depth(p);
//! if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
//! if width(p)+s>w then w:=width(p)+s;
//! end
//!
//! @ @<Incorporate glue into the vertical totals@>=
//! begin x:=x+d; d:=0;@/
//! g:=glue_ptr(p); x:=x+width(g);@/
//! o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
//! o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
//! if subtype(p)>=a_leaders then
//!   begin g:=leader_ptr(p);
//!   if width(g)>w then w:=width(g);
//!   end;
//! end
//!
