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
