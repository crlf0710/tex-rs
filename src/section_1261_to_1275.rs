//! @ @<Cases of |print_cmd_chr|...@>=
//! set_font:begin print("select font "); slow_print(font_name[chr_code]);
//!   if font_size[chr_code]<>font_dsize[chr_code] then
//!     begin print(" at "); print_scaled(font_size[chr_code]);
//!     print("pt");
//!     end;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("batchmode",set_interaction,batch_mode);
//! @!@:batch_mode_}{\.{\\batchmode} primitive@>
//! primitive("nonstopmode",set_interaction,nonstop_mode);
//! @!@:nonstop_mode_}{\.{\\nonstopmode} primitive@>
//! primitive("scrollmode",set_interaction,scroll_mode);
//! @!@:scroll_mode_}{\.{\\scrollmode} primitive@>
//! primitive("errorstopmode",set_interaction,error_stop_mode);
//! @!@:error_stop_mode_}{\.{\\errorstopmode} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! set_interaction: case chr_code of
//!   batch_mode: print_esc("batchmode");
//!   nonstop_mode: print_esc("nonstopmode");
//!   scroll_mode: print_esc("scrollmode");
//!   othercases print_esc("errorstopmode")
//!   endcases;
//!
//! @ @<Assignments@>=
//! set_interaction: new_interaction;
//!
//! @ @<Declare subprocedures for |prefixed_command|@>=
//! procedure new_interaction;
//! begin print_ln;
//! interaction:=cur_chr;
//! @<Initialize the print |selector| based on |interaction|@>;
//! if log_opened then selector:=selector+2;
//! end;
//!
//! @ The \.{\\afterassignment} command puts a token into the global
//! variable |after_token|. This global variable is examined just after
//! every assignment has been performed.
//!
//! @<Glob...@>=
//! @!after_token:halfword; {zero, or a saved token}
//!
//! @ @<Set init...@>=
//! after_token:=0;
//!
//! @ @<Cases of |main_control| that don't...@>=
//! any_mode(after_assignment):begin get_token; after_token:=cur_tok;
//!   end;
//!
//! @ @<Insert a token saved by \.{\\afterassignment}, if any@>=
//! if after_token<>0 then
//!   begin cur_tok:=after_token; back_input; after_token:=0;
//!   end
//!
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
