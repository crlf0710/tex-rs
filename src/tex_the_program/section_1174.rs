//! @ @<Declare act...@>=
//! @t\4@>@<Declare the function called |fin_mlist|@>@t@>@;@/
//! procedure build_choices;
//! label exit;
//! var p:pointer; {the current mlist}
//! begin unsave; p:=fin_mlist(null);
//! case saved(-1) of
//! 0:display_mlist(tail):=p;
//! 1:text_mlist(tail):=p;
//! 2:script_mlist(tail):=p;
//! 3:begin script_script_mlist(tail):=p; decr(save_ptr); return;
//!   end;
//! end; {there are no other cases}
//! incr(saved(-1)); push_math(math_choice_group); scan_left_brace;
//! exit:end;
//!
