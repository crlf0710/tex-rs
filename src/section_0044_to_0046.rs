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
