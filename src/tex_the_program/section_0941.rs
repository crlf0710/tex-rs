//! @ @<If the string |hyph_word[h]| is less than \(or)...@>=
//! k:=hyph_word[h];
//! if length(k)<length(s) then goto found;
//! if length(k)>length(s) then goto not_found;
//! u:=str_start[k]; v:=str_start[s];
//! repeat if str_pool[u]<str_pool[v] then goto found;
//! if str_pool[u]>str_pool[v] then goto not_found;
//! incr(u); incr(v);
//! until u=str_start[k+1];
//! found:q:=hyph_list[h]; hyph_list[h]:=p; p:=q;@/
//! t:=hyph_word[h]; hyph_word[h]:=s; s:=t;
//! not_found:
//!
