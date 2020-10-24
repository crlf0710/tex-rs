//! @ Whenever we reach the following piece of code, we will have
//! |cur_chr=buffer[k-1]| and |k<=limit+1| and |cat=cat_code(cur_chr)|. If an
//! expanded code like \.{\^\^A} or \.{\^\^df} appears in |buffer[(k-1)..(k+1)]|
//! or |buffer[(k-1)..(k+2)]|, we
//! will store the corresponding code in |buffer[k-1]| and shift the rest of
//! the buffer left two or three places.
//!
//! @<If an expanded...@>=
//! begin if buffer[k]=cur_chr then @+if cat=sup_mark then @+if k<limit then
//!   begin c:=buffer[k+1]; @+if c<@'200 then {yes, one is indeed present}
//!     begin d:=2;
//!     if is_hex(c) then @+if k+2<=limit then
//!       begin cc:=buffer[k+2]; @+if is_hex(cc) then incr(d);
//!       end;
//!     if d>2 then
//!       begin hex_to_cur_chr; buffer[k-1]:=cur_chr;
//!       end
//!     else if c<@'100 then buffer[k-1]:=c+@'100
//!     else buffer[k-1]:=c-@'100;
//!     limit:=limit-d; first:=first-d;
//!     while k<=limit do
//!       begin buffer[k]:=buffer[k+d]; incr(k);
//!       end;
//!     goto start_cs;
//!     end;
//!   end;
//! end
//!
//! @ @<Scan ahead in the buffer...@>=
//! begin repeat cur_chr:=buffer[k]; cat:=cat_code(cur_chr); incr(k);
//! until (cat<>letter)or(k>limit);
//! @<If an expanded...@>;
//! if cat<>letter then decr(k);
//!   {now |k| points to first nonletter}
//! if k>loc+1 then {multiletter control sequence has been scanned}
//!   begin cur_cs:=id_lookup(loc,k-loc); loc:=k; goto found;
//!   end;
//! end
//!