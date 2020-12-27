//! @ Here is a procedure that might be called `Get the next non-blank non-relax
//! non-call non-assignment token'.
//!
//! @<Declare act...@>=
//! procedure do_assignments;
//! label exit;
//! begin loop begin @<Get the next non-blank non-relax...@>;
//!   if cur_cmd<=max_non_prefixed_command then return;
//!   set_box_allowed:=false; prefixed_command; set_box_allowed:=true;
//!   end;
//! exit:end;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_group):begin get_token; save_for_after(cur_tok);
//!   end;
//!
//! @ Files for \.{\\read} are opened and closed by the |in_stream| command.
//!
//! @<Put each...@>=
//! primitive("openin",in_stream,1);
//! @!@:open_in_}{\.{\\openin} primitive@>
//! primitive("closein",in_stream,0);
//! @!@:close_in_}{\.{\\closein} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! in_stream: if chr_code=0 then print_esc("closein")
//!   else print_esc("openin");
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(in_stream): open_or_close_in;
//!
//! @ @<Declare act...@>=
//! procedure open_or_close_in;
//! var c:0..1; {1 for \.{\\openin}, 0 for \.{\\closein}}
//! @!n:0..15; {stream number}
//! begin c:=cur_chr; scan_four_bit_int; n:=cur_val;
//! if read_open[n]<>closed then
//!   begin a_close(read_file[n]); read_open[n]:=closed;
//!   end;
//! if c<>0 then
//!   begin scan_optional_equals; scan_file_name;
//!   if cur_ext="" then cur_ext:=".tex";
//!   pack_cur_name;
//!   if a_open_in(read_file[n]) then read_open[n]:=just_open;
//!   end;
//! end;
//!
