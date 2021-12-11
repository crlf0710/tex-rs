//! @ Most of the actual construction work of |mlist_to_hlist| is done
//! by procedures with names
//! like |make_fraction|, |make_radical|, etc. To illustrate
//! the general setup of such procedures, let's begin with a couple of
//! simple ones.
//!
//! @<Declare math...@>=
//! procedure make_over(@!q:pointer);
//! begin info(nucleus(q)):=@|
//!   overbar(clean_box(nucleus(q),cramped_style(cur_style)),@|
//!   3*default_rule_thickness,default_rule_thickness);
//! math_type(nucleus(q)):=sub_box;
//! end;
//!
//! @ @<Declare math...@>=
//! procedure make_under(@!q:pointer);
//! var p,@!x,@!y: pointer; {temporary registers for box construction}
//! @!delta:scaled; {overall height plus depth}
//! begin x:=clean_box(nucleus(q),cur_style);
//! p:=new_kern(3*default_rule_thickness); link(x):=p;
//! link(p):=fraction_rule(default_rule_thickness);
//! y:=vpack(x,natural);
//! delta:=height(y)+depth(y)+default_rule_thickness;
//! height(y):=height(x); depth(y):=delta-height(y);
//! info(nucleus(q)):=y; math_type(nucleus(q)):=sub_box;
//! end;
//!
