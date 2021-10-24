//! @ @<Make the running dimensions in rule |q| extend...@>=
//! begin if is_running(width(q)) then width(q):=width(p);
//! if is_running(height(q)) then height(q):=height(p);
//! if is_running(depth(q)) then depth(q):=depth(p);
//! if o<>0 then
//!   begin r:=link(q); link(q):=null; q:=hpack(q,natural);
//!   shift_amount(q):=o; link(q):=r; link(s):=q;
//!   end;
//! end
//!
