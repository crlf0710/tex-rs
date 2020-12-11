//! @ We don't implement \.{\\write} inside of leaders. (The reason is that
//! the number of times a leader box appears might be different in different
//! implementations, due to machine-dependent rounding in the glue calculations.)
//! @^leaders@>
//!
//! @<Do some work that has been queued up...@>=
//! if not doing_leaders then
//!   begin j:=write_stream(p);
//!   if subtype(p)=write_node then write_out(p)
//!   else  begin if write_open[j] then a_close(write_file[j]);
//!     if subtype(p)=close_node then write_open[j]:=false
//!     else if j<16 then
//!       begin cur_name:=open_name(p); cur_area:=open_area(p);
//!       cur_ext:=open_ext(p);
//!       if cur_ext="" then cur_ext:=".tex";
//!       pack_cur_name;
//!       while not a_open_out(write_file[j]) do
//!         prompt_file_name("output file name",".tex");
//!       write_open[j]:=true;
//!       end;
//!     end;
//!   end
//!
