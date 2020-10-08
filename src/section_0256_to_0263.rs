//! @* \[18] The hash table.
//! Control sequences are stored and retrieved by means of a fairly standard hash
//! table algorithm called the method of ``coalescing lists'' (cf.\ Algorithm 6.4C
//! in {\sl The Art of Computer Programming\/}). Once a control sequence enters the
//! table, it is never removed, because there are complicated situations
//! involving \.{\\gdef} where the removal of a control sequence at the end of
//! a group would be a mistake preventable only by the introduction of a
//! complicated reference-count mechanism.
//!
//! The actual sequence of letters forming a control sequence identifier is
//! stored in the |str_pool| array together with all the other strings. An
//! auxiliary array |hash| consists of items with two halfword fields per
//! word. The first of these, called |next(p)|, points to the next identifier
//! belonging to the same coalesced list as the identifier corresponding to~|p|;
//! and the other, called |text(p)|, points to the |str_start| entry for
//! |p|'s identifier. If position~|p| of the hash table is empty, we have
//! |text(p)=0|; if position |p| is either empty or the end of a coalesced
//! hash list, we have |next(p)=0|. An auxiliary pointer variable called
//! |hash_used| is maintained in such a way that all locations |p>=hash_used|
//! are nonempty. The global variable |cs_count| tells how many multiletter
//! control sequences have been defined, if statistics are being kept.
//!
//! A global boolean variable called |no_new_control_sequence| is set to
//! |true| during the time that new hash table entries are forbidden.
//!
//! @d next(#) == hash[#].lh {link for coalesced lists}
//! @d text(#) == hash[#].rh {string number for control sequence name}
//! @d hash_is_full == (hash_used=hash_base) {test if all positions are occupied}
//! @d font_id_text(#) == text(font_id_base+#) {a frozen font identifier's name}
//!
//! @<Glob...@>=
//! @!hash: array[hash_base..undefined_control_sequence-1] of two_halves;
//!   {the hash table}
//! @!hash_used:pointer; {allocation pointer for |hash|}
//! @!no_new_control_sequence:boolean; {are new identifiers legal?}
//! @!cs_count:integer; {total number of known identifiers}
//!
//! @ @<Set init...@>=
//! no_new_control_sequence:=true; {new identifiers are usually forbidden}
//! next(hash_base):=0; text(hash_base):=0;
//! for k:=hash_base+1 to undefined_control_sequence-1 do hash[k]:=hash[hash_base];
//!
//! @ @<Initialize table entries...@>=
//! hash_used:=frozen_control_sequence; {nothing is used}
//! cs_count:=0;
//! eq_type(frozen_dont_expand):=dont_expand;
//! text(frozen_dont_expand):="notexpanded:";
//! @.notexpanded:@>
//!
//! @ Here is the subroutine that searches the hash table for an identifier
//! that matches a given string of length |l>1| appearing in |buffer[j..
//! (j+l-1)]|. If the identifier is found, the corresponding hash table address
//! is returned. Otherwise, if the global variable |no_new_control_sequence|
//! is |true|, the dummy address |undefined_control_sequence| is returned.
//! Otherwise the identifier is inserted into the hash table and its location
//! is returned.
//!
//! @p function id_lookup(@!j,@!l:integer):pointer; {search the hash table}
//! label found; {go here if you found it}
//! var h:integer; {hash code}
//! @!d:integer; {number of characters in incomplete current string}
//! @!p:pointer; {index in |hash| array}
//! @!k:pointer; {index in |buffer| array}
//! begin @<Compute the hash code |h|@>;
//! p:=h+hash_base; {we start searching here; note that |0<=h<hash_prime|}
//! loop@+begin if text(p)>0 then if length(text(p))=l then
//!     if str_eq_buf(text(p),j) then goto found;
//!   if next(p)=0 then
//!     begin if no_new_control_sequence then
//!       p:=undefined_control_sequence
//!     else @<Insert a new control sequence after |p|, then make
//!       |p| point to it@>;
//!     goto found;
//!     end;
//!   p:=next(p);
//!   end;
//! found: id_lookup:=p;
//! end;
//!
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
