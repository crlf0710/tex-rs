//! @ Subroutine |save_for_after| puts a token on the stack for save-keeping.
//
// @p procedure save_for_after(@!t:halfword);
// begin if cur_level>level_one then
//   begin check_full_save_stack;
//   save_type(save_ptr):=insert_token; save_level(save_ptr):=level_zero;
//   save_index(save_ptr):=t; incr(save_ptr);
//   end;
// end;
//
