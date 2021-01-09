//! @ The height of a `\.{\\vtop}' box is inherited from the first item on its list,
//! if that item is an |hlist_node|, |vlist_node|, or |rule_node|; otherwise
//! the \.{\\vtop} height is zero.
//!
//!
//! @<Readjust the height...@>=
//! begin h:=0; p:=list_ptr(cur_box);
//! if p<>null then if type(p)<=rule_node then h:=height(p);
//! depth(cur_box):=depth(cur_box)-h+height(cur_box); height(cur_box):=h;
//! end
//!
