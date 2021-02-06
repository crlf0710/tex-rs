//! @ @<If all characters of the family fit relative to |h|...@>=
//! q:=trie_r[p];
//! while q>0 do
//!   begin if trie_link(h+so(trie_c[q]))=0 then goto not_found;
//!   q:=trie_r[q];
//!   end;
//! goto found
//!
