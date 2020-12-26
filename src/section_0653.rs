//! @ The code here implicitly uses the fact that running dimensions are
//! indicated by |null_flag|, which will be ignored in the calculations
//! because it is a highly negative number.
//!
//! @<Incorporate box dimensions into the dimensions of the hbox...@>=
//! begin x:=x+width(p);
//! if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
//! if height(p)-s>h then h:=height(p)-s;
//! if depth(p)+s>d then d:=depth(p)+s;
//! end
//!
