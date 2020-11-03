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
