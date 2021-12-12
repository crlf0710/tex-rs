//! @ When there is a subscript without a superscript, the top of the subscript
//! should not exceed the baseline plus four-fifths of the x-height.
//!
//! @<Construct a subscript box |x| when there is no superscript@>=
//! begin x:=clean_box(subscr(q),sub_style(cur_style));
//! width(x):=width(x)+script_space;
//! if shift_down<sub1(cur_size) then shift_down:=sub1(cur_size);
//! clr:=height(x)-(abs(math_x_height(cur_size)*4) div 5);
//! if shift_down<clr then shift_down:=clr;
//! shift_amount(x):=shift_down;
//! end
//!
