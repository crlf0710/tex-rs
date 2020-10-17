//! @ Several of the elementary string operations are performed using \.{WEB}
//! macros instead of \PASCAL\ procedures, because many of the
//! operations are done quite frequently and we want to avoid the
//! overhead of procedure calls. For example, here is
//! a simple macro that computes the length of a string.
//! @.WEB@>
//!
//! @d length(#)==(str_start[#+1]-str_start[#]) {the number of characters
//!   in string number \#}
//!
//! @ The length of the current string is called |cur_length|:
//!
//! @d cur_length == (pool_ptr - str_start[str_ptr])
//!
//! @ Strings are created by appending character codes to |str_pool|.
//! The |append_char| macro, defined here, does not check to see if the
//! value of |pool_ptr| has gotten too high; this test is supposed to be
//! made before |append_char| is used. There is also a |flush_char|
//! macro, which erases the last character appended.
//!
//! To test if there is room to append |l| more characters to |str_pool|,
//! we shall write |str_room(l)|, which aborts \TeX\ and gives an
//! apologetic error message if there isn't enough room.
//!
//! @d append_char(#) == {put |ASCII_code| \# at the end of |str_pool|}
//! begin str_pool[pool_ptr]:=si(#); incr(pool_ptr);
//! end
//! @d flush_char == decr(pool_ptr) {forget the last character in the pool}
//! @d str_room(#) == {make sure that the pool hasn't overflowed}
//!   begin if pool_ptr+# > pool_size then
//!   overflow("pool size",pool_size-init_pool_ptr);
//! @:TeX capacity exceeded pool size}{\quad pool size@>
//!   end
//!
//! @ Once a sequence of characters has been appended to |str_pool|, it
//! officially becomes a string when the function |make_string| is called.
//! This function returns the identification number of the new string as its
//! value.
//!
//! @p function make_string : str_number; {current string enters the pool}
//! begin if str_ptr=max_strings then
//!   overflow("number of strings",max_strings-init_str_ptr);
//! @:TeX capacity exceeded number of strings}{\quad number of strings@>
//! incr(str_ptr); str_start[str_ptr]:=pool_ptr;
//! make_string:=str_ptr-1;
//! end;
//!
//! @ To destroy the most recently made string, we say |flush_string|.
//!
//! @d flush_string==begin decr(str_ptr); pool_ptr:=str_start[str_ptr];
//!   end
//!
//! @ The following subroutine compares string |s| with another string of the
//! same length that appears in |buffer| starting at position |k|;
//! the result is |true| if and only if the strings are equal.
//! Empirical tests indicate that |str_eq_buf| is used in such a way that
//! it tends to return |true| about 80 percent of the time.
//!
//! @p function str_eq_buf(@!s:str_number;@!k:integer):boolean;
//!   {test equality of strings}
//! label not_found; {loop exit}
//! var j: pool_pointer; {running index}
//! @!result: boolean; {result of comparison}
//! begin j:=str_start[s];
//! while j<str_start[s+1] do
//!   begin if so(str_pool[j])<>buffer[k] then
//!     begin result:=false; goto not_found;
//!     end;
//!   incr(j); incr(k);
//!   end;
//! result:=true;
//! not_found: str_eq_buf:=result;
//! end;
//!
//! @ Here is a similar routine, but it compares two strings in the string pool,
//! and it does not assume that they have the same length.
//!
//! @p function str_eq_str(@!s,@!t:str_number):boolean;
//!   {test equality of strings}
//! label not_found; {loop exit}
//! var j,@!k: pool_pointer; {running indices}
//! @!result: boolean; {result of comparison}
//! begin result:=false;
//! if length(s)<>length(t) then goto not_found;
//! j:=str_start[s]; k:=str_start[t];
//! while j<str_start[s+1] do
//!   begin if str_pool[j]<>str_pool[k] then goto not_found;
//!   incr(j); incr(k);
//!   end;
//! result:=true;
//! not_found: str_eq_str:=result;
//! end;
//!
