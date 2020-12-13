//! @ We also need to divide scaled dimensions by integers.
//!
//! @p function x_over_n(@!x:scaled;@!n:integer):scaled;
//! var negative:boolean; {should |remainder| be negated?}
//! begin negative:=false;
//! if n=0 then
//!   begin arith_error:=true; x_over_n:=0; remainder:=x;
//!   end
//! else  begin if n<0 then
//!     begin negate(x); negate(n); negative:=true;
//!     end;
//!   if x>=0 then
//!     begin x_over_n:=x div n; remainder:=x mod n;
//!     end
//!   else  begin x_over_n:=-((-x) div n); remainder:=-((-x) mod n);
//!     end;
//!   end;
//! if negative then negate(remainder);
//! end;
//!
