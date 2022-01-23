//! @ If the nucleus of an |op_noad| is a single character, it is to be
//! centered vertically with respect to the axis, after first being enlarged
//! (via a character list in the font) if we are in display style.  The normal
//! convention for placing displayed limits is to put them above and below the
//! operator in display style.
//!
//! The italic correction is removed from the character if there is a subscript
//! and the limits are not being displayed. The |make_op|
//! routine returns the value that should be used as an offset between
//! subscript and superscript.
//!
//! After |make_op| has acted, |subtype(q)| will be |limits| if and only if
//! the limits have been set above and below the operator. In that case,
//! |new_hlist(q)| will already contain the desired final box.
//!
//! @<Declare math...@>=
//! function make_op(@!q:pointer):scaled;
//! var delta:scaled; {offset between subscript and superscript}
//! @!p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
//! @!c:quarterword;@+@!i:four_quarters; {registers for character examination}
//! @!shift_up,@!shift_down:scaled; {dimensions for box calculation}
//! begin if (subtype(q)=normal)and(cur_style<text_style) then
//!   subtype(q):=limits;
//! if math_type(nucleus(q))=math_char then
//!   begin fetch(nucleus(q));
//!   if (cur_style<text_style)and(char_tag(cur_i)=list_tag) then {make it larger}
//!     begin c:=rem_byte(cur_i); i:=char_info(cur_f)(c);
//!     if char_exists(i) then
//!       begin cur_c:=c; cur_i:=i; character(nucleus(q)):=c;
//!       end;
//!     end;
//!   delta:=char_italic(cur_f)(cur_i); x:=clean_box(nucleus(q),cur_style);
//!   if (math_type(subscr(q))<>empty)and(subtype(q)<>limits) then
//!     width(x):=width(x)-delta; {remove italic correction}
//!   shift_amount(x):=half(height(x)-depth(x)) - axis_height(cur_size);
//!     {center vertically}
//!   math_type(nucleus(q)):=sub_box; info(nucleus(q)):=x;
//!   end
//! else delta:=0;
//! if subtype(q)=limits then
//!   @<Construct a box with limits above and below it, skewed by |delta|@>;
//! make_op:=delta;
//! end;
//!
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
