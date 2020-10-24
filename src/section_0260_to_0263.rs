//! @ @<Insert a new control...@>=
//! begin if text(p)>0 then
//!   begin repeat if hash_is_full then overflow("hash size",hash_size);
//! @:TeX capacity exceeded hash size}{\quad hash size@>
//!   decr(hash_used);
//!   until text(hash_used)=0; {search for an empty location in |hash|}
//!   next(p):=hash_used; p:=hash_used;
//!   end;
//! str_room(l); d:=cur_length;
//! while pool_ptr>str_start[str_ptr] do
//!   begin decr(pool_ptr); str_pool[pool_ptr+l]:=str_pool[pool_ptr];
//!   end; {move current string up to make room for another}
//! for k:=j to j+l-1 do append_char(buffer[k]);
//! text(p):=make_string; pool_ptr:=pool_ptr+d;
//! @!stat incr(cs_count);@+tats@;@/
//! end
//!
//! @ The value of |hash_prime| should be roughly 85\pct! of |hash_size|, and it
//! should be a prime number.  The theory of hashing tells us to expect fewer
//! than two table probes, on the average, when the search is successful.
//! [See J.~S. Vitter, {\sl Journal of the ACM\/ \bf30} (1983), 231--258.]
//! @^Vitter, Jeffrey Scott@>
//!
//! @<Compute the hash code |h|@>=
//! h:=buffer[j];
//! for k:=j+1 to j+l-1 do
//!   begin h:=h+h+buffer[k];
//!   while h>=hash_prime do h:=h-hash_prime;
//!   end
//!
//! @ Single-character control sequences do not need to be looked up in a hash
//! table, since we can use the character code itself as a direct address.
//! The procedure |print_cs| prints the name of a control sequence, given
//! a pointer to its address in |eqtb|. A space is printed after the name
//! unless it is a single nonletter or an active character. This procedure
//! might be invoked with invalid data, so it is ``extra robust.'' The
//! individual characters must be printed one at a time using |print|, since
//! they may be unprintable.
//!
//! @<Basic printing...@>=
//! procedure print_cs(@!p:integer); {prints a purported control sequence}
//! begin if p<hash_base then {single character}
//!   if p>=single_base then
//!     if p=null_cs then
//!       begin print_esc("csname"); print_esc("endcsname"); print_char(" ");
//!       end
//!     else  begin print_esc(p-single_base);
//!       if cat_code(p-single_base)=letter then print_char(" ");
//!       end
//!   else if p<active_base then print_esc("IMPOSSIBLE.")
//! @.IMPOSSIBLE@>
//!   else print(p-active_base)
//! else if p>=undefined_control_sequence then print_esc("IMPOSSIBLE.")
//! else if (text(p)<0)or(text(p)>=str_ptr) then print_esc("NONEXISTENT.")
//! @.NONEXISTENT@>
//! else  begin print_esc(text(p));
//!   print_char(" ");
//!   end;
//! end;
//!
//! @ Here is a similar procedure; it avoids the error checks, and it never
//! prints a space after the control sequence.
//!
//! @<Basic printing procedures@>=
//! procedure sprint_cs(@!p:pointer); {prints a control sequence}
//! begin if p<hash_base then
//!   if p<single_base then print(p-active_base)
//!   else  if p<null_cs then print_esc(p-single_base)
//!     else  begin print_esc("csname"); print_esc("endcsname");
//!       end
//! else print_esc(text(p));
//! end;
//!
