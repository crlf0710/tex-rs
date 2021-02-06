//! @ @<Pack the family into |trie| relative to |h|@>=
//! trie_taken[h]:=true; trie_ref[p]:=h; q:=p;
//! repeat z:=h+so(trie_c[q]); l:=trie_back(z); r:=trie_link(z);
//! trie_back(r):=l; trie_link(l):=r; trie_link(z):=0;
//! if l<256 then
//!   begin if z<256 then ll:=z @+else ll:=256;
//!   repeat trie_min[l]:=r; incr(l);
//!   until l=ll;
//!   end;
//! q:=trie_r[q];
//! until q=0
//!
