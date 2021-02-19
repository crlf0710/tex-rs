//! @ We let |j| be the index of the character being stored when a ligature node
//! is being expanded, since we do not want to advance |hn| until we are sure
//! that the entire ligature consists of letters. Note that it is possible
//! to get to |done3| with |hn=0| and |hb| not set to any value.
//!
//! @<Move the characters of a ligature node to |hu| and |hc|...@>=
//! begin if font(lig_char(s))<>hf then goto done3;
//! j:=hn; q:=lig_ptr(s);@+if q>null then hyf_bchar:=character(q);
//! while q>null do
//!   begin c:=qo(character(q));
//!   if lc_code(c)=0 then goto done3;
//!   if j=63 then goto done3;
//!   incr(j); hu[j]:=c; hc[j]:=lc_code(c);@/
//!   q:=link(q);
//!   end;
//! hb:=s; hn:=j;
//! if odd(subtype(s)) then hyf_bchar:=font_bchar[hf]@+else hyf_bchar:=non_char;
//! end
//!
