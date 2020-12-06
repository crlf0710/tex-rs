//! @ The following function is used to create a scaled integer from a given decimal
//! fraction $(.d_0d_1\ldots d_{k-1})$, where |0<=k<=17|. The digit $d_i$ is
//! given in |dig[i]|, and the calculation produces a correctly rounded result.
//!
//! @p function round_decimals(@!k:small_number) : scaled;
//!   {converts a decimal fraction}
//! var a:integer; {the accumulator}
//! begin a:=0;
//! while k>0 do
//!   begin decr(k); a:=(a+dig[k]*two) div 10;
//!   end;
//! round_decimals:=(a+1) div 2;
//! end;
//!
