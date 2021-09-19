//! @ The kern nodes appended here must be distinguished from other kerns, lest
//! they be wiped away by the hyphenation algorithm or by a previous line break.
//!
//! The two kerns are computed with (machine-dependent) |real| arithmetic, but
//! their sum is machine-independent; the net effect is machine-independent,
//! because the user cannot remove these nodes nor access them via \.{\\lastkern}.
//!
//! @<Append the accent with appropriate kerns...@>=
//! begin t:=slant(f)/float_constant(65536);
//! @^real division@>
//! i:=char_info(f)(character(q));
//! w:=char_width(f)(i); h:=char_height(f)(height_depth(i));
//! if h<>x then {the accent must be shifted up or down}
//!   begin p:=hpack(p,natural); shift_amount(p):=x-h;
//!   end;
//! delta:=round((w-a)/float_constant(2)+h*t-x*s);
//! @^real multiplication@>
//! @^real addition@>
//! r:=new_kern(delta); subtype(r):=acc_kern; link(tail):=r; link(r):=p;
//! tail:=new_kern(-a-delta); subtype(tail):=acc_kern; link(p):=tail; p:=q;
//! end
//!
