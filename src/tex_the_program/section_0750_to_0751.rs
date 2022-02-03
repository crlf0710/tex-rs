//! @ The following program builds a vlist box |v| for displayed limits. The
//! width of the box is not affected by the fact that the limits may be skewed.
//!
//! @<Construct a box with limits above and below it...@>=
//! begin x:=clean_box(supscr(q),sup_style(cur_style));
//! y:=clean_box(nucleus(q),cur_style);
//! z:=clean_box(subscr(q),sub_style(cur_style));
//! v:=new_null_box; type(v):=vlist_node; width(v):=width(y);
//! if width(x)>width(v) then width(v):=width(x);
//! if width(z)>width(v) then width(v):=width(z);
//! x:=rebox(x,width(v)); y:=rebox(y,width(v)); z:=rebox(z,width(v));@/
//! shift_amount(x):=half(delta); shift_amount(z):=-shift_amount(x);
//! height(v):=height(y); depth(v):=depth(y);
//! @<Attach the limits to |y| and adjust |height(v)|, |depth(v)| to
//!   account for their presence@>;
//! new_hlist(q):=v;
//! end
//!
//! @ We use |shift_up| and |shift_down| in the following program for the
//! amount of glue between the displayed operator |y| and its limits |x| and
//! |z|. The vlist inside box |v| will consist of |x| followed by |y| followed
//! by |z|, with kern nodes for the spaces between and around them.
//!
//! @<Attach the limits to |y| and adjust |height(v)|, |depth(v)|...@>=
//! if math_type(supscr(q))=empty then
//!   begin free_node(x,box_node_size); list_ptr(v):=y;
//!   end
//! else  begin shift_up:=big_op_spacing3-depth(x);
//!   if shift_up<big_op_spacing1 then shift_up:=big_op_spacing1;
//!   p:=new_kern(shift_up); link(p):=y; link(x):=p;@/
//!   p:=new_kern(big_op_spacing5); link(p):=x; list_ptr(v):=p;
//!   height(v):=height(v)+big_op_spacing5+height(x)+depth(x)+shift_up;
//!   end;
//! if math_type(subscr(q))=empty then free_node(z,box_node_size)
//! else  begin shift_down:=big_op_spacing4-height(z);
//!   if shift_down<big_op_spacing2 then shift_down:=big_op_spacing2;
//!   p:=new_kern(shift_down); link(y):=p; link(p):=z;@/
//!   p:=new_kern(big_op_spacing5); link(z):=p;
//!   depth(v):=depth(v)+big_op_spacing5+height(z)+depth(z)+shift_down;
//!   end
//!
