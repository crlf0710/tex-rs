//! @ @<Incorporate glue into the horizontal totals@>=
//! begin g:=glue_ptr(p); x:=x+width(g);@/
//! o:=stretch_order(g); total_stretch[o]:=total_stretch[o]+stretch(g);
//! o:=shrink_order(g); total_shrink[o]:=total_shrink[o]+shrink(g);
//! if subtype(p)>=a_leaders then
//!   begin g:=leader_ptr(p);
//!   if height(g)>h then h:=height(g);
//!   if depth(g)>d then d:=depth(g);
//!   end;
//! end
//!
