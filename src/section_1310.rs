//! @ @d undump_four_ASCII==
//!   undump_qqqq(w);
//!   str_pool[k]:=si(qo(w.b0)); str_pool[k+1]:=si(qo(w.b1));
//!   str_pool[k+2]:=si(qo(w.b2)); str_pool[k+3]:=si(qo(w.b3))
//!
//! @<Undump the string pool@>=
//! undump_size(0)(pool_size)('string pool size')(pool_ptr);
//! undump_size(0)(max_strings)('max strings')(str_ptr);
//! for k:=0 to str_ptr do undump(0)(pool_ptr)(str_start[k]);
//! k:=0;
//! while k+4<pool_ptr do
//!   begin undump_four_ASCII; k:=k+4;
//!   end;
//! k:=pool_ptr-4; undump_four_ASCII;
//! init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr
//!
