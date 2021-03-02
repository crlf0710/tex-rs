//! @ @<Undump regions 1 to 6 of |eqtb|@>=
//! k:=active_base;
//! repeat undump_int(x);
//! if (x<1)or(k+x>eqtb_size+1) then goto bad_fmt;
//! for j:=k to k+x-1 do undump_wd(eqtb[j]);
//! k:=k+x;
//! undump_int(x);
//! if (x<0)or(k+x>eqtb_size+1) then goto bad_fmt;
//! for j:=k to k+x-1 do eqtb[j]:=eqtb[k-1];
//! k:=k+x;
//! until k>eqtb_size
//!
