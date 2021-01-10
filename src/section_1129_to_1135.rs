//! @ The help messages here contain a little white lie, since \.{\\noalign}
//! and \.{\\omit} are allowed also after `\.{\\noalign\{...\}}'.
//!
//! @<Declare act...@>=
//! procedure no_align_error;
//! begin print_err("Misplaced "); print_esc("noalign");
//! @.Misplaced \\noalign@>
//! help2("I expect to see \noalign only after the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//! procedure omit_error;
//! begin print_err("Misplaced "); print_esc("omit");
//! @.Misplaced \\omit@>
//! help2("I expect to see \omit only after tab marks or the \cr of")@/
//!   ("an alignment. Proceed, and I'll ignore this case."); error;
//! end;
//!
//! @ We've now covered most of the abuses of \.{\\halign} and \.{\\valign}.
//! Let's take a look at what happens when they are used correctly.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+halign,hmode+valign:init_align;
//! mmode+halign: if privileged then
//!   if cur_group=math_shift_group then init_align
//!   else off_save;
//! vmode+endv,hmode+endv: do_endv;
//!
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
//! @ @<Cases of |handle_right_brace|...@>=
//! align_group: begin back_input; cur_tok:=cs_token_flag+frozen_cr;
//!   print_err("Missing "); print_esc("cr"); print(" inserted");
//! @.Missing \\cr inserted@>
//!   help1("I'm guessing that you meant to end an alignment here.");
//!   ins_error;
//!   end;
//!
//! @ @<Cases of |handle_right_brace|...@>=
//! no_align_group: begin end_graf; unsave; align_peek;
//!   end;
//!
//! @ Finally, \.{\\endcsname} is not supposed to get through to |main_control|.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(end_cs_name): cs_error;
//!
//! @ @<Declare act...@>=
//! procedure cs_error;
//! begin print_err("Extra "); print_esc("endcsname");
//! @.Extra \\endcsname@>
//! help1("I'm ignoring this, since I wasn't doing a \csname.");
//! error;
//! end;
//!
