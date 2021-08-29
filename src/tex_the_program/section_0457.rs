//! @ @<Adjust \(f)for the magnification ratio@>=
//! begin prepare_mag;
//! if mag<>1000 then
//!   begin cur_val:=xn_over_d(cur_val,1000,mag);
//!   f:=(1000*f+@'200000*remainder) div mag;
//!   cur_val:=cur_val+(f div @'200000); f:=f mod @'200000;
//!   end;
//! end
//!
