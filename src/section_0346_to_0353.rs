//! @ We go to |restart| instead of to |switch|, because |state| might equal
//! |token_list| after the error has been dealt with
//! (cf.\ |clear_for_error_prompt|).
//!
//! @<Decry the invalid...@>=
//! begin print_err("Text line contains an invalid character");
//! @.Text line contains...@>
//! help2("A funny symbol that I can't read has just been input.")@/
//! ("Continue, and I'll forget that it ever happened.");@/
//! deletions_allowed:=false; error; deletions_allowed:=true;
//! goto restart;
//! end
//!
//! @ @d add_delims_to(#)==#+math_shift,#+tab_mark,#+mac_param,
//!   #+sub_mark,#+letter,#+other_char
//!
//! @<Handle situations involving spaces, braces, changes of state@>=
//! mid_line+spacer:@<Enter |skip_blanks| state, emit a space@>;
//! mid_line+car_ret:@<Finish line, emit a space@>;
//! skip_blanks+car_ret,any_state_plus(comment):
//!   @<Finish line, |goto switch|@>;
//! new_line+car_ret:@<Finish line, emit a \.{\\par}@>;
//! mid_line+left_brace: incr(align_state);
//! skip_blanks+left_brace,new_line+left_brace: begin
//!   state:=mid_line; incr(align_state);
//!   end;
//! mid_line+right_brace: decr(align_state);
//! skip_blanks+right_brace,new_line+right_brace: begin
//!   state:=mid_line; decr(align_state);
//!   end;
//! add_delims_to(skip_blanks),add_delims_to(new_line): state:=mid_line;
//!
//! @ When a character of type |spacer| gets through, its character code is
//! changed to $\.{"\ "}=@'40$. This means that the ASCII codes for tab and space,
//! and for the space inserted at the end of a line, will
//! be treated alike when macro parameters are being matched. We do this
//! since such characters are indistinguishable on most computer terminal displays.
//!
//! @<Finish line, emit a space@>=
//! begin loc:=limit+1; cur_cmd:=spacer; cur_chr:=" ";
//! end
//!
//! @ The following code is performed only when |cur_cmd=spacer|.
//!
//! @<Enter |skip_blanks| state, emit a space@>=
//! begin state:=skip_blanks; cur_chr:=" ";
//! end
//!
//! @ @<Finish line, |goto switch|@>=
//! begin loc:=limit+1; goto switch;
//! end
//!
//! @ @<Finish line, emit a \.{\\par}@>=
//! begin loc:=limit+1; cur_cs:=par_loc; cur_cmd:=eq_type(cur_cs);
//! cur_chr:=equiv(cur_cs);
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Notice that a code like \.{\^\^8} becomes \.x if not followed by a hex digit.
//!
//! @d is_hex(#)==(((#>="0")and(#<="9"))or((#>="a")and(#<="f")))
//! @d hex_to_cur_chr==
//!   if c<="9" then cur_chr:=c-"0" @+else cur_chr:=c-"a"+10;
//!   if cc<="9" then cur_chr:=16*cur_chr+cc-"0"
//!   else cur_chr:=16*cur_chr+cc-"a"+10
//!
//! @<If this |sup_mark| starts an expanded character...@>=
//! begin if cur_chr=buffer[loc] then if loc<limit then
//!   begin c:=buffer[loc+1]; @+if c<@'200 then {yes we have an expanded char}
//!     begin loc:=loc+2;
//!     if is_hex(c) then if loc<=limit then
//!       begin cc:=buffer[loc]; @+if is_hex(cc) then
//!         begin incr(loc); hex_to_cur_chr; goto reswitch;
//!         end;
//!       end;
//!     if c<@'100 then cur_chr:=c+@'100 @+else cur_chr:=c-@'100;
//!     goto reswitch;
//!     end;
//!   end;
//! state:=mid_line;
//! end
//!
//! @ @<Process an active-character...@>=
//! begin cur_cs:=cur_chr+active_base;
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs); state:=mid_line;
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
