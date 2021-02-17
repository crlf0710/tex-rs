//! @ Insertion and adjustment and mark nodes are constructed by the following
//! pieces of the program.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(insert),hmode+vadjust,mmode+vadjust: begin_insert_or_adjust;
//! any_mode(mark): make_mark;
//!
//! @ @<Forbidden...@>=
//! vmode+vadjust,
//!
//! @ @<Declare act...@>=
//! procedure begin_insert_or_adjust;
//! begin if cur_cmd=vadjust then cur_val:=255
//! else  begin scan_eight_bit_int;
//!   if cur_val=255 then
//!     begin print_err("You can't "); print_esc("insert"); print_int(255);
//! @.You can't \\insert255@>
//!     help1("I'm changing to \insert0; box 255 is special.");
//!     error; cur_val:=0;
//!     end;
//!   end;
//! saved(0):=cur_val; incr(save_ptr);
//! new_save_level(insert_group); scan_left_brace; normal_paragraph;
//! push_nest; mode:=-vmode; prev_depth:=ignore_depth;
//! end;
//!
