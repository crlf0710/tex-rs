//! @ An |align_group| code is supposed to remain on the |save_stack|
//! during an entire alignment, until |fin_align| removes it.
//!
//! A devious user might force an |endv| command to occur just about anywhere;
//! we must defeat such hacks.
//!
//! @<Declare act...@>=
//! procedure do_endv;
//! begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
//! while (input_stack[base_ptr].index_field<>v_template) and
//!       (input_stack[base_ptr].loc_field=null) and
//!       (input_stack[base_ptr].state_field=token_list) do decr(base_ptr);
//! if (input_stack[base_ptr].index_field<>v_template) or
//!       (input_stack[base_ptr].loc_field<>null) or
//!       (input_stack[base_ptr].state_field<>token_list) then
//!   fatal_error("(interwoven alignment preambles are not allowed)");
//! @.interwoven alignment preambles...@>
//!  if cur_group=align_group then
//!   begin end_graf;
//!   if fin_col then fin_row;
//!   end
//! else off_save;
//! end;
//!
