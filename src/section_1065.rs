//! @ At this point, |link(temp_head)=p|, a pointer to an empty one-word node.
//!
//! @<Prepare to insert a token that matches |cur_group|...@>=
//! case cur_group of
//! semi_simple_group: begin info(p):=cs_token_flag+frozen_end_group;
//!   print_esc("endgroup");
//! @.Missing \\endgroup inserted@>
//!   end;
//! math_shift_group: begin info(p):=math_shift_token+"$"; print_char("$");
//! @.Missing \$ inserted@>
//!   end;
//! math_left_group: begin info(p):=cs_token_flag+frozen_right; link(p):=get_avail;
//!   p:=link(p); info(p):=other_token+"."; print_esc("right.");
//! @.Missing \\right\hbox{.} inserted@>
//! @^null delimiter@>
//!   end;
//! othercases begin info(p):=right_brace_token+"}"; print_char("}");
//! @.Missing \} inserted@>
//!   end
//! endcases
//!
