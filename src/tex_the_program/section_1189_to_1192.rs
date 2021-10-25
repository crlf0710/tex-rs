//! @ @<Cases of |print_cmd_chr|...@>=
//! left_right: if chr_code=left_noad then print_esc("left")
//! else print_esc("right");
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+left_right: math_left_right;
//!
//! @ @<Declare act...@>=
//! procedure math_left_right;
//! var t:small_number; {|left_noad| or |right_noad|}
//! @!p:pointer; {new noad}
//! begin t:=cur_chr;
//! if (t=right_noad)and(cur_group<>math_left_group) then
//!   @<Try to recover from mismatched \.{\\right}@>
//! else  begin p:=new_noad; type(p):=t;
//!   scan_delimiter(delimiter(p),false);
//!   if t=left_noad then
//!     begin push_math(math_left_group); link(head):=p; tail:=p;
//!     end
//!   else  begin p:=fin_mlist(p); unsave; {end of |math_left_group|}
//!     tail_append(new_noad); type(tail):=inner_noad;
//!     math_type(nucleus(tail)):=sub_mlist;
//!     info(nucleus(tail)):=p;
//!     end;
//!   end;
//! end;
//!
//! @ @<Try to recover from mismatch...@>=
//! begin if cur_group=math_shift_group then
//!   begin scan_delimiter(garbage,false);
//!   print_err("Extra "); print_esc("right");
//! @.Extra \\right.@>
//!   help1("I'm ignoring a \right that had no matching \left.");
//!   error;
//!   end
//! else off_save;
//! end
//!
