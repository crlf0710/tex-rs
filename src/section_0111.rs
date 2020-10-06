//! @ Here are the inequalities that the quarterword and halfword values
//! must satisfy (or rather, the inequalities that they mustn't satisfy):
//!
//! @<Check the ``constant''...@>=
//! init if (mem_min<>mem_bot)or(mem_max<>mem_top) then bad:=10;@+tini@;@/
//! if (mem_min>mem_bot)or(mem_max<mem_top) then bad:=10;
//! if (min_quarterword>0)or(max_quarterword<127) then bad:=11;
//! if (min_halfword>0)or(max_halfword<32767) then bad:=12;
//! if (min_quarterword<min_halfword)or@|
//!   (max_quarterword>max_halfword) then bad:=13;
//! if (mem_min<min_halfword)or(mem_max>=max_halfword)or@|
//!   (mem_bot-mem_min>max_halfword+1) then bad:=14;
//! if (font_base<min_quarterword)or(font_max>max_quarterword) then bad:=15;
//! if font_max>font_base+256 then bad:=16;
//! if (save_size>max_halfword)or(max_strings>max_halfword) then bad:=17;
//! if buf_size>max_halfword then bad:=18;
//! if max_quarterword-min_quarterword<255 then bad:=19;
//!
