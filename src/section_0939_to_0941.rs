//! @ @<Enter a hyphenation exception@>=
//! begin incr(n); hc[n]:=cur_lang; str_room(n); h:=0;
//! for j:=1 to n do
//!   begin h:=(h+h+hc[j]) mod hyph_size;
//!   append_char(hc[j]);
//!   end;
//! s:=make_string;
//! @<Insert the \(p)pair |(s,p)| into the exception table@>;
//! end
//!
//! @ @<Insert the \(p)pair |(s,p)|...@>=
//! if hyph_count=hyph_size then overflow("exception dictionary",hyph_size);
//! @:TeX capacity exceeded exception dictionary}{\quad exception dictionary@>
//! incr(hyph_count);
//! while hyph_word[h]<>0 do
//!   begin @<If the string |hyph_word[h]| is less than \(or)or equal to
//!   |s|, interchange |(hyph_word[h],hyph_list[h])| with |(s,p)|@>;
//!   if h>0 then decr(h)@+else h:=hyph_size;
//!   end;
//! hyph_word[h]:=s; hyph_list[h]:=p
//!
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
