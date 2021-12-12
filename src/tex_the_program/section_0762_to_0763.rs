//! @ The |make_left_right| function constructs a left or right delimiter of
//! the required size and returns the value |open_noad| or |close_noad|. The
//! |right_noad| and |left_noad| will both be based on the original |style|,
//! so they will have consistent sizes.
//!
//! We use the fact that |right_noad-left_noad=close_noad-open_noad|.
//!
//! @<Declare math...@>=
//! function make_left_right(@!q:pointer;@!style:small_number;
//!   @!max_d,@!max_h:scaled):small_number;
//! var delta,@!delta1,@!delta2:scaled; {dimensions used in the calculation}
//! begin if style<script_style then cur_size:=text_size
//! else cur_size:=16*((style-text_style) div 2);
//! delta2:=max_d+axis_height(cur_size);
//! delta1:=max_h+max_d-delta2;
//! if delta2>delta1 then delta1:=delta2; {|delta1| is max distance from axis}
//! delta:=(delta1 div 500)*delimiter_factor;
//! delta2:=delta1+delta1-delimiter_shortfall;
//! if delta<delta2 then delta:=delta2;
//! new_hlist(q):=var_delimiter(delimiter(q),cur_size,delta);
//! make_left_right:=type(q)-(left_noad-open_noad); {|open_noad| or |close_noad|}
//! end;
//!
//! @ @<Change the current style and |goto delete_q|@>=
//! begin cur_style:=subtype(q); s:=style_node_size;
//! @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
//! goto delete_q;
//! end
//!


